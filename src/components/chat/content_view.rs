use std::str::FromStr;
use yew::prelude::*;
use yew_hooks::{use_clipboard, UseClipboardHandle};
use yewdux::{Dispatch, use_store};

use crate::components::chat::{
    button::{
        CopyButton, DeleteButton, DownButton, EditButton, MarkdownModeButton, RefreshButton,
        UpButton,
    },
    markdown::Markdown,
};
use crate::components::icons::{CrossIcon, TickIcon};
use crate::{
    hooks::submit::use_submit,
    store::slice::{ChatSlice, ConfigSlice},
    types::chat::Role,
};

enum Direction {
    Up,
    Down,
}

#[derive(Debug, Properties, PartialEq)]
pub struct ContentViewProps {
    pub role: String,
    pub content: String,
    pub is_edit: UseStateHandle<bool>,
    pub message_index: i32,
}

#[function_component]
pub fn ContentView(
    ContentViewProps {
        role,
        content,
        is_edit,
        message_index,
    }: &ContentViewProps,
) -> Html {
    let (handle_submit, _) = use_submit();
    let clipboard = use_clipboard();
    let (state, chat_dispatch) = use_store::<ChatSlice>();
    let (cfg_state, _dispatch) = use_store::<ConfigSlice>();
    let is_delete = use_state(|| false);
    let content = content.clone();
    let curr_chat_index = use_state(|| state.curr_chat_index);

    let last_message_index = use_state(|| {
        if !state.chats.is_empty() {
            state.chats[state.curr_chat_index as usize].messages.len() as i32 - 1
        } else {
            0
        }
    });
    // let inlineLatex = use_state(|| cfg_state.inline_latex);
    let markdown_mode = use_state(|| cfg_state.markdown_mode);

    let handle_move = {
        let chat_dispatch = chat_dispatch.clone();
        let state = state.clone();
        let message_index = *message_index as usize;
        let curr_chat_index = curr_chat_index.clone();
        move |direction: Direction| {
            let temp = state.messages[message_index].clone();
            chat_dispatch.reduce_mut(|e| match direction {
                Direction::Up => {
                    e.chats[(*curr_chat_index) as usize].messages[message_index as usize] =
                        state.messages[message_index - 1].clone();
                    e.messages[message_index - 1] = temp;
                }
                Direction::Down => {
                    e.chats[(*curr_chat_index) as usize].messages[message_index] =
                        state.messages[message_index + 1].clone();
                    e.messages[message_index + 1] = temp;
                }
            });
        }
    };

    let handle_refresh = {
        let chat_dispatch = chat_dispatch.clone();
        let state = state.clone();
        let handle_submit = handle_submit.clone();
        let curr_chat_index = curr_chat_index.clone();
        move |e| {
            let messeges = &state.chats[(*curr_chat_index) as usize].messages;
            chat_dispatch.reduce_mut(|e| {
                e.chats[(*curr_chat_index) as usize]
                    .messages
                    .remove(messeges.len() - 1)
            });
            handle_submit();
        }
    };

    fn handle_delete(chat_dispatch: &Dispatch<ChatSlice>, message_index: usize) {
        chat_dispatch.reduce_mut(|f| f.chats.remove(message_index));
    }

    fn handle_copy(clipboard: &UseClipboardHandle, content: String) {
        clipboard.write_text(content);
    }

    html! {
       <>
         <div class="markdown prose w-full md:max-w-full break-words dark:prose-invert dark share-gpt-message">
         if *markdown_mode {
            <Markdown children={content.clone()} />
         } else {
           <span class="whitespace-pre-wrap">{content.clone()}</span>
         }
         </div>
         <div class="flex justify-end gap-2 w-full mt-2">
           if *is_delete {
               <>
               if state.generating && Role::from_str(role).unwrap() == Role::Assistant &&
                 *message_index == *last_message_index {
                   <RefreshButton on_click={handle_refresh} />
                 }
               if *message_index != 0 {
                   <UpButton on_click={ let handle_move=handle_move.clone(); move |e| handle_move(Direction::Up) } />
               }
               if *message_index != *last_message_index {
                 <DownButton on_click={ let handle_move=handle_move.clone(); move |e| handle_move(Direction::Down) } />
               }

               <MarkdownModeButton />
               <CopyButton on_click={ 
                    let (clipboard, content) = (clipboard.clone(), content.clone());
                    move |_| {
                        clipboard.write_text(content.clone());
                    }
                } />
               <EditButton set_is_edit={ let is_edit=is_edit.clone(); move |val| is_edit.set(val) } />
               <DeleteButton set_is_delete={ let is_delete = is_delete.clone(); move |value| is_delete.set(value) } />
             </>

             <>
               <button
                 class="p-1 hover:text-white"
                 aria-label="cancel"
                 onclick={let is_delete = is_delete.clone(); move |e| is_delete.set(false)}
               >
                 <CrossIcon />
               </button>
               <button
                class="p-1 hover:text-white"
                aria-label="confirm"
                onclick={
                    let chat_dispatch = chat_dispatch.clone();
                    let message_index = *message_index as usize;
                    move |_| handle_delete(&chat_dispatch, message_index)
                }
               >
                 <TickIcon />
               </button>
             </>
           }
         </div>
       </>
    }
}
