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
use sqlx::{FromRow, Transaction};
use sqlx::{Pool, Postgres};

use self::errors::AppError;

pub mod engine;
pub mod errors;
pub mod types;
use crate::app_core::engine::types::parsing::TokenReference;
use crate::app_core::engine::types::{PlaceholderReference, ProductionBranch};
use crate::app_core::errors::GenerationError;
use crate::utils::{LogLevel, Loggable};

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

#[async_trait]
pub trait AsyncHealthyPhraseGenerator: AsyncPhraseGenerator + AsyncHealth {}

type AppUploader = dyn AsyncHealthyUploader + Send + Sync;
type AppPhraseGenerator = dyn AsyncHealthyPhraseGenerator + Send + Sync;

pub struct AppCore {
    uploader: Arc<AppUploader>,
    generator: Arc<AppPhraseGenerator>,
    pool: Arc<Pool<Postgres>>,
}

impl AppCore {
    pub fn new(
        uploader: Arc<AppUploader>,
        generator: Arc<AppPhraseGenerator>,
        pool: Arc<Pool<Postgres>>,
    ) -> Self {
        Self {
            uploader,
            generator,
            pool,
        }
    }

    pub fn uploader(&self) -> &AppUploader {
        self.uploader.as_ref()
    }

    pub fn generator(&self) -> &AppPhraseGenerator {
        self.generator.as_ref()
    }

    pub fn pool(&self) -> &Pool<Postgres> {
        self.pool.as_ref()
    }

