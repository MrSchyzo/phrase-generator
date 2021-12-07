use serde::Serialize;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub(in super::super) struct SpeechRequest {
    text: String,
    is_male: bool,
}

impl From<Speech> for SpeechRequest {
    fn from(this: Speech) -> Self {
        Self {
            text: this.text,
            is_male: this.is_male,
        }
    }
}

impl Default for SpeechRequest {
    fn default() -> Self {
        Self {
            text: "Ciao".to_owned(),
            is_male: true,
        }
    }
}

#[derive(Clone)]
pub struct Speech {
    pub text: String,
    pub is_male: bool,
}

impl From<crate::app_core::types::upload::Speech> for Speech {
    fn from(speech: crate::app_core::types::upload::Speech) -> Self {
        Self {
            text: speech.text,
            is_male: speech.is_male,
        }
    }
}
