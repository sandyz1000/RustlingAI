use std::{cell::RefCell, rc::Rc};

use crate::components::{
    chat::command_prompt::CommandPrompt, popup_modal::PopupModal, token_count::TokenCount,
};
use crate::{
    hooks::{submit::use_submit, translation::use_translation},
    store::slice::{ChatSlice, ConfigSlice, InputSlice},
    types::chat::{ChatInterface, MessageInterface},
};
use web_sys::HtmlTextAreaElement;
use yew::prelude::*;
use yewdux::use_store;

#[derive(Debug, Properties, PartialEq)]
pub(crate) struct EditViewProps {
    content: String,
    is_edit: UseStateHandle<bool>,
    message_index: i32,
    sticky: bool,
}

#[function_component]
pub fn EditView(
    EditViewProps {
        content,
        is_edit,
        message_index,
        sticky,
    }: &EditViewProps,
) -> Html {
    let (input_slice, input_dispatch) = use_store::<InputSlice>();
    let (chat_slice, chat_slice_dispatch) = use_store::<ChatSlice>();
    let (chat, chat_dispatch) = use_store::<ChatInterface>();

    let input_role = use_state(|| input_slice.input_role.clone());
    let curr_chat_index = use_state(|| chat_slice.curr_chat_index);
    let content = use_state(|| content.clone());
    let is_modal_open = use_state(|| false);
    let textarea_ref = use_node_ref();

    let t = use_translation(vec![]);

    let resetTextAreaHeight = {
        let textarea_ref = textarea_ref.clone();
        || {
            if let Some(element) = textarea_ref.cast::<HtmlTextAreaElement>() {
                element.set_attribute("style", "height: auto").ok();
            }
        }
    };

    // Function to check if the user is on a mobile device
    fn is_mobile() -> bool {
        let user_agent = gloo_utils::window().navigator().user_agent().unwrap();
        let mobile_regex = regex::Regex::new(
            r"(?i)Android|webOS|iPhone|iPad|iPod|BlackBerry|IEMobile|Opera Mini|playbook|silk",
        )
        .unwrap();
        mobile_regex.is_match(&user_agent)
    }

    let handle_save = {
        let chat_store = chat_slice.clone();
        let content = content.clone();
        let chat_dispatch = chat_dispatch.clone();

        let curr_chat_index = *curr_chat_index as usize;
        let mess_idx = *message_index as usize;
        let is_edit = is_edit.clone();
        |()| {
            if *sticky && (&*content == "" || chat_store.generating) {
                return;
            }
            let up_messages = chat_store.chats[curr_chat_index].messages.clone();
            if *sticky {
                content.set("".to_string());
                resetTextAreaHeight();
            } else {
                chat_dispatch.reduce_mut(|f| f.messages[mess_idx].content = *content);
                is_edit.set(false);
            }
        }
    };

    let (handle_submit, _a) = use_submit();
    let handle_generate = {
        let handle_submit = handle_submit.clone();

        let chat_slice = chat_slice.clone();
        let slice_dispatch = chat_slice_dispatch.clone();
        let input_role = input_role.clone();
        let content = content.clone();
        let curr_chat_index = *curr_chat_index as usize;
        let message_index = *message_index as usize;
        let is_edit = is_edit.clone();
        |()| {
            if chat_slice.generating {
                return;
            }
            let mut updated_messages = chat_slice.chats[curr_chat_index].messages.clone();
            if *sticky {
                if !content.is_empty() {
                    slice_dispatch.reduce_mut(|f| {
                        f.chats[curr_chat_index].messages.push(MessageInterface {
                            role: *input_role,
                            content: *content,
                            folder: None,
                            messages: vec![],
                        })
                    });
                }
                content.set("".to_string());
                resetTextAreaHeight();
            } else {
                slice_dispatch.reduce_mut(|f| {
                    updated_messages[message_index].content = *content;
                    f.messages = updated_messages
                        .get(..=message_index) // Safely handle bounds
                        .unwrap_or(&[]) // Fallback to empty slice if out of bounds
                        .to_vec();
                });
                is_edit.set(false);
            }
            handle_submit();
        }
    };
    // The event handler for key down

    let handle_key_down = {
        let handle_generate = handle_generate.clone();
        let handle_save = handle_save.clone();
        |event: KeyboardEvent| {
            if event.key() == "Enter" && !is_mobile() && !event.is_composing() {
                let enter_to_submit = get_enter_to_submit(); // Placeholder for state retrieval

                if event.ctrl_key() && event.shift_key() {
                    event.prevent_default();
                    handle_generate(());
                    reset_text_area_height();
                } else if (enter_to_submit && !event.shift_key())
                    || (!enter_to_submit && (event.ctrl_key() || event.shift_key()))
                {
                    if is_sticky() {
                        // Placeholder function
                        event.prevent_default();
                        handle_generate(());
                        reset_text_area_height();
                    } else {
                        handle_save(());
                    }
                }
            }
        }
    };

    // Placeholder functions (to be implemented based on state management)
    fn get_enter_to_submit() -> bool {
        // Implement state retrieval logic (similar to useStore.getState().enterToSubmit in JS)
        true // Default value for now
    }

    fn is_sticky() -> bool {
        // Implement logic to check if sticky mode is enabled
        false // Default value for now
    }

    fn reset_text_area_height() {
        // Implement logic to reset text area height
        // web_sys::console::log_1(&"Resetting text area height...".into());
    }

    // Effect for content changes
    {
        let textarea_ref = textarea_ref.clone();
        let content = content.clone();
        
        use_effect_with((*content).clone(), move |_| {
            if let Some(textarea) = textarea_ref.cast::<HtmlTextAreaElement>() {
                // Reset height to auto first
                textarea.set_attribute("style", "height: auto").ok();
                // Set height to scroll height
                let scroll_height = textarea.scroll_height();
                textarea.set_attribute("style", &format!("height: {}px", scroll_height)).ok();
            }
            || ()
        });
    }

    // Initial effect
    {
        let textarea_ref = textarea_ref.clone();
        use_effect(move || {
            if let Some(textarea) = textarea_ref.cast::<HtmlTextAreaElement>() {
                textarea.set_attribute("style", "height: auto").ok();
                let scroll_height = textarea.scroll_height();
                textarea.set_attribute("style", &format!("height: {}px", scroll_height)).ok();
            }
            || ()
        });
    }

    html! {
      <>
        <div
          class={classes!("w-full", if *sticky {"py-2 md:py-3 px-2 md:px-4 border border-black/10 bg-white dark:border-gray-900/50 dark:text-white dark:bg-gray-700 rounded-md shadow-[0_0_10px_rgba(0,0,0,0.10)] dark:shadow-[0_0_15px_rgba(0,0,0,0.10)]"} else {""}) }
        >
          <textarea
            ref={textarea_ref}
            class="m-0 resize-none rounded-lg bg-transparent overflow-y-hidden focus:ring-0 focus-visible:ring-0 leading-7 w-full placeholder:text-gray-500/40"
            onchange={|e| {}}
            value={content.to_string()}
            placeholder={t("submitPlaceholder".to_string(), None)}
            onkeydown={handle_key_down}
            rows={1}
          ></textarea>
        </div>
        <EditViewButtons
          sticky={sticky}
          handle_generate={handle_generate.clone()}
          handle_save={handle_save.clone()}
          set_is_modal_open={set_is_modal_open}
          set_is_edit={is_edit.clone()}
          set_content={ let content = content.clone(); |val: String| content.set(val) }
        />
        if *is_modal_open {
          <PopupModal
            set_is_modal_open={set_is_modal_open}
            title={t("warning".to_string(), None)}
            message={t("clearMessageWarning".to_string(), None)}
            handle_confirm={handle_generate.clone()}
          />
        }
      </>
    }
}

