pub mod types;

use async_graphql::{EmptyMutation, EmptySubscription, Schema};

pub type AppSchema = Schema<types::QueryRoot, EmptyMutation, EmptySubscription>;
