use std::{collections::HashMap, hash::Hash};

use crate::{
    components::{icons::DownArrow, menu::{
        chat_folder::ChatFolder, chat_history::ChatHistory, chat_search::ChatSearch,
    }},
    store::ChatSlice,
    types::chat::{ChatHistoryFolderInterface, ChatHistoryInterface, Folder},
};
use gloo_timers::callback::Interval;
use web_sys::HtmlElement;
use yew::prelude::*;
use yew_hooks::use_debounce;
use yewdux::prelude::*;

#[function_component]
pub fn ChatHistoryList() -> Html {
    let (store, store_dispatch) = use_store::<ChatSlice>();
    let chat_titles = use_state(|| {
        store
            .chats
            .iter()
            .filter(|s| s.title.is_some())
            .map(|s| s.title.as_ref().unwrap().clone())
            .collect::<Vec<String>>()
    });
    let current_chat_index = use_state(|| store.curr_chat_index);
    let is_hover = use_state(|| false);
    let filter = use_state(|| "".to_string());
    let chat_folders = use_state(|| ChatHistoryFolderInterface::new());
    let no_chat_folders = use_state(|| Vec::<ChatHistoryInterface>::new());

    // Refs
    let chats_ref = use_mut_ref(|| store.chats.clone());
    let folders_ref = use_mut_ref(|| store.folders.clone());
    let filter_ref = use_mut_ref(|| (*filter).clone());

    // Update folders logic
    let update_folders = {
        let chat_folders = chat_folders.clone();
        let no_chat_folders = no_chat_folders.clone();
        let store = store.clone();
        let filter_ref = filter_ref.clone();

        move || {
            let mut new_folders = HashMap::new();
            let mut new_no_folders = Vec::new();
            let chats = store.chats.clone();
            let folders = store.folders.clone();

            // Sort folders
            let mut sorted_folders: Vec<Folder> = folders.values().map(|f| f.clone()).collect();
            sorted_folders.sort_by_key(|f| f.order);

            // Process chats
            for (index, chat) in chats.iter().enumerate() {
                let filter_lower = filter_ref.borrow().to_lowercase();
                let chat_title = chat.title.as_ref().unwrap().to_lowercase();
                let folder_name = chat
                    .folder
                    .as_ref()
                    .and_then(|f| folders.get(f))
                    .map(|f| f.name.to_lowercase())
                    .unwrap_or_default();

                if !chat_title.contains(&filter_lower)
                    && !folder_name.contains(&filter_lower)
                    && index as i32 != store.curr_chat_index
                {
                    continue;
                }

                let entry = ChatHistoryInterface {
                    title: chat.title.as_ref().unwrap().clone(),
                    index: index as i32,
                    id: chat.id.clone(),
                };

                match &chat.folder {
                    Some(folder_id) => {
                        new_folders
                            .entry(folder_id.clone())
                            .or_insert_with(Vec::new)
                            .push(entry);
                    }
                    None => new_no_folders.push(entry),
                }
            }

            chat_folders.set(new_folders);
            no_chat_folders.set(new_no_folders);
        }
    };

    // Store subscription effect
    {
        let store_dispatch: Dispatch<ChatSlice> = store_dispatch.clone();
        let store = store.clone();
        let update_folders = update_folders.clone();
        let chats_ref = chats_ref.clone();
        let folders_ref = folders_ref.clone();

        // TODO: Verify if this working correctly
        use_effect_with((), move |_| {
            let unsubscribe = store_dispatch.subscribe({
                // let store = store.clone();
                move |store: std::rc::Rc<ChatSlice>| {
                    if store.chats != *chats_ref.borrow() && !store.generating {
                        update_folders();
                        *chats_ref.borrow_mut() = store.chats.clone();
                    } else if store.folders != *folders_ref.borrow() {
                        update_folders();
                        *folders_ref.borrow_mut() = store.folders.clone();
                    }
                }
            });

            || ()
        });
    }

    // Document title effect
    {
        let chat_titles = chat_titles.clone();
        let current_chat_index = *current_chat_index;
        let store = store.clone();
        let store_dispatch = store_dispatch.clone();

        use_effect_with((current_chat_index, chat_titles), move |(index, titles)| {
            if *index < titles.len() as i32 {
                // Set document title
                gloo_utils::window()
                    .document()
                    .unwrap()
                    .set_title(&titles[(*index) as usize]);

                // Expand folder
                let chat = &store.chats[*index as usize];

                if let Some(folder_id) = &chat.folder {
                    let mut folders = store.folders.clone();
                    if let Some(folder) = folders.get_mut(folder_id) {
                        folder.expanded = true;
                        store_dispatch.reduce_mut(|s| s.folders = folders.clone());
                    }
                }
            }

            || ()
        });
    }

    let handle_drop = {
        let store_dispatch = store_dispatch.clone();
        move |e: DragEvent| {
            if e.data_transfer().is_some() {
                e.stop_propagation();
                if let Ok(chat_index) = e.data_transfer().unwrap()
                    .get_data("chatIndex")
                    .map(|s| s.parse::<usize>().ok().unwrap()) {
                    store_dispatch.reduce_mut(|s| s.chats[chat_index].folder.take());
                }
            }
        }
    };

    let handle_drag_over = {
        let is_hover = is_hover.clone();
        move |_e: DragEvent| {
            _e.prevent_default();
            is_hover.set(true);
        }
    };

    let handle_drag_leave = {
        let is_hover = is_hover.clone();
        move |_e: DragEvent| {
            is_hover.set(false);
        }
    };

    let handle_drag_end = {
        let is_hover = is_hover.clone();
        move |_e| {
            is_hover.set(false);
        }
    };
    html! {
        <div
      class={classes!("flex-col", "flex-1", "overflow-y-auto", "hide-scroll-bar", "border-b", "border-white/20", if *is_hover {"bg-gray-800/40"} else {""})}
      ondrop={handle_drop}
      ondragover={handle_drag_over}
      ondragleave={handle_drag_leave}
      ondragend={handle_drag_end}
    >
      <ChatSearch filter={filter.clone()} />
      <div class="flex flex-col gap-2 text-gray-100 text-sm">

        {
            chat_folders.iter().map(|(id, chat_histories)| {
                html!{
                    <ChatFolder
                        folder_id={ id.clone() }
                        folder_chats={ (*chat_histories).clone() }
                    //   key={folderId}
                    />
                }
            }).collect::<Html>()
        }
        {
            no_chat_folders.iter().map(|c| {
                html!{
                    <ChatHistory title={c.title.clone()} key={format!("{}-{}", c.title, c.id)} chat_index={c.index} />
                }
            }).collect::<Html>()
        }
      </div>
      <div class="w-full h-10" />
    </div>
    }
}

