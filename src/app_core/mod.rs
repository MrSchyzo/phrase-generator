pub mod errors;
pub mod types;

use std::sync::Arc;

use async_trait::async_trait;
use rand::RngCore;
use reqwest::{Client, Url};
use serde::Serialize;

use crate::served::types::graphql::Speech;
use crate::utils::{LogLevel, Loggable};

use self::errors::{AppError, DataError};

pub type AppResult<T> = Result<T, AppError>;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SpeechRequest {
    text: String,
    is_male: bool,
}

impl From<SpeechToUpload> for SpeechRequest {
    fn from(this: SpeechToUpload) -> Self {
        Self {
            text: this.text,
            is_male: this.is_male,
        }
    }
}

#[derive(Clone)]
pub struct SpeechToUpload {
    pub text: String,
    pub is_male: bool,
}

pub struct SpeechGenerationOptions {}

pub struct UploadResult {
    url: Url,
}

impl UploadResult {
    pub fn parse(url: &str) -> AppResult<Self> {
        Ok(Self {
            url: Url::parse(url)
                .map_err(DataError::from)
                .map_err(AppError::from)?,
        })
    }

    pub fn url(&self) -> &Url {
        &self.url
    }
}

#[async_trait]
pub trait AsyncHealth {
    async fn is_healthy(&self) -> AppResult<()>;
}

#[async_trait]
pub trait AsyncUploader {
    async fn upload(&self, request: &SpeechToUpload) -> AppResult<UploadResult>;
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
    client: Client,
}
impl Uploader {
    pub fn new(client: Client) -> Self {
        Self { client }
    }
}

#[async_trait]
impl AsyncUploader for Uploader {
    async fn upload(&self, request: &SpeechToUpload) -> AppResult<UploadResult> {
        let req: SpeechRequest = request.clone().into();
        (&self.client)
            .post("http://localhost:8080/speak")
            .json(&req)
            .send()
            .await
            .log_err("Unable to upload the requested speech", LogLevel::Error)
            .map_err(AppError::for_upload)?
            .text()
            .await
            .log_err("Unable to retrieve the response", LogLevel::Error)
            .map_err(AppError::for_upload)
            .and_then(|url| UploadResult::parse(&url))
    }
}

#[async_trait]
impl AsyncHealth for Uploader {
    async fn is_healthy(&self) -> AppResult<()> {
        (&self.client)
            .post("http://localhost:8080/speak")
            .json(&SpeechRequest {
                text: "Prova".to_owned(),
                is_male: true,
            })
            .send()
            .await
            .log_err("Uploader is not healthy", LogLevel::Warning)
            .map_err(AppError::for_infrastructure)
            .map(|_| ())
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
