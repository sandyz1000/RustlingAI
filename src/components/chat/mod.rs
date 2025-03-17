pub(crate) mod avatar;
pub(crate) mod button;
pub(crate) mod code_block;
pub(crate) mod command_prompt;
pub(crate) mod config_menu;
pub(crate) mod content_view;
pub(crate) mod edit_view;
pub(crate) mod markdown;
pub(crate) mod message;
pub(crate) mod role_selector;

use super::icons::SendIcon;
use crate::{
    components::{
        chat::message::Message,
        icons::CrossIcon,
        menu::chat_histories::{ScrollToBottom, ScrollToBottomButton},
        mobile_bar::MobileBar,
        share_gpt::ShareGPT,
        stop_gen_btn::StopGeneratingButton,
    },
    hooks::translation::use_translation,
    store::slice::{ChatSlice, ConfigSlice, InputSlice},
    types::chat::{ConfigInterface, Role},
};
use config_menu::ConfigMenu;
use message::NewMessageButton;
use yew::prelude::*;
use yewdux::prelude::*;

#[function_component]
pub(crate) fn Chat() -> Html {
    let (state, _) = use_store::<ConfigSlice>();
    html! {
      <div
        class={classes!("flex", "h-full", "flex-1", "flex-col", if state.hide_side_menu {"md:pl-0"} else {"md:pl-[260px]"})}
      >
        <MobileBar />
        <main class="relative h-full w-full transition-width flex flex-col overflow-hidden items-stretch flex-1">
          <ChatContent />
          <StopGeneratingButton />
        </main>
      </div>
    }
}

#[function_component]
pub(crate) fn ChatTitle() -> Html {
    let x_translation = use_translation(vec!["model".to_string()]);
    let (chat_state, _dispatch) = use_store::<ChatSlice>();
    let config = use_state(|| None::<ConfigInterface>);
    if !chat_state.chats.is_empty() && chat_state.curr_chat_index < chat_state.chats.len() as i32 {
        let c = chat_state.chats[chat_state.curr_chat_index as usize]
            .config
            .clone();
        config.set(c.clone());
    }
    let curr_chat_index = use_state(|| chat_state.curr_chat_index);
    let set_config = {
        let curr_chat_index = curr_chat_index.clone();
        let _dispatch = _dispatch.clone();
        move |config| {
            _dispatch.reduce_mut(|e| e.chats[*curr_chat_index as usize].config = Some(config));
        }
    };

    let is_modal_open = use_state(|| false);

    // for migrating from old ChatInterface to new ChatInterface (with config)
    {
        let _dispatch = _dispatch.clone();
        use_effect_with(curr_chat_index, move |_curr_idx| {
            let chats = chat_state.chats.clone();
            let set_config = set_config.clone();
            if chats.len() > 0 {
                set_config(ConfigInterface::default());
            }
        });
    }

    let set_is_modal_open = {
        let is_modal_open = is_modal_open.clone();
        move |val: bool| {
            is_modal_open.set(val);
        }
    };
    if let Some(cfg) = &*config {
        return html! {
          <>
            <div
              class="flex gap-x-4 gap-y-1 flex-wrap w-full items-center justify-center border-b border-black/10 bg-gray-50 p-3 dark:border-gray-900/50 dark:bg-gray-700 text-gray-600 dark:text-gray-300 cursor-pointer"
              onclick={ let set_is_modal_open = set_is_modal_open.clone(); move |_e| set_is_modal_open(true) }
            >
              <div class="text-center p-1 rounded-md bg-gray-300/20 dark:bg-gray-900/10 hover:bg-gray-300/50 dark:hover:bg-gray-900/50">
                {format!("{}: {}", x_translation("model".to_string(), None), cfg.model)}
              </div>
              <div class="text-center p-1 rounded-md bg-gray-300/20 dark:bg-gray-900/10 hover:bg-gray-300/50 dark:hover:bg-gray-900/50">
                {format!("{}: {}", x_translation("token.label".to_string(), None), cfg.max_tokens)}
              </div>
              <div class="text-center p-1 rounded-md bg-gray-300/20 dark:bg-gray-900/10 hover:bg-gray-300/50 dark:hover:bg-gray-900/50">
                {format!("{}: {}", x_translation("temperature.label".to_string(), None), cfg.temperature)}
              </div>
              <div class="text-center p-1 rounded-md bg-gray-300/20 dark:bg-gray-900/10 hover:bg-gray-300/50 dark:hover:bg-gray-900/50">
                {format!("{}: {}", x_translation("topP.label".to_string(), None), cfg.top_p)}
              </div>
              <div class="text-center p-1 rounded-md bg-gray-300/20 dark:bg-gray-900/10 hover:bg-gray-300/50 dark:hover:bg-gray-900/50">
                {format!("{}: {}", x_translation("presencePenalty.label".to_string(), None), cfg.presence_penalty)}
              </div>
              <div class="text-center p-1 rounded-md bg-gray-300/20 dark:bg-gray-900/10 hover:bg-gray-300/50 dark:hover:bg-gray-900/50">
                {format!("{}: {}", x_translation("frequencyPenalty.label".to_string(), None), cfg.frequency_penalty)}
              </div>
            </div>
            if *is_modal_open {
              <ConfigMenu { set_is_modal_open } config={config.clone()} />
            }
          </>
        };
    }

    html! {
      <></>
    }
}

