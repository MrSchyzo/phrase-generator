use std::sync::Arc;

use crate::app_core::types::upload::UploadedSpeech;
use crate::outgoing::tts_wrapper::TtsWrapper;
use async_trait::async_trait;
use rand::RngCore;

use crate::served::types::graphql::Speech;

use self::errors::AppError;

pub mod engine;
pub mod errors;
pub mod types;

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