    pub async fn is_healthy(&self) -> AppResult<()> {
        let (generator_res, uploader_res) =
            futures::join!(self.generator().is_healthy(), self.uploader().is_healthy());

        if generator_res.is_err() {
            generator_res
        } else {
            uploader_res
        }
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

pub struct PhraseGenerator {
    pool: Arc<Pool<Postgres>>,
}

impl PhraseGenerator {
    pub fn new(pool: Arc<Pool<Postgres>>) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl AsyncPhraseGenerator for PhraseGenerator {
    // WHAT IS THIS SMOKING PILE OF SPAGHETT'
    async fn generate(&self, opts: SpeechGenerationOptions) -> AppResult<Speech> {
        let max_phrases = 2048u64; //TODO: hardcoded

        let mut transaction = self
            .pool
            .begin()
            .await
            .map_err(AppError::for_generation_in_sql)?;

        let current_total = sqlx::query_scalar::<_, i64>("SELECT COUNT(id) FROM generated_phrase")
            .fetch_one(&mut transaction)
            .await
            .map_err(AppError::for_generation_in_sql)?;

        let remaining = (max_phrases as i64 - current_total).max(0);

        let result = if rand::thread_rng().next_u64() % max_phrases < (remaining as u64) {
            tracing::info!("Generating new phrase, remaining {remaining}");
            //TODO: add some retry, generation can fail
            let s = generate_phrase(opts, &mut transaction).await?;
            if let Some(id) = sqlx::query_scalar::<_, sqlx::types::Uuid>(
                "SELECT id FROM generated_phrase WHERE content = $1",
            )
            .bind(&s)
            .fetch_optional(&mut transaction)
            .await
            .map_err(AppError::for_generation_in_sql)?
            {
                Ok((id, s))
            } else {
                sqlx::query!(
                    "INSERT INTO generated_phrase (content) VALUES ($1) RETURNING id",
                    &s
                )
                .fetch_one(&mut transaction)
                .await
                .map(|res| res.id)
                .map_err(AppError::for_generation_in_sql)
                .map(|uuid| (uuid, s))
            }
        } else {
            tracing::info!("Extracting existing phrase, remaining {remaining}");

            sqlx::query_as::<_, (sqlx::types::Uuid, String)>(
                "SELECT id, content FROM generated_phrase ORDER BY random() limit 1",
            )
            .fetch_one(&mut transaction)
            .await
            .map_err(AppError::for_generation_in_sql)
        }
        .map(|(uuid, text)| Speech {
            id: uuid.to_string(),
            text,
        })?;

        transaction
            .commit()
            .await
            .map_err(AppError::for_generation_in_sql)?;

        Ok(result)
    }
}

#[async_trait]
impl AsyncHealth for PhraseGenerator {
    async fn is_healthy(&self) -> AppResult<()> {
        self.pool
            .acquire()
            .await
            .map(|_| ())
            .map_err(AppError::for_infrastructure_db_connections_unavailable)
            .log_err("DB is not healthy", LogLevel::Warning)
    }
}

#[async_trait]
impl AsyncHealthyPhraseGenerator for PhraseGenerator {}

type TrivialGenerationSubStep = GenerationSubStep<i32, i32, i32>;
type TrivialGenerationState = dyn GenerationState<i32, i32, i32, i32>;

#[derive(Clone)]
pub struct GenerationSubStep<
    Placeholder: Sized + Hash + Send + Clone + Eq,
    Semantics: Sized + Hash + Send + Clone + Eq,
    Grammar: Sized + Hash + Send + Clone + Eq,
> {
    grammar_context: HashSet<Grammar>,
    semantic_context: HashSet<Semantics>,

    grammar_lookup: HashMap<Placeholder, HashSet<Grammar>>,
    semantics_lookup: HashMap<Placeholder, HashSet<Semantics>>,
}

impl<
        Placeholder: Sized + Hash + Send + Clone + Eq,
        Semantics: Sized + Hash + Send + Clone + Eq,
        Grammar: Sized + Hash + Send + Clone + Eq,
    > GenerationSubStep<Placeholder, Semantics, Grammar>
{
    pub fn new() -> Self {
        Self {
            grammar_context: HashSet::new(),
            semantic_context: HashSet::new(),
            grammar_lookup: HashMap::new(),
            semantics_lookup: HashMap::new(),
        }
    }

    pub fn propagate_grammar_tags(&mut self, tags: Vec<Grammar>) {
        for tag in tags.into_iter() {
            self.grammar_context.insert(tag);
        }
    }

    pub fn propagate_semantic_tags(&mut self, tags: Vec<Semantics>) {
        for tag in tags.into_iter() {
            self.semantic_context.insert(tag);
        }
    }

    pub fn register_grammar_tags(&mut self, placeholder: Placeholder, tags: Vec<Grammar>) -> bool {
        self.grammar_lookup
            .insert(placeholder, tags.into_iter().collect())
            == None
    }

    pub fn register_semantic_tags(
        &mut self,
        placeholder: Placeholder,
        tags: Vec<Semantics>,
    ) -> bool {
        self.semantics_lookup
            .insert(placeholder, tags.into_iter().collect())
            == None
    }

    pub fn get_grammar_of(&self, placeholder: &Placeholder) -> Option<Vec<&Grammar>> {
        self.grammar_lookup
            .get(placeholder)
            .map(|grammar| grammar.iter().collect_vec())
    }

    pub fn get_semantics_of(&self, placeholder: &Placeholder) -> Option<Vec<&Semantics>> {
        self.semantics_lookup
            .get(placeholder)
            .map(|grammar| grammar.iter().collect_vec())
    }

    pub fn deconstruct_context(&self) -> (Vec<&Semantics>, Vec<&Grammar>) {
        (
            self.semantic_context.iter().collect_vec(),
            self.grammar_context.iter().collect_vec(),
        )
    }
}

trait GenerationState<
    Word: Sized + Hash + Send + Clone + Eq,
    Placeholder: Sized + Hash + Send + Clone + Eq,
    Semantics: Sized + Hash + Send + Clone + Eq,
    Grammar: Sized + Hash + Send + Clone + Eq,
>: Send
{
    fn begin_generation_sub_step(&mut self);
    fn end_generation_sub_step(
        &mut self,
    ) -> Option<GenerationSubStep<Placeholder, Semantics, Grammar>>;

    fn current_depth(&self) -> u16;
    fn is_too_deep(&self) -> bool;

    fn propagate_semantics(&mut self, semantics: Vec<Semantics>);
    fn propagate_grammar(&mut self, grammar: Vec<Grammar>);
    fn current_context(&self) -> (Vec<&Semantics>, Vec<&Grammar>);

    fn register_semantics(&mut self, placeholder: Placeholder, semantics: Vec<Semantics>) -> bool;
    fn register_grammar(&mut self, placeholder: Placeholder, grammar: Vec<Grammar>) -> bool;
    fn extract_placeholder_semantics(&self, placeholder: &Placeholder) -> Option<Vec<&Semantics>>;
    fn extract_placeholder_grammar(&self, placeholder: &Placeholder) -> Option<Vec<&Grammar>>;

    fn alter_length(&mut self, amount: i32);
    fn is_too_long(&self) -> bool;

    fn register_word(&mut self, word: Word);
    fn unregister_word(&mut self, word: &Word);
    fn used_words(&self) -> Vec<&Word>;
    fn has_used_word(&self, word: &Word) -> bool;
}

pub struct InMemoryGenerationState {
    max_depth: u16,

    length: i32,
    max_length: i32,

    used_words: HashSet<i32>,
    sub_steps: Vec<TrivialGenerationSubStep>,

    current_sub_step: TrivialGenerationSubStep,
}

impl InMemoryGenerationState {
    #[allow(unused)]
    pub fn new(max_depth: u16, max_length: i32) -> Self {
        Self {
            max_depth,
            length: 0i32,
            max_length,
            used_words: HashSet::new(),
            sub_steps: Vec::new(),
            current_sub_step: GenerationSubStep::new(),
        }
    }
}

impl GenerationState<i32, i32, i32, i32> for InMemoryGenerationState {
    fn begin_generation_sub_step(&mut self) {
        self.sub_steps.push(self.current_sub_step.clone());
        self.current_sub_step = GenerationSubStep::new()
    }

    fn end_generation_sub_step(&mut self) -> Option<TrivialGenerationSubStep> {
        if let Some(sub_step) = self.sub_steps.pop() {
            let current_sub_step = self.current_sub_step.clone();
            self.current_sub_step = sub_step;
            return Some(current_sub_step);
        }

        None
    }

    fn current_depth(&self) -> u16 {
        self.sub_steps.len() as u16
    }

    fn is_too_deep(&self) -> bool {
        self.current_depth() > self.max_depth
    }

    fn propagate_semantics(&mut self, semantics: Vec<i32>) {
        self.current_sub_step.propagate_semantic_tags(semantics)
    }

    fn propagate_grammar(&mut self, grammar: Vec<i32>) {
        self.current_sub_step.propagate_grammar_tags(grammar)
    }

    fn current_context(&self) -> (Vec<&i32>, Vec<&i32>) {
        self.current_sub_step.deconstruct_context()
    }

    fn register_semantics(&mut self, placeholder: i32, semantics: Vec<i32>) -> bool {
        self.current_sub_step
            .register_semantic_tags(placeholder, semantics)
    }

    fn register_grammar(&mut self, placeholder: i32, grammar: Vec<i32>) -> bool {
        self.current_sub_step
            .register_grammar_tags(placeholder, grammar)
    }

    fn extract_placeholder_semantics(&self, placeholder: &i32) -> Option<Vec<&i32>> {
        self.current_sub_step.get_semantics_of(placeholder)
    }

    fn extract_placeholder_grammar(&self, placeholder: &i32) -> Option<Vec<&i32>> {
        self.current_sub_step.get_grammar_of(placeholder)
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

    fn used_words(&self) -> Vec<&i32> {
        self.used_words.iter().collect_vec()
    }

    fn has_used_word(&self, word: &i32) -> bool {
        self.used_words.contains(word)
    }
}

async fn generate_phrase(
    _: SpeechGenerationOptions,
    transaction: &mut Transaction<'_, Postgres>,
) -> AppResult<String> {
    let mut state = InMemoryGenerationState::new(100, 500);

    generate_from_non_terminal_symbol(
        &TokenReference::new_trivial_reference("Start".to_owned()),
        &mut state,
        transaction,
    )
    .await
    .map(|s| {
        s.replace("-", "")
            .replace("_", "")
            .replace("  ", " ")
            .trim()
            .to_owned()
    })
}

#[async_recursion]
async fn generate_from_placeholder(
    placeholder: &PlaceholderReference,
    state: &mut TrivialGenerationState,
    transaction: &mut Transaction<'_, Postgres>,
) -> AppResult<String> {
    match placeholder {
        PlaceholderReference::NonTerminalSymbol(nts) => {
            generate_from_non_terminal_symbol(nts, state, transaction).await
        }
        PlaceholderReference::WordSelector(word) => {
            generate_from_word_selector(word, state, transaction).await
        }
    }
}

#[async_recursion]
async fn generate_from_non_terminal_symbol(
    token: &TokenReference,
    state: &mut TrivialGenerationState,
    transaction: &mut Transaction<'_, Postgres>,
) -> AppResult<String> {
    if state.is_too_deep() {
        return Err(GenerationError::ExcessiveDepth(state.current_depth()).into());
    }

    let branch = pick_production(token, state, transaction).await?;

    let (semantics, grammar) = compute_semantic_and_grammar_dependencies(token, state)?;
    let mut generation_lookup: HashMap<i32, String> = HashMap::new();

    tracing::info!("Dependency on grammar: {:?}", grammar);
    tracing::info!("Dependency on semantics: {:?}", semantics);

    state.begin_generation_sub_step();
    state.propagate_grammar(grammar);
    state.propagate_semantics(semantics);
    for placeholder in branch.ordered_placeholder_references()? {
        generation_lookup.insert(
            placeholder.id(),
            generate_from_placeholder(placeholder, state, transaction).await?,
        );
    }
    if let Some((semantics, grammar)) = state
        .end_generation_sub_step()
        .as_ref()
        .map(GenerationSubStep::deconstruct_context)
    {
        tracing::info!(
            "Token {} has released grammar {:?} and semantics {:?}",
            token.id(),
            grammar,
            semantics,
        );
        state.register_grammar(token.id(), grammar.into_iter().copied().collect());
        state.register_semantics(token.id(), semantics.into_iter().copied().collect());
    } else {
        return Err(AppError::for_generation_non_existent_sub_step());
    }

    let result = branch
        .placeholder_appearance_order_in_production()
        .iter()
        .filter_map(|i| generation_lookup.get(i))
        .join(" ");

    state.alter_length(result.len() as i32);

    Ok(result)
}

async fn pick_production(
    token: &TokenReference,
    state: &mut TrivialGenerationState,
    transaction: &mut Transaction<'_, Postgres>,
) -> AppResult<ProductionBranch> {
    let template = if !state.is_too_long() {
        include_str!("../../draft_ideas/select_random_production.sql")
    } else {
        include_str!("../../draft_ideas/select_shortest_production.sql")
    };

    let (row,): (String,) = sqlx::query_as::<Postgres, (String,)>(template)
        .bind(token.reference())
        .fetch_optional(transaction)
        .await
        .map_err(AppError::for_generation_in_sql)?
        .ok_or_else(|| {
            AppError::for_generation_no_production_branches_found(token.reference().to_string())
        })?;

    ProductionBranch::from_str(&row)
}

#[derive(FromRow)]
struct SelectedWord {
    id: i32,
    content: String,
    non_repeatable: bool,
    semantic_output: Vec<i32>,
    grammar_output: Vec<i32>,
}

#[async_recursion]
async fn generate_from_word_selector(
    token: &TokenReference,
    state: &mut TrivialGenerationState,
    transaction: &mut Transaction<'_, Postgres>,
) -> AppResult<String> {
    let selected_word = pick_word(token, state, transaction).await?;

    tracing::info!("Found word: {}", selected_word.content);
    tracing::info!("Semantics: {:?}", selected_word.semantic_output);
    tracing::info!("Grammar: {:?}", selected_word.grammar_output);

    state.register_semantics(token.id(), selected_word.semantic_output.clone());
    state.register_grammar(token.id(), selected_word.grammar_output.clone());

    if token.grammar_can_propagate() {
        state.propagate_grammar(selected_word.grammar_output);
    }

    if token.semantic_can_propagate() {
        state.propagate_semantics(selected_word.semantic_output);
    }

    if selected_word.non_repeatable {
        state.register_word(selected_word.id)
    }

    Ok(selected_word.content)
}

async fn pick_word(
    token: &TokenReference,
    state: &mut TrivialGenerationState,
    transaction: &mut Transaction<'_, Postgres>,
) -> AppResult<SelectedWord> {
    let search_tag_names = token.reference().split(',').map(|s| s.trim()).collect_vec();
    let search_tags = retrieve_tag_ids_from_strings(&(search_tag_names), transaction).await?;
    let used_words = state
        .used_words()
        .into_iter()
        .chain(vec![&i32::MIN])
        .collect_vec();
    let (semantic_tags, grammar_tags) = compute_semantic_and_grammar_dependencies(token, state)?;

    tracing::info!("search_tags: {:?}", search_tags);
    tracing::info!("semantic_tags: {:?}", semantic_tags);
    tracing::info!("grammar_tags: {:?}", grammar_tags);
    tracing::info!("used_words: {:?}", used_words);

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
            "<CONTEXTUAL_GRAMMAR_TAGS_PLACEHOLDERS>",
            (0..grammar_tags.len())
                .map(|i| format!("${}", i + search_tags.len() + semantic_tags.len() + 1))
                .join(",")
                .as_str(),
        )
        .replace(
            "<USED_WORDS>",
            (0..used_words.len())
                .map(|i| {
                    format!(
                        "${}",
                        i + search_tags.len() + semantic_tags.len() + grammar_tags.len() + 1
                    )
                })
                .join(",")
                .as_str(),
        );

    let mut query = sqlx::query_as::<sqlx::Postgres, SelectedWord>(&template);

    for &tag in search_tags
        .iter()
        .chain(semantic_tags.iter())
        .chain(grammar_tags.iter())
        .chain(used_words.into_iter())
    {
        tracing::info!("Binding parameter to value {}", tag);
        query = query.bind(tag);
    }

    query
        .fetch_optional(transaction)
        .await
        .map_err(AppError::for_generation_in_sql)?
        .ok_or_else(AppError::for_generation_no_words_found)
}

fn compute_semantic_and_grammar_dependencies(
    token: &TokenReference,
    state: &TrivialGenerationState,
) -> AppResult<(Vec<i32>, Vec<i32>)> {
    let (context_semantics, context_grammar) = state.current_context();

    let semantic_tags = match (
        token.semantic_dependency_on_other(),
        token.semantic_depends_on_context(),
    ) {
        (None, false) => vec![],
        (None, true) => context_semantics,
        (Some(ref id), false) => state
            .extract_placeholder_semantics(id)
            .ok_or_else(|| GenerationError::NonRegisteredPlaceholder(*id))?,
        (Some(ref id), true) => state
            .extract_placeholder_semantics(id)
            .ok_or_else(|| GenerationError::NonRegisteredPlaceholder(*id))?
            .into_iter()
            .chain(context_semantics)
            .collect(),
    };

    let grammar_tags = match (
        token.grammar_dependency_on_other(),
        token.grammar_depends_on_context(),
    ) {
        (None, false) => vec![],
        (None, true) => context_grammar,
        (Some(ref id), false) => state
            .extract_placeholder_grammar(id)
            .ok_or_else(|| GenerationError::NonRegisteredPlaceholder(*id))?,
        (Some(ref id), true) => state
            .extract_placeholder_grammar(id)
            .ok_or_else(|| GenerationError::NonRegisteredPlaceholder(*id))?
            .into_iter()
            .chain(context_grammar)
            .collect(),
    };

    Ok((
        semantic_tags.into_iter().copied().collect(),
        grammar_tags.into_iter().copied().collect(),
    ))
}

async fn retrieve_tag_ids_from_strings(
    tags: &[&str],
    transaction: &mut Transaction<'_, Postgres>,
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
        .fetch_all(transaction)
        .await
        .map_err(AppError::for_generation_in_sql)?;

    Ok(ids.into_iter().map(|(i,)| i).collect_vec())
}
