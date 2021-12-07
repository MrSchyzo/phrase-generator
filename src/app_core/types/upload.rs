use reqwest::Url;

#[derive(Clone)]
pub struct Speech {
    pub text: String,
    pub is_male: bool,
}

#[derive(Clone)]
pub struct UploadedSpeech {
    pub url: Url,
}

impl From<crate::outgoing::tts_wrapper::types::UploadResult> for UploadedSpeech {
    fn from(result: crate::outgoing::tts_wrapper::types::UploadResult) -> Self {
        Self { url: result.url }
    }
}
