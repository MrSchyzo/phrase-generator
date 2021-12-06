use async_graphql::{Context, Enum, InputObject, Object};

use std::sync::Arc;

use crate::app_core::{AppCore, AppResult, SpeechGenerationOptions, SpeechToUpload};

pub struct QueryRoot;

#[derive(InputObject)]
pub struct SpeechGenerationOpts {
    pub category: String,
}

#[derive(InputObject)]
pub struct Voice {
    pub language: Language,
    pub gender: Gender,
}

#[derive(Enum, Copy, Clone, Eq, PartialEq)]
pub enum Language {
    Italian,
}

#[derive(Enum, Copy, Clone, Eq, PartialEq)]
pub enum Gender {
    Male,
    Female,
}

#[Object]
impl QueryRoot {
    async fn random<'ctx>(
        &self,
        ctx: &Context<'ctx>,
        _opts: SpeechGenerationOpts,
    ) -> AppResult<Speech> {
        let generator = ctx.data_unchecked::<Arc<AppCore>>().generator();
        generator.generate(SpeechGenerationOptions {}).await
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

    pub async fn audio_url<'c>(&self, ctx: &Context<'c>, voice: Voice) -> AppResult<String> {
        let uploader = ctx.data_unchecked::<Arc<AppCore>>().uploader();
        let text = &self.text;
        uploader
            .upload(&SpeechToUpload {
                is_male: matches!(voice.gender, Gender::Male),
                text: text.clone(),
            })
            .await
            .map(|res| res.url().as_str().to_owned())
    }
}
