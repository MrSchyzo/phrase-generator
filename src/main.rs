use std::time::Instant;
use actix_web::web::Data;
use actix_web::{guard, web, App, HttpResponse, HttpServer, Result};
use async_graphql::http::{playground_source, GraphQLPlaygroundConfig};
use async_graphql::{EmptyMutation, EmptySubscription, Schema, Object, Context};
use async_graphql_actix_web::{GraphQLRequest, GraphQLResponse};
use async_trait::async_trait;
use serde::Serialize;
use reqwest::{Client, Url};

struct AppCore {
    uploader: Uploader,
}

type MySchema = Schema<QueryRoot, EmptyMutation, EmptySubscription>;
type Res<T> = Result<T, String>;

type AppResult<T> = Result<T, AppError>;

enum HttpError {
    GenericError(String)
}

enum UploadError {
    HttpFailed(HttpError)
}

enum InfrastructureError {

}

enum AppError {
    UploadError(UploadError),
    InfrastructureError(InfrastructureError),
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct SpeechRequest {
    text: String,
    is_male: bool,
}

struct UploadResult {
    url: Url
}

#[async_trait]
trait AsyncUploader {
    async fn upload(&self, request: &SpeechRequest) -> AppResult<UploadResult>;

    async fn is_healthy(&self) -> AppResult<()>;
}
#[derive(Clone)]
struct Uploader {
    client: Client
}
impl Uploader {
    fn new(client: Client) -> Self {
        Self {client}
    }

    async fn upload(&self, request: &SpeechRequest) -> Res<String> {
        let now = Instant::now();
        let result = (&self.client).post("http://localhost:8080/speak").json(request).send().await.map_err(|e| format!("Failed! {:?}", e))?;

        println!("GOOD: {}ms", now.elapsed().as_millis());
        result.text().await.map_err(|e| format!("Failed! {:?}", e))
    }

    async fn is_healthy(&self) -> bool {
        (&self.client).post("http://localhost:8080/speak").json(&SpeechRequest {
            text: "Prova".to_owned(),
            is_male: true,
        }).send().await.map(|_| true).unwrap_or(false)
    }
}

struct Resolver;
impl Resolver {
    async fn generate_random() -> Res<Speech> {
        Ok(Speech {
            id: "1".to_owned(),
            text: "hello world".to_owned(),
        })
    }

    async fn upload_audio(text: &str, uploader: &Uploader) -> Res<String> {
        uploader.upload(&SpeechRequest {
            text: text.to_owned(),
            is_male: true,
        }).await
    }
}

struct QueryRoot;

#[Object]
impl QueryRoot {
    async fn random(&self) -> Res<Speech> {
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

    async fn audio_url<'ctx>(&self, ctx: &Context<'ctx>) -> Res<String> {
        let uploader = ctx.data_unchecked::<Uploader>();
        let text = &self.text;
        Resolver::upload_audio(text, uploader).await
    }
}

async fn health(uploader: web::Data<Uploader>) -> Result<HttpResponse> {
    if uploader.is_healthy().await {
        Ok(HttpResponse::Ok().content_type("application/json").body(""))
    } else {
        Ok(HttpResponse::InternalServerError().finish())
    }
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
    let client = Client::new();
    let uploader = Uploader::new(client);

    let schema = Schema::build(QueryRoot, EmptyMutation, EmptySubscription)
        .data(uploader.clone())
        .finish();

    println!("Playground: http://localhost:8000");

    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(schema.clone()))
            .app_data(Data::new(uploader.clone()))
            .service(web::resource("/health").guard(guard::Get()).to(health))
            .service(web::resource("/").guard(guard::Post()).to(index))
            .service(web::resource("/").guard(guard::Get()).to(index_playground))
    })
        .bind("0.0.0.0:8000")?
        .run()
        .await
}
