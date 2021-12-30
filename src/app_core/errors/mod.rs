use std::num::ParseIntError;

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
    pub fn for_unrecognized_placeholder(placeholder: String) -> Self {
        DataError::GrammarParseError(ParseError::RegexDidNotRecognize(placeholder)).into()
    }
    pub fn for_group_not_found(group_name: String, placeholder: String) -> Self {
        DataError::GrammarParseError(ParseError::GroupNotFound(group_name, placeholder)).into()
    }
    pub fn for_number_parse_error(
        number_string: String,
        placeholder: String,
        reason: ParseIntError,
    ) -> Self {
        DataError::GrammarParseError(ParseError::CannotParseToNumber(
            number_string,
            placeholder,
            reason,
        ))
        .into()
    }
    pub fn for_unrecognized_dependency_marker(marker: String) -> Self {
        DataError::GrammarParseError(ParseError::UnrecognizedDependencyMarker(marker)).into()
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
    #[error("Grammar regex error, {0:?}")]
    GrammarParseError(#[from] ParseError),
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

#[derive(Error, Debug, Clone)]
pub enum ParseError {
    #[error("regex did not recognize '{0}'")]
    RegexDidNotRecognize(String),
    #[error("unable to parse '{0}' to a number inside '{1}' because {2:?}")]
    CannotParseToNumber(String, String, ParseIntError),
    #[error("cannot find regex group '{0}' inside '{1}'")]
    GroupNotFound(String, String),
    #[error("cannot recognize dependency marker '{0}'")]
    UnrecognizedDependencyMarker(String),
}