#[function_component]
pub fn ShowMoreButton() -> Html {
    html! {
        <button class="btn relative btn-dark btn-small m-auto mb-2">
            <div class="flex items-center justify-center gap-2">{"Show more"}</div>
        </button>
    }
}

#[derive(Properties, PartialEq, Clone)]
pub struct ScrollToBottomProps {
    #[prop_or(500)]
    pub check_interval: u32, // Interval for stickiness check (ms)

    #[prop_or("".to_string())]
    pub class_name: String, // Root element class

    #[prop_or(100)]
    pub debounce: u32, // Debounce time for scrolling

    #[prop_or_default]
    pub debug: bool, // Debug logs

    #[prop_or_default]
    pub follow_button_class_name: String, // Follow button class

    #[prop_or(String::from("smooth"))]
    pub initial_scroll_behavior: String, // "auto" or "smooth"

    #[prop_or(String::from("bottom"))]
    pub mode: String, // "bottom" or "top"

    #[prop_or_default]
    pub nonce: Option<String>, // CSP nonce

    #[prop_or_default]
    pub scroller: Option<Callback<ScrollerParams, u32>>, // Custom scrolling logic

    #[prop_or_default]
    pub scroll_view_class_name: Option<String>, // Scroll container class

    pub children: Children, // Child elements
}

