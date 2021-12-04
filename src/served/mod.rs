pub mod types;

use crate::app_core::AppCore;
use std::sync::Arc;

use actix_web::web::Data;
use actix_web::{HttpResponse, Result};
use async_graphql::http::{playground_source, GraphQLPlaygroundConfig};
use async_graphql::{EmptyMutation, EmptySubscription, Schema};
use async_graphql_actix_web::{GraphQLRequest, GraphQLResponse};

pub type AppSchema = Schema<types::graphql::QueryRoot, EmptyMutation, EmptySubscription>;

pub async fn health(core: Data<Arc<AppCore>>) -> Result<HttpResponse> {
    match core.is_healthy().await {
        Ok(_) => Ok(HttpResponse::Ok().content_type("application/json").body("")),
        Err(error) => Ok(HttpResponse::InternalServerError().body(format!("{}", error))),
    }
}

pub async fn life() -> Result<HttpResponse> {
    Ok(HttpResponse::Ok().content_type("application/json").body(""))
}

pub async fn index(schema: Data<AppSchema>, req: GraphQLRequest) -> GraphQLResponse {
    schema.execute(req.into_inner()).await.into()
}

pub async fn index_playground() -> Result<HttpResponse> {
    let source = playground_source(GraphQLPlaygroundConfig::new("/").subscription_endpoint("/"));
    Ok(HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(source))
}
