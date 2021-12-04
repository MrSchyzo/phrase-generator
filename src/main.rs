mod app_core;
mod served;

use crate::app_core::{AppCore, Uploader};
use std::sync::Arc;

use actix_web::web::{self, Data};
use actix_web::{guard, App, HttpResponse, HttpServer, Result};
use async_graphql::http::{playground_source, GraphQLPlaygroundConfig};
use async_graphql::{EmptyMutation, EmptySubscription, Schema};
use async_graphql_actix_web::{GraphQLRequest, GraphQLResponse};
use reqwest::Client;
use served::types::QueryRoot;
use served::AppSchema;

async fn health(core: Data<Arc<AppCore>>) -> Result<HttpResponse> {
    match core.is_healthy().await {
        Ok(_) => Ok(HttpResponse::Ok().content_type("application/json").body("")),
        Err(error) => Ok(HttpResponse::InternalServerError().body(format!("{}", error))),
    }
}

async fn life() -> Result<HttpResponse> {
    Ok(HttpResponse::Ok().content_type("application/json").body(""))
}

async fn index(schema: Data<AppSchema>, req: GraphQLRequest) -> GraphQLResponse {
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