#[derive(Clone, Debug)]
pub struct ScrollerParams {
    pub max_value: u32,
    pub min_value: u32,
    pub offset_height: u32,
    pub scroll_height: u32,
    pub scroll_top: u32,
}

#[function_component]
pub(crate) fn ScrollToBottomButton() -> Html {
    let scroll_btm = |e: MouseEvent| {};
    let at_bottom = use_state(|| false);
    html! {
      <button
        class={classes!("cursor-pointer", "absolute", "right-6", "bottom-[60px]", "md:bottom-[60px]", "z-10", "rounded-full", "border", "border-gray-200", "bg-gray-50", "text-gray-600", "dark:border-white/10", "dark:bg-white/10", "dark:text-gray-200", if *at_bottom {"hidden"} else {""}) }
        aria-label="scroll to bottom"
        onclick={scroll_btm}
      >
        <DownArrow />
      </button>
    }
}


// TODO: Revisit and fix this
#[function_component]
pub fn ScrollToBottom(props: &ScrollToBottomProps) -> Html {
    let node_ref = use_node_ref();
    let is_sticky = use_state(|| true);

    // Debounced scroll handler
    let on_scroll = {
        let is_sticky = is_sticky.clone();
        let node_ref = node_ref.clone();
        let debounce_time = props.debounce;
        // TODO: Fix this implementation
        |e| {
            // use_debounce(
            //     move || {
            //         if let Some(elem) = node_ref.cast::<HtmlElement>() {
            //             let scroll_top = elem.scroll_top() as u32;
            //             let scroll_height = elem.scroll_height() as u32;
            //             let offset_height = elem.offset_height() as u32;
            //             let bottom_reached = scroll_top + offset_height >= scroll_height - 10;

            //             is_sticky.set(bottom_reached);
            //         }
            //     },
            //     debounce_time,
            // );
        }
    };

    // Scroll to bottom/top logic
    let scroll_to_target = {
        let node_ref = node_ref.clone();
        let mode = props.mode.clone();
        let scroller = props.scroller.clone();

        Callback::from(move |_| {
            if let Some(elem) = node_ref.cast::<HtmlElement>() {
                let max_scroll = elem.scroll_height() as u32 - elem.offset_height() as u32;
                let min_scroll = 0;
                let scroll_to = match scroller {
                    Some(ref callback) => callback.emit(ScrollerParams {
                        max_value: max_scroll,
                        min_value: min_scroll,
                        offset_height: elem.offset_height() as u32,
                        scroll_height: elem.scroll_height() as u32,
                        scroll_top: elem.scroll_top() as u32,
                    }),
                    None => {
                        if mode == "bottom" {
                            max_scroll
                        } else {
                            min_scroll
                        }
                    }
                };

                elem.set_scroll_top(scroll_to as i32);
            }
        })
    };

    // Interval for checking stickiness
    {
        let node_ref = node_ref.clone();
        let is_sticky = is_sticky.clone();
        let check_interval = props.check_interval;

        use_effect(move || {
            let interval = Interval::new(check_interval, move || {
                if let Some(elem) = node_ref.cast::<HtmlElement>() {
                    let scroll_top = elem.scroll_top() as u32;
                    let scroll_height = elem.scroll_height() as u32;
                    let offset_height = elem.offset_height() as u32;
                    let bottom_reached = scroll_top + offset_height >= scroll_height - 10;

                    is_sticky.set(bottom_reached);
                }
            });

            move || drop(interval)
        })
    }

    html! {
        <div class={props.class_name.clone()} ref={node_ref} onscroll={on_scroll}>
            <div class={props.scroll_view_class_name.clone().unwrap_or_default()}>
                { for props.children.iter() }
            </div>
            if !*is_sticky {
                <button class={props.follow_button_class_name.clone()} onclick={scroll_to_target}>
                    { "Scroll to " } { if props.mode == "bottom" { "Bottom" } else { "Top" } }
                </button>
            }
        </div>
    }
}
