use thiserror::Error;

#[derive(Error, Debug, Clone)]
pub enum AppError {
    #[error("Upload failed. {0}")]
    Upload(#[from] UploadError),
    #[error("Infrastructure had an error. {0}")]
    Infrastructure(#[from] InfrastructureError),
    #[error("Some data produced an error. {0}")]
    Data(#[from] DataError),
}

impl AppError {
    pub fn for_upload(error: reqwest::Error) -> Self {
        UploadError::from(error).into()
    }
    pub fn for_infrastructure(error: reqwest::Error) -> Self {
        InfrastructureError::from(error).into()
    }
}

#[derive(Error, Debug, Clone)]
pub enum UploadError {
    #[error("Server had problems connecting to its dependencies.")]
    HttpFailed(#[from] HttpError),
}

impl From<reqwest::Error> for UploadError {
    fn from(error: reqwest::Error) -> Self {
        HttpError::from(error).into()
    }
}

#[derive(Error, Debug, Clone)]
pub enum InfrastructureError {
    #[error("Client is not available because: {0}")]
    ClientNotAvailable(#[from] HttpError),
}

impl From<reqwest::Error> for InfrastructureError {
    fn from(error: reqwest::Error) -> Self {
        HttpError::from(error).into()
    }
}

#[derive(Error, Debug, Clone)]
pub enum DataError {
    #[error("URL cannot be parsed because {0:?}")]
    UrlParseError(#[from] url::ParseError),
}

#[derive(Error, Debug, Clone)]
pub enum HttpError {
    #[error("{0}")]
    ReqwestError(String),
}

impl From<reqwest::Error> for HttpError {
    fn from(error: reqwest::Error) -> Self {
        Self::ReqwestError(format!("{}", error))
    }
}
