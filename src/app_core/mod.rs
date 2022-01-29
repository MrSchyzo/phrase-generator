use crate::app_core::types::upload::UploadedSpeech;
use std::collections::{HashMap, HashSet};
use std::hash::Hash;
use std::str::FromStr;
use std::sync::Arc;

use crate::outgoing::tts_wrapper::TtsWrapper;
use crate::served::types::graphql::Speech;
use async_recursion::async_recursion;
use async_trait::async_trait;
use itertools::Itertools;
use rand::RngCore;
use sqlx::postgres::PgPoolOptions;
use sqlx::FromRow;
use sqlx::{Pool, Postgres};

use self::errors::AppError;

pub mod engine;
pub mod errors;
pub mod types;
use crate::app_core::engine::types::parsing::TokenReference;
use crate::app_core::engine::types::{PlaceholderReference, ProductionBranch};
use crate::app_core::errors::GenerationError;

pub type AppResult<T> = Result<T, AppError>;

pub struct SpeechGenerationOptions {}

#[async_trait]
pub trait AsyncHealth {
    async fn is_healthy(&self) -> AppResult<()>;
}

#[async_trait]
pub trait AsyncUploader {
    async fn upload(
        &self,
        request: crate::app_core::types::upload::Speech,
    ) -> AppResult<UploadedSpeech>;
}

#[async_trait]
pub trait AsyncPhraseGenerator {
    async fn generate(&self, options: SpeechGenerationOptions) -> AppResult<Speech>;
}

#[async_trait]
pub trait AsyncHealthyUploader: AsyncUploader + AsyncHealth {}

type AppUploader = dyn AsyncHealthyUploader + Send + Sync;
type AppPhraseGenerator = dyn AsyncPhraseGenerator + Send + Sync;

pub struct AppCore {
    uploader: Arc<AppUploader>,
    generator: Arc<AppPhraseGenerator>,
}

impl AppCore {
    pub fn new(uploader: Arc<AppUploader>, generator: Arc<AppPhraseGenerator>) -> Self {
        Self {
            uploader,
            generator,
        }
    }

    pub fn uploader(&self) -> &AppUploader {
        self.uploader.as_ref()
    }

    pub fn generator(&self) -> &AppPhraseGenerator {
        self.generator.as_ref()
    }

    pub async fn is_healthy(&self) -> AppResult<()> {
        self.uploader().is_healthy().await
    }
}

#[derive(Clone)]
pub struct Uploader {
    wrapper: Arc<TtsWrapper>,
}

impl Uploader {
    pub fn new(wrapper: Arc<TtsWrapper>) -> Self {
        Self { wrapper }
    }
}

#[async_trait]
impl AsyncUploader for Uploader {
    async fn upload(
        &self,
        request: crate::app_core::types::upload::Speech,
    ) -> AppResult<UploadedSpeech> {
        self.wrapper.upload(request.into()).await.map(Into::into)
    }
}

#[async_trait]
impl AsyncHealth for Uploader {
    async fn is_healthy(&self) -> AppResult<()> {
        self.wrapper.health().await
    }
}

#[async_trait]
impl AsyncHealthyUploader for Uploader {}

pub struct PhraseGenerator;

#[async_trait]
impl AsyncPhraseGenerator for PhraseGenerator {
    async fn generate(&self, _: SpeechGenerationOptions) -> AppResult<Speech> {
        if rand::thread_rng().next_u32() % 100 > 50 {
            Ok(Speech {
                id: "1".to_owned(),
                text: "Ciao mondo".to_owned(),
            })
        } else {
            Ok(Speech {
                id: "3".to_owned(),
                text: "Ciaone mondone".to_owned(),
            })
        }
    }
}

trait GenerationState<Word: Sized + Hash + Send>: Send {
    fn increase_depth(&mut self);
    fn decrease_depth(&mut self);
    fn current_depth(&self) -> u16;
    fn is_too_deep(&self) -> bool;

    fn alter_length(&mut self, amount: i32);
    fn is_too_long(&self) -> bool;

    fn register_word(&mut self, word: Word);
    fn unregister_word(&mut self, word: &Word);
    fn has_used_word(&self, word: &Word) -> bool;
}

pub struct InMemoryGenerationState {
    depth: u16,
    max_depth: u16,

    length: i32,
    max_length: i32,

    used_words: HashSet<i32>,
}

impl InMemoryGenerationState {
    #[allow(unused)]
    pub fn new(max_depth: u16, max_length: i32) -> Self {
        Self {
            depth: 0u16,
            max_depth,
            length: 0i32,
            max_length,
            used_words: HashSet::new(),
        }
    }
}

impl GenerationState<i32> for InMemoryGenerationState {
    fn increase_depth(&mut self) {
        self.depth += 1
    }

    fn decrease_depth(&mut self) {
        self.depth -= 1
    }

    fn current_depth(&self) -> u16 {
        self.depth
    }

    fn is_too_deep(&self) -> bool {
        self.depth > self.max_depth
    }

    fn alter_length(&mut self, amount: i32) {
        self.length += amount
    }

    fn is_too_long(&self) -> bool {
        self.length > self.max_length
    }

    fn register_word(&mut self, word: i32) {
        self.used_words.insert(word);
    }

    fn unregister_word(&mut self, word: &i32) {
        self.used_words.remove(word);
    }

    fn has_used_word(&self, word: &i32) -> bool {
        self.used_words.contains(word)
    }
}

#[allow(unused)]
async fn generate_phrase(_: SpeechGenerationOptions) -> AppResult<Speech> {
    let mut _a = InMemoryGenerationState::new(100, 500);
    Ok(Speech {
        id: "".to_string(),
        text: "".to_string(),
    })
}

