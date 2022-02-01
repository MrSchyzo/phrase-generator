use async_trait::async_trait;
use reqwest::{Client, Url};

use types::{Speech, SpeechRequest, UploadResult};

use crate::app_core::errors::AppError;
use crate::app_core::AppResult;
use crate::utils::{LogLevel, Loggable};

pub mod types;

pub type TtsWrapper = dyn TtsWrapperClient + Send + Sync;

#[async_trait]
pub trait TtsWrapperClient {
    async fn health(&self) -> AppResult<()>;

    async fn upload(&self, request: Speech) -> AppResult<UploadResult>;
}

pub struct TtsWrapperConnectionOpts {
    pub root_url: Url,
}

pub struct SimpleTtsWrapperClient {
    client: Client,
    connection_options: TtsWrapperConnectionOpts,
}

impl SimpleTtsWrapperClient {
    pub fn new(client: Client, connection_options: TtsWrapperConnectionOpts) -> Self {
        Self {
            client,
            connection_options,
        }
    }
}

#[async_trait]
impl TtsWrapperClient for SimpleTtsWrapperClient {
    async fn health(&self) -> AppResult<()> {
        (&self.client)
            .post(format!("{}/speak", &self.connection_options.root_url))
            .json(&(SpeechRequest::default()))
            .send()
            .await
            .log_err("Uploader is not healthy", LogLevel::Warning)
            .map_err(AppError::for_infrastructure_http_client_failed)
            .map(|_| ())
    }

    async fn upload(&self, request: Speech) -> AppResult<UploadResult> {
        (&self.client)
            .post(format!("{}/speak", &self.connection_options.root_url))
            .json(&(SpeechRequest::from(request)))
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