#[derive(Debug, PartialEq, Properties)]
pub struct EvButtonsProps {
    sticky: bool,
    handle_generate: Callback<()>,
    handle_save: Callback<()>,
    set_is_modal_open: Callback<bool>,
    set_is_edit: Callback<bool>,
    set_content: Callback<String>,
}

#[function_component]
pub fn EditViewButtons(
    EvButtonsProps {
        sticky,
        handle_generate,
        handle_save,
        set_is_modal_open,
        set_is_edit,
        set_content,
    }: &EvButtonsProps,
) -> Html {
    let t = use_translation(vec![]);
    let (chatstore, _) = use_store::<ChatSlice>();
    let (configstore, _) = use_store::<ConfigSlice>();
    let generating = chatstore.generating;
    let advanced_mode = use_state(|| configstore.advanced_mode);
    let handle_save = handle_save.clone();
    let set_is_edit = set_is_edit.clone();
    html! {
      <div class="flex">
        <div class="flex-1 text-center mt-2 flex justify-center">
          if *sticky {
            <button
              class={classes!("btn", "relative", "mr-2", "btn-primary", if generating {"cursor-not-allowed opacity-40"} else {""}) }
              onclick={let handle_generate = handle_generate.clone(); |e| handle_generate.emit(())}
              aria-label={t("generate".to_string(), None)}
            >
              <div class="flex items-center justify-center gap-2">
                {t("generate".to_string(), None)}
              </div>
            </button>
          }

          if *sticky {
            <button
              class="btn relative mr-2 btn-primary"
              onclick={let set_is_modal_open = set_is_modal_open.clone(); |e| if !generating {set_is_modal_open.emit(true)}}
            >
              <div class="flex items-center justify-center gap-2">
                {t("generate".to_string(), None)}
              </div>
            </button>
          }

          <button
            class={classes!("btn", "relative", "mr-2", if *sticky { format!("btn-neutral {}", if generating { "cursor-not-allowed opacity-40"} else {""} ) } else {"btn-neutral".to_string()})
            }
            onclick={|e| handle_save.emit(())}
            aria-label={t("save".to_string(), None)}
          >
            <div class="flex items-center justify-center gap-2">
              {t("save".to_string(), None)}
            </div>
          </button>

          if *sticky {
            <button
              class="btn relative btn-neutral"
              onclick={|e| set_is_edit.emit(false)}
              aria-label={t("cancel".to_string(), None)}
            >
              <div class="flex items-center justify-center gap-2">
                {t("cancel".to_string(), None)}
              </div>
            </button>
          }
        </div>
        if *sticky && *advanced_mode {
          <TokenCount />
        }
        <CommandPrompt set_content={set_content.clone()} />
      </div>
    }
}
