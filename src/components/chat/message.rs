use yew::prelude::*;
use yewdux::use_store;

use crate::{components::chat::{avatar::Avatar, content_view::ContentView, edit_view::EditView, role_selector::RoleSelector}, store::slice::ConfigSlice, types::chat::Role};

#[derive(Debug, PartialEq, Properties)]
pub struct MessageProps {
    pub role: Role,
    content: String,
    message_index: i32,
    #[prop_or(false)]
    sticky: bool,
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
        class={classes!("w-full", "border-b", "border-black/10", "dark:border-gray-900/50", "text-gray-800", "dark:text-gray-100", "group", BACKGROUND_STYLE[(message_index % 2) as usize])}
      >
        <div
          class={classes!("text-base", "gap-4", "md:gap-6", "m-auto", "p-4", "md:py-6", "flex", "transition-all", "ease-in-out", if *hide_side_menu {"md:max-w-5xl lg:max-w-5xl xl:max-w-6xl"} else {"md:max-w-3xl lg:max-w-3xl xl:max-w-4xl"})}
        >
          <Avatar role={role.clone()} />
          <div class="w-[calc(100%-50px)]">
            if *advanced_mode {
                <RoleSelector
                    role={role}
                    messageIndex={messageIndex}
                    sticky={sticky}
                />
            }

            <MessageContent
              role={role}
              content={content}
              messageIndex={messageIndex}
              sticky={sticky}
            />
          </div>
        </div>
      </div>
    }
}

#[derive(Debug, Properties, PartialEq)]
pub struct MsgContentProps {
    role: String,
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
    let isEdit = use_state(|| *sticky);
    let (config_state, _) = use_store::<ConfigSlice>();
    let advancedMode = use_state(|| config_state.advanced_mode);

    html! {
        <div class="relative flex flex-col gap-2 md:gap-3 lg:w-[calc(100%-115px)]">
        if *advancedMode {
            <div class="flex flex-grow flex-col gap-3"></div>
        }
        if *isEdit {
          <EditView
            content={content.to_string()}
            is_edit = {isEdit.clone()}
            { message_index }
            sticky={*sticky}
          />
        } else {
          <ContentView
            role={role.to_string()}
            content={content.to_string()}
            is_edit = {isEdit.clone()}
            { message_index }
          />
        }
      </div>
    }
}