#[allow(unused)]
#[async_recursion]
async fn random_production_step(
    nts_name: &str,
    state: &mut dyn GenerationState<i32>,
) -> AppResult<String> {
    let pool = PgPoolOptions::new()
        .max_connections(8)
        .connect("postgres://postgres:password@localhost:49153/postgres")
        .await
        .map_err(AppError::for_generation_in_sql)?;

    let query_template = include_str!("../../draft_ideas/select_random_production.sql");
    let query = sqlx::query_as(query_template).bind(nts_name);

    let (row,): (String,) = query
        .fetch_one(&pool)
        .await
        .map_err(AppError::for_generation_in_sql)?;

    let branch = ProductionBranch::from_str(&row)?;

    let _placeholders = branch.ordered_placeholder_references()?;

    Ok("".to_owned())
}

#[allow(unused)]
#[async_recursion]
async fn generate_from_placeholder(
    placeholder: &PlaceholderReference,
    state: &mut dyn GenerationState<i32>,
    pool: &Pool<Postgres>,
) -> AppResult<String> {
    match placeholder {
        PlaceholderReference::NonTerminalSymbol(nts) => {
            generate_from_non_terminal_symbol(nts, state, pool).await
        }
        PlaceholderReference::WordSelector(word) => {
            generate_from_word_selector(word, state, pool).await
        }
    }
}

#[allow(unused)]
#[async_recursion]
async fn generate_from_non_terminal_symbol(
    token: &TokenReference,
    state: &mut dyn GenerationState<i32>,
    pool: &Pool<Postgres>,
) -> AppResult<String> {
    if state.is_too_deep() {
        return Err(GenerationError::ExcessiveDepth(state.current_depth()).into());
    }

    let template = if state.is_too_long() {
        include_str!("../../draft_ideas/select_random_production.sql")
    } else {
        include_str!("../../draft_ideas/select_shortest_production.sql")
    };

    let (row,): (String,) = sqlx::query_as(template)
        .bind(token.reference())
        .fetch_one(pool)
        .await
        .map_err(AppError::for_generation_in_sql)?;

    let branch = ProductionBranch::from_str(&row)?;

    let mut generation_lookup: HashMap<i32, String> = HashMap::new();

    state.increase_depth();
    for placeholder in branch.ordered_placeholder_references()? {
        generation_lookup.insert(
            placeholder.id(),
            generate_from_placeholder(placeholder, state, pool).await?,
        );
    }
    state.decrease_depth();

    let result = branch
        .placeholder_appearance_order_in_production()
        .iter()
        .filter_map(|i| generation_lookup.get(i))
        .join(" ");

    state.alter_length(result.len() as i32);

    Ok(result)
}

#[allow(unused)]
#[derive(FromRow)]
struct SelectedWord {
    id: i32,
    content: String,
    non_repeatable: bool,
    semantic_output: Vec<i32>,
    grammar_output: Vec<i32>,
}

#[allow(unused)]
#[async_recursion]
async fn generate_from_word_selector(
    token: &TokenReference,
    _state: &mut dyn GenerationState<i32>,
    pool: &Pool<Postgres>,
) -> AppResult<String> {
    //all of them
    let search_tag_names = token.reference().split(',').map(|s| s.trim()).collect_vec();
    let search_tags = retrieve_tag_ids_from_strings(&(search_tag_names), pool).await?;
    //any of them
    let semantic_tags = vec![1, 2, 3];
    //any of them
    let grammar_tags = vec![1, 2, 3];
    //any of them
    let context_grammar_tags = vec![2, 3, 4];
    //any of them
    let context_semantic_tags = vec![3, 4, 5];

    let template = include_str!("../../draft_ideas/select_random_word.sql")
        .replace(
            "<SELECTED_SEMANTIC_TAGS_PLACEHOLDERS>",
            (0..search_tags.len())
                .map(|i| format!("${}", i + 1))
                .join(",")
                .as_str(),
        )
        .replace(
            "<CONTEXTUAL_SEMANTIC_TAGS_PLACEHOLDERS>",
            (0..semantic_tags.len())
                .map(|i| format!("${}", i + search_tags.len() + 1))
                .join(",")
                .as_str(),
        )
        .replace(
            "<CONTEXTUAL_GRAMMAR_TAGS>",
            (0..grammar_tags.len())
                .map(|i| format!("${}", i + search_tags.len() + semantic_tags.len() + 1))
                .join(",")
                .as_str(),
        );

    let mut query = sqlx::query_as::<sqlx::Postgres, SelectedWord>(&template);

    for &tag in search_tags
        .iter()
        .chain(semantic_tags.iter())
        .chain(grammar_tags.iter())
    {
        query = query.bind(tag);
    }

    let selected_word = query
        .fetch_one(pool)
        .await
        .map_err(AppError::for_generation_in_sql)?;

    Ok("to".to_owned())
}

async fn retrieve_tag_ids_from_strings(
    tags: &[&str],
    pool: &Pool<Postgres>,
) -> AppResult<Vec<i32>> {
    let template = include_str!("../../draft_ideas/select_ids_of_tags.sql").replace(
        "<SEMANTIC_TAGS_PLACEHOLDERS>",
        (0..tags.len())
            .map(|i| format!("${}", i + 1))
            .join(",")
            .as_str(),
    );

    let mut query = sqlx::query_as::<sqlx::Postgres, (i32,)>(&template);

    for &tag in tags {
        query = query.bind(tag);
    }

    let ids = query
        .fetch_all(pool)
        .await
        .map_err(AppError::for_generation_in_sql)?;

    Ok(ids.into_iter().map(|(i,)| i).collect_vec())
}
