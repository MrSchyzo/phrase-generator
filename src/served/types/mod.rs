use async_graphql::{Context, Object};
use std::sync::Arc;

use crate::app_core::{AppCore, AppResult, Resolver};

pub struct QueryRoot;

#[Object]
impl QueryRoot {
    async fn random(&self) -> AppResult<Speech> {
        Resolver::generate_random().await
    }
}

pub struct Speech {
    pub id: String,
    pub text: String,
}

#[Object]
impl Speech {
    pub async fn id(&self) -> &str {
        &self.id
    }

    pub async fn text(&self) -> &str {
        &self.text
    }

    pub async fn audio_url<'ctx>(&self, ctx: &Context<'ctx>) -> AppResult<String> {
        let uploader = ctx.data_unchecked::<Arc<AppCore>>().uploader();
        let text = &self.text;
        Resolver::upload_audio(text, uploader)
            .await
            .map(|res| res.url().as_str().to_owned())
    }
}
