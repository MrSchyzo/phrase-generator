use std::sync::Arc;
use reqwest::{Client, Url};

type AppResult<T> = Result<T, AppError>;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct SpeechRequest {
    text: String,
    is_male: bool,
}

struct UploadResult {
    url: Url,
}

impl UploadResult {
    fn parse(url: &str) -> AppResult<Self> {
        Ok(Self {
            url: Url::parse(url)
                .map_err(DataError::from)
                .map_err(AppError::from)?,
        })
    }

    fn url(&self) -> &Url {
        &self.url
    }
}

#[derive(Error, Debug, Clone)]
enum HttpError {
    #[error("{0}")]
    ReqwestError(String),
}

impl From<reqwest::Error> for HttpError {
    fn from(error: reqwest::Error) -> Self {
        Self::ReqwestError(format!("{}", error))
    }
}

#[derive(Error, Debug, Clone)]
enum UploadError {
    #[error("Http failure: {0}")]
    HttpFailed(#[from] HttpError),
}

#[derive(Error, Debug, Clone)]
enum InfrastructureError {
    #[error("Client is not available because: {0}")]
    ClientNotAvailable(#[from] HttpError),
}

#[derive(Error, Debug, Clone)]
enum DataError {
    #[error("URL cannot be parsed because {0:?}")]
    UrlParseError(#[from] url::ParseError),
}

#[derive(Error, Debug, Clone)]
enum AppError {
    #[error("Upload failed. {0}")]
    Upload(#[from] UploadError),
    #[error("Infrastructure had an error. {0}")]
    Infrastructure(#[from] InfrastructureError),
    #[error("Some data produced an error. {0}")]
    Data(#[from] DataError),
}

#[async_trait]
trait AsyncUploader {
    async fn upload(&self, request: &SpeechRequest) -> AppResult<UploadResult>;

    async fn is_healthy(&self) -> AppResult<()>;
}

type AppUploader = dyn AsyncUploader + Send + Sync;

struct AppCore {
    uploader: Arc<AppUploader>,
}

impl AppCore {
    fn new(uploader: Arc<AppUploader>) -> Self {
        Self { uploader }
    }

    fn uploader(&self) -> &AppUploader {
        self.uploader.as_ref()
    }

    async fn is_healthy(&self) -> AppResult<()> {
        self.uploader().is_healthy().await
    }
}



#[derive(Clone)]
struct Uploader {
    client: Client,
}
impl Uploader {
    fn new(client: Client) -> Self {
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
            .map_err(HttpError::from)
            .map_err(UploadError::from)
            .map_err(AppError::from)?;

        result
            .text()
            .await
            .map_err(HttpError::from)
            .map_err(UploadError::from)
            .map_err(AppError::from)
            .and_then(|url| UploadResult::parse(&url))
    }

    async fn is_healthy(&self) -> AppResult<()> {
        (&self.client)
            .post("http://localhost:8080/speak")
            .json(&SpeechRequest {
                text: "Prova".to_owned(),
                is_male: true,
            })
            .send()
            .await
            .map_err(HttpError::from)
            .map_err(InfrastructureError::from)
            .map_err(AppError::from)
            .map(|_| ())
    }
}

struct Resolver;
impl Resolver {
    async fn generate_random() -> AppResult<Speech> {
        Ok(Speech {
            id: "1".to_owned(),
            text: "Ciao mondo".to_owned(),
        })
    }

    async fn upload_audio(text: &str, uploader: &AppUploader) -> AppResult<UploadResult> {
        uploader
            .upload(&SpeechRequest {
                text: text.to_owned(),
                is_male: true,
            })
            .await
    }
}
