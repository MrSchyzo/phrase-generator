mod app_core;
mod outgoing;
mod served;
pub mod utils;

use crate::app_core::{AppCore, PhraseGenerator, Uploader};
use std::str::FromStr;
use std::sync::Arc;

use actix_web::web::{self, Data};
use actix_web::{guard, App, HttpServer};

use async_graphql::{EmptyMutation, EmptySubscription, Schema};

use crate::outgoing::tts_wrapper::{SimpleTtsWrapperClient, TtsWrapperConnectionOpts};
use reqwest::{Client, Url};
use served::types::graphql::QueryRoot;
use sqlx::postgres::PgPoolOptions;
use sqlx::FromRow;

use crate::app_core::engine::types::{PlaceholderReference, ProductionBranch};
use tracing::info;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    tracing_subscriber::fmt::init();

    let tts_wrapper_root =
        std::env::var("TTS_WRAPPER_URL").unwrap_or_else(|_| "http://localhost:8080".to_owned());

    info!("Connecting to TTS wrapper at: {}", tts_wrapper_root);

    foo().await.unwrap();

    let uploader = Uploader::new(Arc::new(SimpleTtsWrapperClient::new(
        Client::new(),
        TtsWrapperConnectionOpts {
            root_url: Url::parse(&tts_wrapper_root).unwrap(),
        },
    )));
    let generator = PhraseGenerator {};
    let core = Arc::new(AppCore::new(Arc::new(uploader), Arc::new(generator)));

    let schema = Schema::build(QueryRoot, EmptyMutation, EmptySubscription)
        .data(core.clone()) //For GQL field async resolvers through Context
        .finish();

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

// https://docs.rs/sqlx/0.5.9/sqlx/macro.query.html#force-not-null
#[derive(FromRow)]
struct WordSemantic {
    id: i32,
    content: String,
    non_repeatable: bool,
    semantic_id: i32,
    semantic_tag: String,
}

async fn foo() -> Result<(), Box<dyn std::error::Error>> {
    use futures::TryStreamExt;

    let pool = PgPoolOptions::new()
        .max_connections(8)
        .connect("postgres://postgres:password@localhost:49153/postgres")
        .await?;

    let sem: Vec<i32> = vec![1];
    let query_template = format!("
        SELECT
            w.id as id, w.content as content, w.non_repeatable as non_repeatable, s.id as semantic_id, s.name as semantic_tag
        FROM
            word as w
        INNER JOIN
            word_semantic as ws ON w.id = ws.word
        INNER JOIN
            semantic_tag as s ON s.id = ws.semantic_tag
        WHERE
            s.id IN ({})
        ", (0..sem.len()).map(|i| format!("${}", i + 1)).collect::<Vec<_>>().join(","));
    let mut query = sqlx::query_as::<sqlx::Postgres, WordSemantic>(&query_template);

    for sem_tag in sem.iter() {
        query = query.bind(sem_tag);
    }

    let mut rows = query.fetch(&pool);

    while let Some(word) = rows.try_next().await? {
        println!(
            "{}: {} (non_repeatable? {}). {}, {}",
            word.id, word.content, word.non_repeatable, word.semantic_id, word.semantic_tag
        );
    }
    let query_template = "
        select p.production
        from production p
        inner join non_terminal_symbol nts
        on nts.id = p.non_terminal_symbol and nts.name = $1
        order by nts_amount desc
        limit 1;
    "
    .to_string();
    let query = sqlx::query_as(&query_template).bind("Start");

    let (row,): (String,) = query.fetch_one(&pool).await?;

    println!("{row}");

    let branch = ProductionBranch::from_str(&row)?;

    for reference in branch.ordered_placeholder_references()?.iter() {
        match reference {
            PlaceholderReference::NonTerminalSymbol(nts_ref) => {
                println!(
                    "Grammar on nothing, {}; {}; can propagate? {}",
                    nts_ref.id(),
                    nts_ref.reference(),
                    nts_ref.grammar_can_propagate()
                );
            }
            PlaceholderReference::WordSelector(_) => {
                println!("Word!")
            }
        }
    }

    Ok(())
}
