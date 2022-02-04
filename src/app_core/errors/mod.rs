use sqlx::Error;
use std::num::ParseIntError;

use thiserror::Error;

#[derive(Error, Debug, Clone)]
pub enum AppError {
    #[error("Upload failed. {0}")]
    Upload(#[from] UploadError),
    #[error("Generation failed. {0}")]
    Generation(#[from] GenerationError),
    #[error("Infrastructure had an error. {0}")]
    Infrastructure(#[from] InfrastructureError),
    #[error("Some data produced an error. {0}")]
    Data(#[from] DataError),
    #[error("Multiple errors: {0:?}")]
    Multiple(Vec<AppError>),
}

impl AppError {
    pub fn for_upload(error: reqwest::Error) -> Self {
        UploadError::from(error).into()
    }
    pub fn for_upload_in_sql(error: sqlx::Error) -> Self {
        UploadError::from(error).into()
    }
    pub fn for_upload_in_sql_uuid(error: sqlx::types::uuid::Error) -> Self {
        UploadError::from(error).into()
    }
    pub fn for_generation_in_sql(error: sqlx::Error) -> Self {
        GenerationError::from(error).into()
    }
    pub fn for_generation_no_words_found() -> Self {
        GenerationError::NoWordsFound.into()
    }
    pub fn for_generation_no_production_branches_found(name: String) -> Self {
        GenerationError::NoProductionBranchesFound(name).into()
    }
    pub fn for_generation_non_registered_placeholder(placeholder_id: i32) -> Self {
        GenerationError::NonRegisteredPlaceholder(placeholder_id).into()
    }
    pub fn for_generation_non_existent_sub_step() -> Self {
        GenerationError::NonExistentSubStep.into()
    }
    pub fn for_infrastructure_http_client_failed(error: reqwest::Error) -> Self {
        InfrastructureError::from(error).into()
    }
    pub fn for_infrastructure_db_connections_unavailable(error: sqlx::Error) -> Self {
        InfrastructureError::from(error).into()
    }
    pub fn for_multiple_errors(errors: Vec<AppError>) -> Self {
        Self::Multiple(errors)
    }
    pub fn for_regex_did_not_recognize(string_to_recognize: String) -> Self {
        DataError::GrammarParse(ParseError::RegexDidNotRecognize(string_to_recognize)).into()
    }
    pub fn for_group_not_found(group_name: String, string_to_recognize: String) -> Self {
        DataError::GrammarParse(ParseError::GroupNotFound(group_name, string_to_recognize)).into()
    }
    pub fn for_number_parse_error(number_string: String, reason: ParseIntError) -> Self {
        DataError::GrammarParse(ParseError::CannotParseToNumber(number_string, reason)).into()
    }
    pub fn for_unrecognized_dependency_marker(marker: String) -> Self {
        DataError::GrammarParse(ParseError::UnrecognizedDependencyMarker(marker)).into()
    }
    pub fn for_production_id_clash(clashing_id: i32) -> Self {
        DataError::Production(ProductionError::IdClash(clashing_id)).into()
    }
    pub fn for_production_cycle_detected(cycle_ids: Vec<i32>) -> Self {
        DataError::Production(ProductionError::CycleDetected(cycle_ids)).into()
    }
}

#[derive(Error, Debug, Clone)]
pub enum UploadError {
    #[error("Server had problems connecting to its dependencies.")]
    HttpFailed(#[from] HttpError),
    #[error("DB Error, {0}")]
    DBFailed(String),
    #[error("DB UUID parsing failed, {0}")]
    UuidNotParsed(String),
}

#[derive(Error, Debug, Clone)]
pub enum GenerationError {
    #[error("DB Error, {0}.")]
    DBFailed(String),
    #[error("Generation overtook depth limit: last detected was {0}.")]
    ExcessiveDepth(u16),
    #[error("Retrieving a non-existent generation sub-step")]
    NonExistentSubStep,
    #[error("Retrieving a non-registered placeholder for the current generation: {0}")]
    NonRegisteredPlaceholder(i32),
    #[error("Unable to find any suitable word during word resolution.")]
    NoWordsFound,
    #[error("Unable to find any suitable production branches for NTS named '{0}'")]
    NoProductionBranchesFound(String),
}

impl From<sqlx::Error> for GenerationError {
    fn from(e: Error) -> Self {
        Self::DBFailed(format!("{e}"))
    }
}

impl From<sqlx::Error> for InfrastructureError {
    fn from(e: Error) -> Self {
        Self::DBConnectionsUnavailable(format!("{e}"))
    }
}

impl From<sqlx::Error> for UploadError {
    fn from(e: Error) -> Self {
        Self::DBFailed(format!("{e}"))
    }
}

impl From<sqlx::types::uuid::Error> for UploadError {
    fn from(e: sqlx::types::uuid::Error) -> Self {
        Self::DBFailed(format!("{e}"))
    }
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
    #[error("No DB connection seems available: {0}")]
    DBConnectionsUnavailable(String),
}

impl From<reqwest::Error> for InfrastructureError {
    fn from(error: reqwest::Error) -> Self {
        HttpError::from(error).into()
    }
}

#[derive(Error, Debug, Clone)]
pub enum DataError {
    #[error("URL cannot be parsed because {0:?}")]
    UrlParse(#[from] url::ParseError),
    #[error("Grammar regex error, {0:?}")]
    GrammarParse(#[from] ParseError),
    #[error("Grammar regex errors, {0:?}")]
    GrammarParses(Vec<ParseError>),
    #[error("Production is not well formed, {0:?}")]
    Production(#[from] ProductionError),
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
    #[error("unable to parse '{0}' to a number because {1:?}")]
    CannotParseToNumber(String, ParseIntError),
    #[error("cannot find regex group '{0}' inside '{1}'")]
    GroupNotFound(String, String),
    #[error("cannot recognize dependency marker '{0}'")]
    UnrecognizedDependencyMarker(String),
}

#[derive(Error, Debug, Clone)]
pub enum ProductionError {
    #[error("this production has an ID collision, a colliding ID is '{0}'")]
    IdClash(i32),
    #[error("a dependency cycle has been detected with a walk through the following ids: {0:?}")]
    CycleDetected(Vec<i32>),
}
