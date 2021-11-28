use std::sync::Arc;

use actix_web::web::Data;
use actix_web::{guard, web, App, HttpResponse, HttpServer, Result};
use async_graphql::http::{playground_source, GraphQLPlaygroundConfig};
use async_graphql::{Context, EmptyMutation, EmptySubscription, Object, Schema};
use async_graphql_actix_web::{GraphQLRequest, GraphQLResponse};
use async_trait::async_trait;
use reqwest::{Client, Error as ReqwestError, Url};
use serde::Serialize;
use thiserror::Error;

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

type MySchema = Schema<QueryRoot, EmptyMutation, EmptySubscription>;
type AppResult<T> = Result<T, AppError>;
type AppUploader = dyn AsyncUploader + Send + Sync;

#[derive(Error, Debug, Clone)]
enum HttpError {
    #[error("{0}")]
    ReqwestError(String),
}

impl From<ReqwestError> for HttpError {
    fn from(error: ReqwestError) -> Self {
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

#[async_trait]
trait AsyncUploader {
    async fn upload(&self, request: &SpeechRequest) -> AppResult<UploadResult>;

    async fn is_healthy(&self) -> AppResult<()>;
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

struct QueryRoot;

#[Object]
impl QueryRoot {
    async fn random(&self) -> AppResult<Speech> {
        Resolver::generate_random().await
    }
}

struct Speech {
    id: String,
    text: String,
}

#[Object]
impl Speech {
    async fn id(&self) -> &str {
        &self.id
    }

    async fn text(&self) -> &str {
        &self.text
    }

    async fn audio_url<'ctx>(&self, ctx: &Context<'ctx>) -> AppResult<String> {
        let uploader = ctx.data_unchecked::<Arc<AppCore>>().uploader();
        let text = &self.text;
        Resolver::upload_audio(text, uploader)
            .await
            .map(|res| res.url().as_str().to_owned())
    }
}

async fn health(core: web::Data<Arc<AppCore>>) -> Result<HttpResponse> {
    match core.is_healthy().await {
        Ok(_) => Ok(HttpResponse::Ok().content_type("application/json").body("")),
        Err(error) => Ok(HttpResponse::InternalServerError().body(format!("{}", error))),
    }
}

async fn life() -> Result<HttpResponse> {
    Ok(HttpResponse::Ok().content_type("application/json").body(""))
}

async fn index(schema: web::Data<MySchema>, req: GraphQLRequest) -> GraphQLResponse {
    schema.execute(req.into_inner()).await.into()
}

async fn index_playground() -> Result<HttpResponse> {
    let source = playground_source(GraphQLPlaygroundConfig::new("/").subscription_endpoint("/"));
    Ok(HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(source))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let core = Arc::new(AppCore::new(Arc::new(Uploader::new(Client::new()))));

    let schema = Schema::build(QueryRoot, EmptyMutation, EmptySubscription)
        .data(core.clone())
        .finish();

    println!("Playground: http://localhost:8000");

    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(schema.clone()))
            .app_data(Data::new(core.clone()))
            .service(web::resource("/health").guard(guard::Get()).to(health))
            .service(web::resource("/life").guard(guard::Get()).to(life))
            .service(web::resource("/").guard(guard::Post()).to(index))
            .service(web::resource("/").guard(guard::Get()).to(index_playground))
    })
    .bind("0.0.0.0:8000")?
    .run()
    .await
}
