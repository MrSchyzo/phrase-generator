use async_graphql::{Context, Enum, InputObject, Object};

use std::sync::Arc;

use crate::app_core::errors::AppError;
use crate::app_core::{AppCore, AppResult, SpeechGenerationOptions};

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

#[derive(Enum, Copy, Clone, Eq, PartialEq, sqlx::Type)]
#[sqlx(type_name = "lang")] // May also be the name of a user defined enum type
#[sqlx(rename_all = "lowercase")] // similar to serde rename_all
pub enum Language {
    Ita,
}

impl ToString for Language {
    fn to_string(&self) -> String {
        match self {
            Language::Ita => "ita".to_string(),
        }
    }
}

#[derive(Enum, Copy, Clone, Eq, PartialEq, sqlx::Type)]
#[sqlx(type_name = "gender")] // May also be the name of a user defined enum type
#[sqlx(rename_all = "lowercase")] // similar to serde rename_all
pub enum Gender {
    Male,
    Female,
}

impl ToString for Gender {
    fn to_string(&self) -> String {
        match self {
            Gender::Male => "male".to_owned(),
            Gender::Female => "female".to_owned(),
        }
    }
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

    //TODO: refactor this crap
    pub async fn audio_url<'c>(&self, ctx: &Context<'c>, voice: Voice) -> AppResult<String> {
        let text = &self.text;
        let id =
            sqlx::types::Uuid::parse_str(&self.id).map_err(AppError::for_upload_in_sql_uuid)?;
        let uploader = ctx.data_unchecked::<Arc<AppCore>>().uploader();
        let mut transaction = ctx
            .data_unchecked::<Arc<AppCore>>()
            .pool()
            .begin()
            .await
            .map_err(AppError::for_upload_in_sql)?;

        // See: https://docs.rs/sqlx/0.4.2/sqlx/macro.query.html#type-overrides-bind-parameters-postgres-only
        if let Some(url) = sqlx::query!(
            "SELECT url FROM generated_phrase_speech WHERE generated_phrase = $1 AND lang = $2 AND gender = $3",
            id as sqlx::types::Uuid, voice.language as _, voice.gender as _
        )
        .fetch_optional(&mut transaction)
        .await
        .map_err(AppError::for_upload_in_sql)?
            .map(|res| res.url) {
            Ok(url)
        } else {
            let url = uploader
                .upload(crate::app_core::types::upload::Speech {
                    is_male: matches!(voice.gender, Gender::Male),
                    text: text.clone(),
                })
                .await
                .map(|res| res.url.to_string())?;

            sqlx::query!(
                "INSERT INTO generated_phrase_speech (generated_phrase, lang, gender, url) VALUES ($1, $2, $3, $4)",
                id as _, voice.language as _, voice.gender as _, &url as _
            ).execute(&mut transaction)
                .await
                .map_err(AppError::for_upload_in_sql)?;

            transaction.commit().await.map_err(AppError::for_upload_in_sql)?;

            Ok(url)
        }
    }
}
