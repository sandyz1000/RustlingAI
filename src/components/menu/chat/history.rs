use wasm_bindgen::JsCast;
use web_sys::HtmlInputElement;
use yew::prelude::*;
use yewdux::use_store;

use crate::{
    components::icons::{ChatIcon, CrossIcon, DeleteIcon, EditIcon, TickIcon},
    store::{ChatSlice, ConfigSlice},
    types::chat::{ChatInterface, ConfigInterface},
};

#[derive(Debug, Properties, PartialEq)]
pub struct ChatHistoryProps {
    pub title: String,
    pub chat_index: i32,
}

enum ChatHistoryClass {
    Normal,
    Active,
    NormalGradient,
    ActiveGradient,
}

impl std::fmt::Display for ChatHistoryClass {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ChatHistoryClass::Normal => "flex py-2 px-2 items-center gap-3 relative rounded-md bg-gray-900 hover:bg-gray-850 break-all hover:pr-4 group transition-opacity",
            ChatHistoryClass::Active => "flex py-2 px-2 items-center gap-3 relative rounded-md break-all pr-14 bg-gray-800 hover:bg-gray-800 group transition-opacity",
            ChatHistoryClass::NormalGradient => "absolute inset-y-0 right-0 w-8 z-10 bg-gradient-to-l from-gray-900 group-hover:from-gray-850",
            ChatHistoryClass::ActiveGradient => "absolute inset-y-0 right-0 w-8 z-10 bg-gradient-to-l from-gray-800",
        }
        .fmt(f)
    }
}

#[function_component]
pub fn ChatHistory(ChatHistoryProps { title, chat_index }: &ChatHistoryProps) -> Html {
    let (chat_store, _dispatch) = use_store::<ChatSlice>();
    let (config_store, _) = use_store::<ConfigSlice>();
    let (chat_config, _) = use_store::<ConfigInterface>();
    let set_curr_chat_index = {
        let _dispatch = _dispatch.clone();
        move |index: i32| _dispatch.reduce_mut(|cs| cs.curr_chat_index = index)
    };
    let active = *chat_index == chat_store.curr_chat_index;
    let generating = chat_store.generating;

    let is_delete = use_state(|| false);
    let is_edit = use_state(|| false);
    let title = use_state(|| title.to_string());
    let input_ref = use_node_ref();

    let edit_title = {
        let store = chat_store.clone();
        let title = title.clone();
        let _dispatch = _dispatch.clone();
        let is_edit = is_edit.clone();
        let chat_index = (*chat_index) as usize;
        move || {
            let mut chats = store.chats.clone();
            chats[chat_index].title = Some((*title).clone());
            _dispatch.reduce_mut(|c| c.chats = chats);
            is_edit.set(false);
        }
    };

    let delete_chat = {
        let chat_store = chat_store.clone();
        let config_store = config_store.clone();
        let chat_config = chat_config.clone();
        let _dispatch = _dispatch.clone();
        let is_delete = is_delete.clone();
        let set_curr_chat_index = set_curr_chat_index.clone();
        let chat_index = (*chat_index) as usize;
        move || {
            let mut chats = chat_store.chats.clone();
            let default_sys_msg = config_store.default_system_message.clone();
            if !chats.is_empty() {
                chats.remove(chat_index);
            }
            if chats.is_empty() {
                chats = vec![ChatInterface::new(
                    "".to_string(),
                    None,
                    vec![],
                    Some((*chat_config).clone()),
                    default_sys_msg,
                )];
            }
            set_curr_chat_index(0);
            _dispatch.reduce_mut(|c| c.chats = chats);
            is_delete.set(false);
        }
    };

    let handle_drag_start = {
        let chat_index = *chat_index as usize;
        move |e: DragEvent| {
            if e.data_transfer().is_some() {
                let _ = e
                    .data_transfer()
                    .unwrap()
                    .set_data("chatIndex", &chat_index.to_string());
            }
        }
    };
    let handle_key_down = {
        let edit_title = edit_title.clone();
        move |e: KeyboardEvent| {
            if e.key() == "Enter" {
                edit_title();
            }
        }
    };
    let handle_tick = {
        let is_edit = is_edit.clone();
        let is_delete = is_delete.clone();
        let edit_title = edit_title.clone();
        let delete_chat = delete_chat.clone();
        move |e: MouseEvent| {
            e.stop_propagation();
            if *is_edit {
                edit_title();
            } else if *is_delete {
                delete_chat();
            }
        }
    };
    let handle_cross = {
        let is_edit = is_edit.clone();
        let is_delete = is_delete.clone();
        move |_e| {
            is_edit.set(false);
            is_delete.set(false);
        }
    };

    {
        let is_edit = is_edit.clone();
        let input_ref = input_ref.clone();
        use_effect_with(is_edit, move |_| {
            if let Some(input) = input_ref.cast::<web_sys::HtmlInputElement>() {
                input
                    .focus()
                    .unwrap_or_else(|_| log::warn!("Failed to focus input"));
            }
            || ()
        })
    }
    let on_change_edit = {
        let title = title.clone();
        move |e: Event| {
            if let Some(s) = e.dyn_into::<HtmlInputElement>().ok() {
                let v = s.value();
                title.set(v);
            }
        }
    };

    html! {
        <a
        class={classes!(if active {ChatHistoryClass::ActiveGradient.to_string()} else {ChatHistoryClass::NormalGradient.to_string()}) }
        onclick={
            let set_curr_chat_index = set_curr_chat_index.clone();
            let chat_index = *chat_index;
            move |_e| if !generating { set_curr_chat_index(chat_index) }
        }
        draggable="true"
        ondragstart={handle_drag_start}
      >
        <ChatIcon />
        <div class="flex-1 text-ellipsis max-h-5 overflow-hidden break-all relative" title={(*title).clone()}>
          if *is_edit {
            <input
              type="text"
              class="focus:outline-blue-600 text-sm border-none bg-transparent p-0 m-0 w-full"
              value={(*title).clone()}
              onchange={ on_change_edit }
              onkeydown={handle_key_down}
              ref={input_ref}
            />
          } else {
            <>
            {(*title).clone()}
            </>
          }

          if *is_edit {
          <div
              class={classes!(if active {ChatHistoryClass::ActiveGradient.to_string()} else {ChatHistoryClass::NormalGradient.to_string()}, if generating {"cursor-not-allowed opacity-40"} else {"cursor-pointer opacity-100"} )}
            />
        }
        </div>
        if active {
            <div class="absolute flex right-1 z-10 text-gray-300 visible">
            if *is_delete || *is_edit {
                <>
                <button
                  class="p-1 hover:text-white"
                  onclick={handle_tick}
                  aria-label="confirm"
                >
                  <TickIcon />
                </button>
                <button
                  class="p-1 hover:text-white"
                  onclick={handle_cross}
                  aria-label="cancel"
                >
                  <CrossIcon />
                </button>
              </>
            } else {
                <>
                <button
                  class="p-1 hover:text-white"
                  onclick={let is_edit = is_edit.clone(); move |_| is_edit.set(true) }
                  aria-label="edit chat title"
                >
                  <EditIcon />
                </button>
                <button
                  class="p-1 hover:text-white"
                  onclick={let is_delete = is_delete.clone(); move |_| is_delete.set(true) }
                  aria-label="delete chat"
                >
                  <DeleteIcon />
                </button>
              </>
            }
          </div>
        }
      </a>
    }
}