#[function_component]
pub(crate) fn ChatInput() -> Html {
    html! {
      <div class="w-full border-t md:border-t-0 dark:border-white/20 md:border-transparent md:dark:border-transparent md:bg-vert-light-gradient bg-white dark:bg-gray-800 md:!bg-transparent dark:md:bg-vert-dark-gradient">
        <form class="stretch mx-2 flex flex-row gap-3 pt-2 last:mb-2 md:last:mb-6 lg:mx-auto lg:max-w-3xl lg:pt-6">
          <div class="relative flex h-full flex-1 md:flex-col">
            <TextField />
          </div>
        </form>
      </div>
    }
}

#[function_component]
pub(crate) fn TextField() -> Html {
    html! {
      <div class="flex flex-col w-full py-2 flex-grow md:py-3 md:pl-4 relative border border-black/10 bg-white dark:border-gray-900/50 dark:text-white dark:bg-gray-700 rounded-md shadow-[0_0_10px_rgba(0,0,0,0.10)] dark:shadow-[0_0_15px_rgba(0,0,0,0.10)]">
        <textarea
          tabIndex={0}
          data-id="2557e994-6f98-4656-a955-7808084f8b8c"
          rows={1}
          class="m-0 w-full resize-none border-0 bg-transparent p-0 pl-2 pr-7 focus:ring-0 focus-visible:ring-0 dark:bg-transparent md:pl-0"
          style="max-height: 200px; height: 24px; overflow-y: hidden;"
        ></textarea>
        <button
          class="absolute p-1 rounded-md text-gray-500 bottom-1.5 right-1 md:bottom-2.5 md:right-2 hover:bg-gray-100 dark:hover:text-gray-400 dark:hover:bg-gray-900 disabled:hover:bg-transparent dark:disabled:hover:bg-transparent"
          aria-label="submit"
        >
          <SendIcon />
        </button>
      </div>
    }
}


#[function_component]
pub(crate) fn ChatContent() -> Html {
    let (input_store, _) = use_store::<InputSlice>();
    let (chat_store, _) = use_store::<ChatSlice>();
    let (config_store, _) = use_store::<ConfigSlice>();
    let input_role = use_state(|| input_store.input_role.clone());
    let error = use_state(|| chat_store.error.clone());
    let sticky_index = use_state(|| 0);
    let messages = use_state(|| {
        if !chat_store.chats.is_empty()
            && chat_store.curr_chat_index >= 0
            && chat_store.curr_chat_index < chat_store.chats.len() as i32
        {
            let messages = chat_store.chats[chat_store.curr_chat_index as usize].messages.clone();
            sticky_index.set(messages.len() as i32);
            messages
        } else {
            vec![]
        }
    });

    let advance_mode = use_state(|| config_store.advanced_mode);
    let generating = use_state(|| chat_store.generating);
    let hide_side_menu = use_state(|| config_store.hide_side_menu);
    let save_ref = use_node_ref();

    html! {
      <div class="flex-1 overflow-hidden">
        <ScrollToBottom
          class_name="h-full dark:bg-gray-800" follow_button_class_name="hidden"
        >
          <ScrollToBottomButton />
          <div class="flex flex-col items-center text-sm dark:bg-gray-800">
            <div
              class="flex flex-col items-center text-sm dark:bg-gray-800 w-full"
              ref={save_ref}
            >
              if *advance_mode {
                <ChatTitle />
              }
              if !*generating && *advance_mode && messages.is_empty() {
                  <NewMessageButton msg_index={-1} />
              }
              {
                messages.iter().enumerate().map(|(index, message)| {
                    html! {
                        if *advance_mode || index != 0 || message.role != Role::System {
                            <div key={index}>
                                <Message
                                role={message.role.clone()}
                                content={message.content.clone()}
                                message_index={index as i32}
                                />
                                if !*generating && *advance_mode {
                                    <NewMessageButton msg_index={index as i32} />
                                }
                            </div>
                        } else {
                            <></>
                        }
                    }
                }).collect::<Html>()
              }
            </div>

            <Message
                role={(*input_role).clone()}
                content=""
                message_index={*sticky_index}
                sticky={false}
            />
            if !error.is_empty() {
              <div class="relative py-2 px-3 w-3/5 mt-3 max-md:w-11/12 border rounded-md border-red-500 bg-red-500/10">
                <div class="text-gray-600 dark:text-gray-100 text-sm whitespace-pre-wrap">
                  { error.to_string() }
                </div>
                <div
                  class="text-white absolute top-1 right-1 cursor-pointer"
                  onclick={ let error = error.clone(); move |_| error.set("".to_string()) }
                >
                  <CrossIcon />
                </div>
              </div>
            }
            <div
              class={classes!("mt-4", "w-full", "m-auto", if *hide_side_menu { "md:max-w-5xl lg:max-w-5xl xl:max-w-6xl" } else {"md:max-w-3xl lg:max-w-3xl xl:max-w-4xl"} )}
            >
              if *generating {
                <div class="md:w-[calc(100%-50px)] flex gap-4 flex-wrap justify-center">
                  // <DownloadChat {save_ref} />
                  <ShareGPT />
                  // <CloneChat />
                </div>
              }
            </div>
            <div class="w-full h-36"></div>
          </div>
        </ScrollToBottom>
      </div>
    }
}
