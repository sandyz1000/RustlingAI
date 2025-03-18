mod api;
mod components;
mod google_api;
mod hooks;
mod store;
mod types;
mod utils;
mod constants;

use components::{api_popup::ApiPopup, chat::Chat, menu::Menu, toast::Toast};
use gloo::events::EventListener;
use gloo_storage::{LocalStorage, Storage};
use hooks::initialise_chat::use_initialise_chat;
use store::{AuthSlice, ChatSlice, ConfigSlice};
use types::chat::ChatInterface;
use wasm_bindgen::JsCast;
use yew::prelude::*;
use yewdux::use_store;


#[function_component]
pub fn App() -> Html {
    let init_new_chat = use_initialise_chat();
    let (chat_store, chat_dispatch) = use_store::<ChatSlice>();
    let (_, config_dispatch) = use_store::<ConfigSlice>();
    let (_, auth_dispatch) = use_store::<AuthSlice>();

    {
        // Replace with dynamic i18n.language value
        // Dispathing an event from the JavaScript
        // 
        // ```js
        // document.documentElement.dispatchEvent(event);
        // const event = new CustomEvent("languageChanged", { detail: "fr" });
        // ```
        use_effect_with("en".to_string(),
            |language| {
                let html_element = gloo_utils::document().document_element().unwrap();
    
                // Set the initial language
                let _ = html_element.set_attribute("lang", language);
    
                // Create an event listener for language changes
                let listener = EventListener::new(&html_element, "languageChanged", {
                    let html_element = html_element.clone();
                    move |event| {
                        if let Some(lang) = event.dyn_ref::<web_sys::CustomEvent>().and_then(|e| e.detail().as_string()) {
                            let _ = html_element.set_attribute("lang", &lang);
                        }
                    }
                });
    
                // Cleanup: Drop the listener when the component unmounts
                || drop(listener)
            },
        );
    }
    {
        let dispatch = auth_dispatch.clone();
        let init_new_chat = init_new_chat.clone();
        use_effect(move || {

            // Handle legacy localStorage migration
            if let Ok(Some(api_key)) = LocalStorage::get("apiKey") {
                dispatch.reduce_mut(|s| s.api_key = api_key);
                LocalStorage::delete("apiKey");
            }
    
            if let Ok(Some(theme)) = LocalStorage::get("theme") {
                config_dispatch.reduce_mut(|s| s.theme = theme);
                LocalStorage::delete("theme");
            }
    
            match LocalStorage::get::<Option<String>>("chats") {
                Ok(Some(old_chats)) => {
                    let old_chats = serde_json::from_str::<Vec<ChatInterface>>(&old_chats);
                    match old_chats {
                        Ok(chats) if !chats.is_empty() => {
                            chat_dispatch.reduce_mut(|s| {
                                s.chats = chats;
                                s.curr_chat_index = 0;
                            });
                        }
                        _ => {
                            init_new_chat();
                        }
                    }
                    LocalStorage::delete("chats");
                }
                _ => {
                    // let state = dispatch.get();
                    if chat_store.chats.is_empty() {
                        init_new_chat();
                    } else {
                        // Validate current chat index
                        let max_index = chat_store.chats.len().saturating_sub(1);
                        chat_dispatch.reduce_mut(|s| s.curr_chat_index = s.curr_chat_index.min(max_index as i32) );
                    }
                }
            }
    
            || ()
        });
    }
    html! {
        <div class="overflow-hidden w-full h-full relative">
            <Menu />
            <Chat />
            <ApiPopup />
            <Toast />
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
