use std::collections::HashMap;

use gloo_utils::format::JsValueSerdeExt;
use serde::Serialize;
use wasm_bindgen::{JsCast, JsValue};
use web_sys::{Blob, BlobPropertyBag, Url};

use crate::types::{chat::{ChatInterface, ConfigInterface, MessageInterface}, export::{OpenAIChat, OpenAIChatNode}};

pub fn get_today() -> String {
    chrono::Local::now().format("%Y-%m-%d").to_string()
}

pub fn download_file(data: &impl Serialize, filename: String) {
    // Serialize data to JSON
    let js_value: JsValue = JsValue::from_serde(&data).unwrap();

    // Create Blob
    let option = BlobPropertyBag::new();
    option.set_type("application/json");
    let blob =
        Blob::new_with_str_sequence_and_options(&js_value, &option).expect("Failed to create blob");

    // Create object URL
    let url = Url::create_object_url_with_blob(&blob).unwrap();

    // Create and click temporary anchor
    let document = gloo_utils::window().document().unwrap();
    let anchor = document
        .create_element("a")
        .unwrap()
        .dyn_into::<web_sys::HtmlAnchorElement>()
        .unwrap();

    anchor.set_href(&url);
    anchor.set_download(&filename);
    anchor.click();

    // Cleanup
    Url::revoke_object_url(&url).unwrap();
}


// Conversion implementation
pub fn convert_openai_to_my_format(openai_chat: &OpenAIChat) -> ChatInterface {
    let mut messages = Vec::new();
    
    // Recursive traversal function
    fn traverse_tree(
        mapping: &HashMap<String, OpenAIChatNode>,
        current_id: &str,
        messages: &mut Vec<MessageInterface>,
    ) {
        if let Some(node) = mapping.get(current_id) {
            // Process current node
            if let Some(msg) = &node.message {
                let content = msg.content.parts.join("");
                if !content.is_empty() {
                    messages.push(MessageInterface {
                        role: msg.author.role.clone(),
                        content,
                        folder: None,
                        messages: vec![],
                    });
                }
            }

            // Process last child if exists
            if let Some(last_child) = node.children.last() {
                traverse_tree(mapping, last_child, messages);
            }
        }
    }

    // Find root node (first node in mapping)
    if let Some(root_id) = openai_chat.mapping.keys().next() {
        if let Some(root_node) = openai_chat.mapping.get(root_id) {
            traverse_tree(&openai_chat.mapping, &root_node.id, &mut messages);
        }
    }

    ChatInterface {
        id: uuid::Uuid::new_v4().to_string(),
        title: Some(openai_chat.title.clone()),
        messages,
        config: Some(ConfigInterface::default()), // Implement your default config
        title_set: true,
        folder: None,
    }
}

// Batch conversion function
pub fn import_openai_chat_export(openai_chats: Vec<OpenAIChat>) -> Vec<ChatInterface> {
    openai_chats
        .into_iter()
        .map(|chat| convert_openai_to_my_format(&chat))
        .collect()
}
