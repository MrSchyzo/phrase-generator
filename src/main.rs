mod app_core;
mod outgoing;
mod served;
pub mod utils;

use crate::app_core::{AppCore, PhraseGenerator, Uploader};
use std::sync::Arc;

use actix_web::web::{self, Data};
use actix_web::{guard, App, HttpServer};

use async_graphql::{EmptyMutation, EmptySubscription, Schema};

use reqwest::Client;
use served::types::graphql::QueryRoot;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let uploader = Uploader::new(Client::new());
    let generator = PhraseGenerator {};
    let core = Arc::new(AppCore::new(Arc::new(uploader), Arc::new(generator)));

    let schema = Schema::build(QueryRoot, EmptyMutation, EmptySubscription)
        .data(core.clone()) //For GQL field async resolvers through Context
        .finish();

    tracing_subscriber::fmt::init();

    println!("Playground: http://localhost:8000");

    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(schema.clone())) //For routes through Data<>
            .app_data(Data::new(core.clone())) //For routes through Data<>
            .service(
                web::resource("/health")
                    .guard(guard::Get())
                    .to(served::health),
            )
            .service(web::resource("/life").guard(guard::Get()).to(served::life))
            .service(web::resource("/").guard(guard::Post()).to(served::index))
            .service(
                web::resource("/")
                    .guard(guard::Get())
                    .to(served::index_playground),
            )
    })
    .bind("0.0.0.0:8000")?
    .run()
    .await
}
