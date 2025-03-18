use yew::prelude::*;
use yewdux::use_store;

use crate::{
    components::{
        chat::{
            avatar::Avatar, content_view::ContentView, edit_view::EditView,
            role_selector::RoleSelector,
        },
        icons::PlusIcon,
    },
    store::{ChatSlice, ConfigSlice},
    types::chat::{ChatInterface, MessageInterface, Role},
};

#[derive(Debug, PartialEq, Properties)]
pub struct MessageProps {
    pub role: Role,
    pub content: String,
    pub message_index: i32,
    #[prop_or(false)]
    pub sticky: bool,
}

const BACKGROUND_STYLE: [&str; 2] = ["dark:bg-gray-800", "bg-gray-50 dark:bg-gray-650"];

#[function_component]
pub fn Message(
    MessageProps {
        role,
        content,
        message_index,
        sticky,
    }: &MessageProps,
) -> Html {
    let (config, _) = use_store::<ConfigSlice>();
    let hide_side_menu = use_state(|| config.hide_side_menu);
    let advanced_mode = use_state(|| config.advanced_mode);
    html! {
        <div
        class={classes!("w-full", "border-b", "border-black/10", "dark:border-gray-900/50", "text-gray-800", "dark:text-gray-100", "group", 
        BACKGROUND_STYLE[(message_index % 2) as usize])}
      >
        <div
          class={classes!("text-base", "gap-4", "md:gap-6", "m-auto", "p-4", "md:py-6", "flex", "transition-all", "ease-in-out", 
          if *hide_side_menu {"md:max-w-5xl lg:max-w-5xl xl:max-w-6xl"} else {"md:max-w-3xl lg:max-w-3xl xl:max-w-4xl"})}
        >
          <Avatar role={role.clone()} />
          <div class="w-[calc(100%-50px)]">
            if *advanced_mode {
                <RoleSelector
                    role={role.clone()}
                    message_index={message_index}
                    sticky={sticky}
                />
            }

            <MessageContent
              role={role.clone()}
              content={content.clone()}
              message_index={message_index}
              sticky={sticky}
            />
          </div>
        </div>
      </div>
    }
}

#[derive(Debug, Properties, PartialEq)]
pub struct MsgContentProps {
    role: Role,
    content: String,
    message_index: i32,
    #[prop_or(false)]
    sticky: bool,
}

#[function_component]
fn MessageContent(
    MsgContentProps {
        role,
        content,
        message_index,
        sticky,
    }: &MsgContentProps,
) -> Html {
    let is_edit = use_state(|| *sticky);
    let (config_state, _) = use_store::<ConfigSlice>();
    let advanced_mode = use_state(|| config_state.advanced_mode);

    html! {
        <div class="relative flex flex-col gap-2 md:gap-3 lg:w-[calc(100%-115px)]">
        if *advanced_mode {
            <div class="flex flex-grow flex-col gap-3"></div>
        }
        if *is_edit {
          <EditView
            content={content.to_string()}
            is_edit = {is_edit.clone()}
            { message_index }
            sticky={*sticky}
          />
        } else {
          <ContentView
            role={role.to_string()}
            content={content.to_string()}
            is_edit = {is_edit.clone()}
            { message_index }
          />
        }
      </div>
    }
}

#[derive(Debug, Properties, PartialEq)]
pub struct MessageBtnProps {
    pub msg_index: i32,
}

#[function_component]
pub fn NewMessageButton(MessageBtnProps { msg_index }: &MessageBtnProps) -> Html {
    let (store, store_dispatch) = use_store::<ChatSlice>();
    let add_message = {
        let store = store.clone();
        let store_dispatch = store_dispatch.clone();
        move |_e| {
            let curr = store.curr_chat_index;
            store_dispatch.reduce_mut(|d| {
                if curr == -1 {
                    let mut title_index = 1;
                    let mut title = format!("New Chat {}", title_index);
                    while d
                        .chats
                        .iter()
                        .any(|chat| chat.title.as_ref().unwrap().clone() == title)
                    {
                        title_index += 1;
                        title = format!("New Chat {}", title_index);
                    }

                    let default_chat =
                        ChatInterface::new(title, None, vec![], None, "".to_string());
                    d.chats.insert(0, default_chat);
                    d.curr_chat_index = 0;
                } else {
                    d.messages.push(MessageInterface {
                        role: Role::User,
                        content: "".to_string(),
                        folder: None,
                        messages: vec![],
                    })
                }
            });
        }
    };
    html! {
        <div
        class="h-0 w-0 relative"
        key={*msg_index}
        aria-label="insert message"
      >
        <div
          class="absolute top-0 right-0 translate-x-1/2 translate-y-[-50%] text-gray-600 dark:text-white cursor-pointer bg-gray-200 dark:bg-gray-600/80 rounded-full p-1 text-sm hover:bg-gray-300 dark:hover:bg-gray-800/80 transition-bg duration-200"
          onclick={add_message}
        >
          <PlusIcon />
        </div>
      </div>
    }
}
