use serde::{Deserialize, Serialize};
use std::{collections::HashMap, path::Display};
use yewdux::prelude::*;
use super::api::OFFICIAL_APIENDPOINT;
pub const DEFAULT_USER_MAX_TOKEN: i32 = 4000;

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub enum Role {
    #[default]
    User,
    Assistant,
    System,
}
impl std::fmt::Display for Role {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Role::User => "user",
            Role::Assistant => "assistant",
            Role::System => "system",
        }.fmt(f)
    }
}

impl std::str::FromStr for Role {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "user" => Ok(Role::User),
            "assistant" => Ok(Role::Assistant),
            "system" => Ok(Role::System),
            _ => Err(()),
        }
    }
}

#[derive(Debug, Clone, Default, Store, PartialEq, Serialize, Deserialize)]
pub(crate) struct MessageInterface {
    pub(crate) role: Role,
    pub(crate) content: String,
    pub(crate) folder: Option<String>,
    pub(crate) messages: Vec<MessageInterface>,
}

#[derive(Debug, Clone, Default, Store, PartialEq, Serialize, Deserialize)]
pub struct ChatInterface {
    pub(crate) id: String,
    pub(crate) title: Option<String>,
    pub(crate) folder: Option<String>,
    pub(crate) messages: Vec<MessageInterface>,
    pub(crate) config: Option<ConfigInterface>,
    /// Indicates whether the title has been set.
    ///
    /// This boolean flag is used to track if the title has been assigned a value.
    pub(crate) title_set: bool,
}

impl ChatInterface {
    pub fn new(
        title: Option<String>,
        folder: Option<String>,
        mut messages: Vec<MessageInterface>,
        config: Option<ConfigInterface>,
        default_sys_msg: String,
    ) -> Self {
        let title = match title {
            Some(t) => t,
            None => "New Chat".to_string(),
        };
        if messages.is_empty() {
            messages = vec![MessageInterface {
                role: Role::System,
                content: default_sys_msg,
                folder: None,
                messages: vec![],
            }]
        }

        Self {
            id: "".to_string(), // Generate new uuid
            title: Some(title),
            folder,
            messages,
            config,
            title_set: false,
        }
    }
}

#[derive(Debug, Clone, Store, PartialEq, Serialize, Deserialize)]
pub(crate) struct ConfigInterface {
    pub(crate) model: ModelOptions,
    pub(crate) max_tokens: i32,
    pub(crate) temperature: i32,
    pub(crate) presence_penalty: i32,
    pub(crate) top_p: i32,
    pub(crate) frequency_penalty: i32,
}

