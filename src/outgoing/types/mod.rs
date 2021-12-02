use std::sync::Arc;
use async_graphql::Context;

pub(crate) struct QueryRoot;

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
