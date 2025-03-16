use std::sync::Arc;

use yew::prelude::*;

#[hook]
pub(crate) fn use_add_chat() -> Arc<dyn Fn(Option<String>) + Send + Sync> {
    let add_chat = |folder: Option<String>| {
        todo!()
    };
    Arc::new(add_chat)
}
