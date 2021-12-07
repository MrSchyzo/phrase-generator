use reqwest::Url;

use crate::app_core::errors::{AppError, DataError};
use crate::app_core::AppResult;

pub struct UploadResult {
    pub url: Url,
}

impl UploadResult {
    pub fn parse(url: &str) -> AppResult<Self> {
        Ok(Self {
            url: Url::parse(url)
                .map_err(DataError::from)
                .map_err(AppError::from)?,
        })
    }
}
