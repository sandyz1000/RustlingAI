use yew::prelude::*;
use std::sync::Arc;

pub struct Namespace {
    pub ns: String
}

#[hook]
pub fn use_translation(pages: Vec<String>) -> Arc<dyn Fn(String, Option<Namespace>) -> String + Send + Sync> {

    let translator = move |key: String, _namespace: Option<Namespace>| -> String {
        format!("Translated: {}", key) // Replace with actual translation logic
    };

    Arc::new(translator)
}
