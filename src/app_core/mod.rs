use std::str::FromStr;
use std::sync::Arc;

use crate::app_core::types::upload::UploadedSpeech;
use crate::outgoing::tts_wrapper::TtsWrapper;
use async_trait::async_trait;
use rand::RngCore;
use sqlx::postgres::PgPoolOptions;

use crate::served::types::graphql::Speech;

use self::errors::AppError;

pub mod engine;
pub mod errors;
pub mod types;
use crate::app_core::engine::types::ProductionBranch;

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

trait GenerationState {
    fn increase_depth(&mut self);
    fn decrease_depth(&mut self);
    fn is_too_deep(&self) -> bool;

    fn alter_length(&mut self, amount: i32);
    fn is_too_long(&self) -> bool;

    fn register_word(&mut self, word: &str);
    fn has_used_word(&self, word: &str) -> bool;
}

#[allow(unused)]
async fn random_production_step(
    nts_name: &str,
    mut state: Box<dyn GenerationState>,
) -> AppResult<String> {
    let pool = PgPoolOptions::new()
        .max_connections(8)
        .connect("postgres://postgres:password@localhost:49153/postgres")
        .await
        .map_err(AppError::for_generation)?;

    let mut conn = pool.acquire().await.map_err(AppError::for_generation)?;
    let query_template = include_str!("../../draft_ideas/select_random_production.sql");
    let query = sqlx::query_as(query_template).bind(nts_name);

    let (row,): (String,) = query
        .fetch_one(&mut conn)
        .await
        .map_err(AppError::for_generation)?;

    let branch = ProductionBranch::from_str(&row)?;

    state.increase_depth();
    let _placeholders = branch.ordered_placeholder_references()?;

    state.decrease_depth();
    Ok("".to_owned())
}