impl Default for ConfigInterface {
    fn default() -> Self {
        Self {
            model: ModelOptions::Gpt35Turbo,
            max_tokens: DEFAULT_USER_MAX_TOKEN,
            temperature: 1,
            presence_penalty: 0,
            top_p: 1,
            frequency_penalty: 0,
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct ChatHistoryInterface {
    pub(crate) title: String,
    pub(crate) index: i32,
    pub(crate) id: String,
}

pub type ChatHistoryFolderInterface = HashMap<String, Vec<ChatHistoryInterface>>;

pub type FolderCollection = HashMap<String, Folder>;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Folder {
    pub(crate) id: String,
    pub(crate) name: String,
    pub(crate) expanded: bool,
    pub(crate) order: i32,
    pub(crate) color: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ModelOptions {
    Gpt4o,
    Gpt4o20240513,
    Gpt4,
    Gpt432k,
    Gpt41106Preview,
    Gpt40125Preview,
    Gpt4Turbo,
    Gpt4Turbo20240409,
    Gpt35Turbo,
    Gpt35Turbo16k,
    Gpt35Turbo1106,
    Gpt35Turbo0125,
}

impl std::str::FromStr for ModelOptions {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "gpt-4o" => Ok(Self::Gpt4o),
            "gpt-4o-2024-05-13" => Ok(Self::Gpt4o20240513),
            "gpt-4" => Ok(Self::Gpt4),
            "gpt-4-32k" => Ok(Self::Gpt432k),
            "gpt-4-1106-preview" => Ok(Self::Gpt41106Preview),
            "gpt-4-0125-preview" => Ok(Self::Gpt40125Preview),
            "gpt-4-turbo" => Ok(Self::Gpt4Turbo),
            "gpt-4-turbo-2024-04-09" => Ok(Self::Gpt4Turbo20240409),
            "gpt-3.5-turbo" => Ok(Self::Gpt35Turbo),
            "gpt-3.5-turbo-16k" => Ok(Self::Gpt35Turbo16k),
            "gpt-3.5-turbo-1106" => Ok(Self::Gpt35Turbo1106),
            "gpt-3.5-turbo-0125" => Ok(Self::Gpt35Turbo0125),
            _ => Err(()),
        }
    }
}

// impl Iterator for ModelOptions {
//     type Item = Self;

//     fn next(&mut self) -> Option<Self> {
//         todo!()
//     }
// }

impl std::fmt::Display for ModelOptions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Self::Gpt4o => "gpt-4o",
            Self::Gpt4o20240513 => "gpt-4o-2024-05-13",
            Self::Gpt4 => "gpt-4",
            Self::Gpt432k => "gpt-4-32k",
            Self::Gpt41106Preview => "gpt-4-1106-preview",
            Self::Gpt40125Preview => "gpt-4-0125-preview",
            Self::Gpt4Turbo => "gpt-4-turbo",
            Self::Gpt4Turbo20240409 => "gpt-4-turbo-2024-04-09",
            Self::Gpt35Turbo => "gpt-3.5-turbo",
            Self::Gpt35Turbo16k => "gpt-3.5-turbo-16k",
            Self::Gpt35Turbo1106 => "gpt-3.5-turbo-1106",
            Self::Gpt35Turbo0125 => "gpt-3.5-turbo-0125",
        };
        write!(f, "{}", s)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct TokenUsage {
    pub prompt_tokens: u32,
    pub completion_tokens: u32,
}

pub type TotalTokenUsed = HashMap<ModelOptions, TokenUsage>;

#[derive(Debug, Clone, PartialEq)]
pub struct LocalStorageInterfaceV0ToV1 {
    pub chats: Vec<ChatInterface>,
    pub current_chat_index: usize,
    pub api_key: String,
    pub api_free: bool,
    pub api_free_endpoint: String,
    pub theme: Theme,
}

impl LocalStorageInterfaceV0ToV1 {
    pub fn migrate_v0(other: &Self) -> Self {
        let chats = other
            .chats
            .iter()
            .map(|c| {
                let chat_config = match &c.config {
                    Some(cfg) => cfg.clone(),
                    None => ConfigInterface::default(),
                };
                let new_chat = ChatInterface {
                    config: Some(chat_config),
                    title_set: false,
                    id: c.id.clone(),
                    title: c.title.clone(),
                    folder: c.folder.clone(),
                    messages: c.messages.clone(),
                };
                new_chat
            })
            .collect();
        Self {
            chats,
            ..other.clone()
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct LocalStorageInterfaceV1ToV2 {
    pub chats: Vec<ChatInterface>,
    pub current_chat_index: usize,
    pub api_key: String,
    pub api_free: bool,
    pub api_free_endpoint: String,
    pub api_endpoint: Option<String>,
    pub theme: Theme,
}

impl LocalStorageInterfaceV1ToV2 {
    pub fn migrate_v1(other: &Self) -> Self {
        let api_endpoint = Some(if other.api_free {
            other.api_free_endpoint.clone()
        } else {
            OFFICIAL_APIENDPOINT.to_string()
        });
        Self {
            api_endpoint,
            ..other.clone()
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct LocalStorageInterfaceV2ToV3 {
    pub chats: Vec<ChatInterface>,
    pub current_chat_index: usize,
    pub api_key: String,
    pub api_free: bool,
    pub api_free_endpoint: String,
    pub api_endpoint: Option<String>,
    pub theme: Theme,
    pub auto_title: bool,
}

impl LocalStorageInterfaceV2ToV3 {
    pub fn migrate_v2(other: &Self) -> Self {
        let chats = other
            .chats
            .iter()
            .map(|chat| {
                let _c = chat.config.as_ref().unwrap().clone();
                let config = Some(ConfigInterface {
                    top_p: ConfigInterface::default().top_p,
                    frequency_penalty: ConfigInterface::default().frequency_penalty,
                    .._c
                });
                let new_chat = ChatInterface {
                    config,
                    ..chat.clone()
                };
                new_chat
            })
            .collect();
        Self {
            chats,
            auto_title: false,
            ..other.clone()
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct LocalStorageInterfaceV3ToV4 {
    pub chats: Vec<ChatInterface>,
    pub current_chat_index: usize,
    pub api_key: String,
    pub api_free: bool,
    pub api_free_endpoint: String,
    pub api_endpoint: Option<String>,
    pub theme: Theme,
    pub auto_title: bool,
    pub prompts: Vec<Prompt>,
}

impl LocalStorageInterfaceV3ToV4 {
    pub fn migrate_v3(other: &Self) -> Self {
        Self {
            prompts: vec![],
            ..other.clone()
        }
    }
}

pub type LocalStorageInterfaceV4ToV5 = LocalStorageInterfaceV3ToV4;
pub type LocalStorageInterfaceV5ToV6 = LocalStorageInterfaceV3ToV4;

#[derive(Debug, Clone, PartialEq)]
pub struct LocalStorageInterfaceV6ToV7 {
    pub chats: Vec<ChatInterface>,
    pub current_chat_index: usize,
    pub api_free: Option<bool>,
    pub api_key: String,
    pub api_endpoint: String,
    pub theme: Theme,
    pub auto_title: bool,
    pub prompts: Vec<Prompt>,
    pub default_chat_config: ConfigInterface,
    pub default_system_message: String,
    pub hide_menu_options: bool,
    pub first_visit: bool,
    pub hide_side_menu: bool,
}

#[derive(Debug, Clone, PartialEq)]
pub struct LocalStorageInterfaceV7ToV8 {
    pub chats: Vec<ChatInterface>,
    pub current_chat_index: usize,
    pub api_free: Option<bool>,
    pub api_key: String,
    pub api_endpoint: String,
    pub theme: Theme,
    pub auto_title: bool,
    pub prompts: Vec<Prompt>,
    pub default_chat_config: ConfigInterface,
    pub default_system_message: String,
    pub hide_menu_options: bool,
    pub first_visit: bool,
    pub hide_side_menu: bool,
    pub folders_name: Vec<String>,
    pub folders_expanded: Vec<bool>,
    pub folders: FolderCollection,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Prompt {
    pub(crate) id: String,
    pub(crate) name: String,
    pub(crate) prompt: String,
}

impl Default for Prompt {
    fn default() -> Self {
        Self {
            id: "0d3e9cb7-b585-43fa-acc3-840c189f6b93".to_string(),
            name: "English Translator".to_string(),
            prompt: "I want you to act as an English translator, spelling corrector and improver. I will speak to you in any language and you will detect the language, translate it and answer in the corrected and improved version of my text, in English. I want you to replace my simplified A0-level words and sentences with more beautiful and elegant, upper level English words and sentences. Keep the meaning same, but make them more literary. I want you to only reply the correction, the improvements and nothing else, do not write explanations. Do you understand?".to_string(),
        }
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub enum Theme {
    #[default]
    Light,
    Dark,
}

impl std::fmt::Display for Theme {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Theme::Light => write!(f, "light"),
            Theme::Dark => write!(f, "dark"),
        }
    }
}
