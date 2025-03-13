use serde::{Serialize, Deserialize};

pub(crate) const OFFICIAL_APIENDPOINT: &str = "https://api.openai.com/v1/chat/completions";
pub(crate) const CUSTOM_APIENDPOINT: &str = "https://chatgpt-api.shn.hk/v1/";
// pub const defaultAPIEndpoint = VITE_DEFAULT_API_ENDPOINT || officialAPIEndpoint;


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GoogleFileResource {
    pub kind: String,
    pub id: String,
    pub name: String,
    pub mime_type: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GoogleTokenInfo {
    pub azp: String,
    pub aud: String,
    pub sub: String,
    pub scope: String,
    pub exp: String,
    pub expires_in: String,
    pub email: String,
    pub email_verified: String,
    pub access_type: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GoogleFileList {
    pub next_page_token: Option<String>,
    pub kind: String,
    pub incomplete_search: bool,
    pub files: Vec<GoogleFileResource>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum SyncStatus {
    Unauthenticated,
    Syncing,
    Synced,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EventSourceDataInterface {
    pub choices: Vec<EventSourceDataChoices>,
    pub created: u64,
    pub id: String,
    pub model: String,
    pub object: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum EventSourceData {
    Data(EventSourceDataInterface),
    Done(String),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EventSourceDataChoices {
    pub delta: Delta,
    pub finish_reason: Option<String>,
    pub index: u32,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Delta {
    pub content: Option<String>,
    pub role: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ShareGPTSubmitBodyInterface {
    pub avatar_url: String,
    pub items: Vec<ShareGPTItem>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ShareGPTItem {
    pub from: FromType,
    pub value: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum FromType {
    Gpt,
    Human,
}

