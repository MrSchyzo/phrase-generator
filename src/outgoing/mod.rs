mod types;

use async_graphql::{EmptyMutation, EmptySubscription, Schema};

type AppSchema = Schema<types::QueryRoot, EmptyMutation, EmptySubscription>;
