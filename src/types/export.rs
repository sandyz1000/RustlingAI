
use std::collections::HashMap;
use serde::{Serialize, Deserialize};

use super::chat::{ChatInterface, FolderCollection, Role};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ExportBase {
    pub version: u32,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ExportV1 {
    #[serde(flatten)]
    pub base: ExportBase,
    pub chats: Option<Vec<ChatInterface>>,
    pub folders: FolderCollection,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OpenAIChat {
    pub title: String,
    pub mapping: HashMap<String, OpenAIChatNode>,
    pub current_node: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OpenAIChatNode {
    pub id: String,
    pub message: Option<OpenAIChatMessage>,
    pub parent: Option<String>,
    pub children: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OpenAIChatMessage {
    pub author: OpenAIChatAuthor,
    pub content: OpenAIChatContent,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OpenAIChatAuthor {
    pub role: Role,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OpenAIChatContent {
    pub parts: Option<Vec<String>>,
}
