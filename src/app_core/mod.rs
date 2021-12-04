pub mod errors;

use std::sync::Arc;

use async_trait::async_trait;
use reqwest::{Client, Url};
use serde::Serialize;

use crate::served::types::Speech;

use self::errors::{AppError, DataError};

pub type AppResult<T> = Result<T, AppError>;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SpeechRequest {
    text: String,
    is_male: bool,
}

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
pub trait AsyncUploader {
    async fn upload(&self, request: &SpeechRequest) -> AppResult<UploadResult>;
}

#[async_trait]
pub trait AsyncHealth {
    async fn is_healthy(&self) -> AppResult<()>;
}

#[async_trait]
pub trait AsyncHealthyUploader: AsyncUploader + AsyncHealth {}

type AppUploader = dyn AsyncHealthyUploader + Send + Sync;

pub struct AppCore {
    uploader: Arc<AppUploader>,
}

impl AppCore {
    pub fn new(uploader: Arc<AppUploader>) -> Self {
        Self { uploader }
    }

    pub fn uploader(&self) -> &AppUploader {
        self.uploader.as_ref()
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
    async fn upload(&self, request: &SpeechRequest) -> AppResult<UploadResult> {
        let result = (&self.client)
            .post("http://localhost:8080/speak")
            .json(request)
            .send()
            .await
            .map_err(AppError::for_upload)?;

        result
            .text()
            .await
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
            .map_err(AppError::for_infrastructure)
            .map(|_| ())
    }
}

#[async_trait]
impl AsyncHealthyUploader for Uploader {}

pub struct Resolver;
impl Resolver {
    pub async fn generate_random() -> AppResult<Speech> {
        Ok(Speech {
            id: "1".to_owned(),
            text: "Ciao mondo".to_owned(),
        })
    }

    pub async fn upload_audio(text: &str, uploader: &AppUploader) -> AppResult<UploadResult> {
        uploader
            .upload(&SpeechRequest {
                text: text.to_owned(),
                is_male: true,
            })
            .await
    }
}
