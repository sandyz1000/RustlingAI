use yewdux::prelude::*;

use crate::{components::toast::ToastStatus, constants::DEFAULT_SYSTEM_MESSAGE, types::{
    api::SyncStatus,
    chat::{ChatInterface, ConfigInterface, FolderCollection, MessageInterface, Prompt, Role, Theme, TotalTokenUsed},
}};

#[derive(Debug, Store, Default, Clone, PartialEq)]
pub(crate) struct PromptSlice {
    pub(crate) prompts: Vec<Prompt>
}

#[derive(Debug, Store, Default, Clone, PartialEq)]
pub(crate) struct ToastSlice {
    pub(crate) show: bool,
    pub(crate) message: String,
    pub(crate) status: ToastStatus,
}

impl ToastSlice {
    pub fn new(show: bool, message: Option<String>, status: ToastStatus) -> Self {
        Self {
            show,
            message: message.unwrap(),
            status,
        }
    }
}

#[derive(Debug, Default, Clone, PartialEq, Store)]
pub(crate) struct AuthSlice {
    pub(crate) api_key: Option<String>,
    pub(crate) api_endpoint: Option<String>,
    pub(crate) first_version: bool,
}

impl AuthSlice {
    pub fn new(api_key: Option<String>, api_endpoint: Option<String>, first_version: bool) -> Self {
        Self {
            api_key,
            api_endpoint,
            first_version,
        }
    }
}

#[derive(Debug, Default, Clone, PartialEq, Store)]
pub(crate) struct ChatSlice {
    pub(crate) messages: Vec<MessageInterface>,
    pub(crate) chats: Vec<ChatInterface>,
    pub(crate) curr_chat_index: i32,
    pub(crate) generating: bool,
    pub(crate) error: String,
    pub(crate) folders: FolderCollection,
}

#[derive(Debug)]
pub(crate) struct CloudAuthSlice {
    pub(crate) google_access_token: Option<String>,
    pub(crate) google_refresh_token: Option<String>,
    pub(crate) cloud_sync: bool,
    pub(crate) sync_status: SyncStatus,
    pub(crate) file_id: Option<String>,
}

pub type StoreState = CloudAuthSlice;

#[derive(Debug, Clone, PartialEq, Store)]
pub(crate) struct ConfigSlice {
    pub(crate) open_config: bool,
    pub(crate) theme: Theme,
    pub(crate) auto_title: bool,
    pub(crate) hide_menu_options: bool,
    pub(crate) advanced_mode: bool,
    pub(crate) default_chat_config: ConfigInterface,
    pub(crate) default_system_message: String,
    pub(crate) hide_side_menu: bool,
    pub(crate) enter_to_submit: bool,
    pub(crate) inline_latex: bool,
    pub(crate) markdown_mode: bool,
    pub(crate) count_total_tokens: bool,
    pub(crate) total_token_used: TotalTokenUsed,
}

impl Default for ConfigSlice {
    fn default() -> Self {
        Self {
            open_config: false,
            theme: Theme::Dark,
            hide_menu_options: false,
            hide_side_menu: false,
            auto_title: false,
            enter_to_submit: true,
            advanced_mode: true,
            default_chat_config: ConfigInterface::default(),
            default_system_message: DEFAULT_SYSTEM_MESSAGE.to_string(),
            inline_latex: false,
            markdown_mode: true,
            count_total_tokens: false,
            total_token_used: TotalTokenUsed::new(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Store)]
pub struct InputSlice {
    pub input_role: Role
}

impl Default for InputSlice {
    fn default() -> Self {
        Self { input_role: Role::User }
    }
}
