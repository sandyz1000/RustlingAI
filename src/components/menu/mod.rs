pub(crate) mod about;
pub(crate) mod api;
pub(crate) mod chat_config;
pub(crate) mod chat_folder;
pub(crate) mod chat_history;
pub(crate) mod chat_search;
pub(crate) mod config;
pub(crate) mod prompt_library;
pub(crate) mod settings;
pub(crate) mod menu_options;

use std::rc::Rc;

use chat_folder::NewChat;
use gloo::events::EventListener;
use menu_options::MenuOptions;
use yew::prelude::*;
use yewdux::use_store;

use crate::{components::icons::{CrossIcon2, DownArrow, MenuIcon, NewFolderIcon}, store::slice::{ChatSlice, ConfigSlice}, types::chat::Folder};

#[function_component]
pub fn NewFolder() -> Html {
  let (state, _dispatch) = use_store::<ChatSlice>();
  let generating = state.generating;
  let folders = state.folders.clone();
  let addFolder = {
    let mut index = 1;
    let mut name = format!("New Folder {}", index);
    for (_, f) in &folders {
      if f.name == name {
        index += 1;
        name = format!("New Folder {}", index);
      }
    }
    let id = "".to_string();
    let mut folder = Folder { id: id.clone(), name, expanded: false, order: 0, color: None};
    move || {
      _dispatch.reduce_mut(|c| c.folders.insert(id, folder));
    }
  };
    html! {
      <a
      class={classes!("flex", "py-3", "px-3", "items-center", "gap-3", "rounded-md", "hover:bg-gray-500/10", "transition-colors", "duration-200", "text-white", "text-sm", "mb-2", "flex-shrink-0", "border", "border-white/20", "transition-opacity", if generating {"cursor-not-allowed opacity-40"} else {"cursor-pointer opacity-100"}) }
      onclick={let addFolder = addFolder.clone(); |e| if !generating {addFolder()}}
    >
      <NewFolderIcon />
    </a>
    }
}


#[function_component]
pub fn Menu() -> Html {
    fn get_inner_width() -> f64 {
        web_sys::window().unwrap().inner_width().unwrap().as_f64().unwrap()
    }
    let (state, _dispatch) = use_store::<ConfigSlice>();
    let window_width_ref = use_mut_ref(|| get_inner_width());
    let hide_side_menu = use_state(|| state.hide_side_menu);
    
    {
        let hide_side_menu = hide_side_menu.clone();
        let window_width_ref = window_width_ref.clone();

        use_effect(move || {
            // Check initial window width
            let width = get_inner_width();
            if width < 768.0 {
                hide_side_menu.set(true);
            }

            // Create resize event listener
            let listener = Rc::new(EventListener::new(
                &web_sys::window().unwrap(),
                "resize",
                move |_event| {
                    let current_width = get_inner_width();
                    if *window_width_ref.borrow() != current_width && current_width < 768.0 {
                        hide_side_menu.set(true);
                    }
                    *window_width_ref.borrow_mut() = current_width;
                },
            ));

            // Cleanup function to remove the event listener
            || drop(listener)
        });
    }

    html!{
        <>
      <div
        id="menu"
        class={classes!("group/menu", "dark", "bg-gray-900", "fixed", "md:inset-y-0", "md:flex", "md:w-[260px]", "md:flex-col", "transition-transform", "z-[999]", "top-0", "left-0", "h-full", "max-md:w-3/4", if *hide_side_menu {"translate-x-[-100%]"} else {"translate-x-[0%]"}) }
      >
        <div class="flex h-full min-h-0 flex-col">
          <div class="flex h-full w-full flex-1 items-start border-white/20">
            <nav class="flex h-full flex-1 flex-col space-y-1 px-2 pt-2">
              <div class="flex gap-2">
                <NewChat />
                <NewFolder />
              </div>
              <ChatHistoryList />
              <MenuOptions />
            </nav>
          </div>
        </div>
        <div
          id="menu-close"
          class={classes!("md:hidden", "absolute", "z-[999]", "right-0", "translate-x-full", "top-10", "bg-gray-900", "p-2", "cursor-pointer", "hover:bg-black", "text-white", if *hide_side_menu {"hidden"} else {""})}
          onclick={|e| {}}
        >
          <CrossIcon2 />
        </div>
        <div
          class={classes!("group/menu", "md:group-hover/menu:opacity-100", "max-md:hidden", "transition-opacity", "absolute", "z-[999]", "right-0", "translate-x-full", "top-10", "bg-gray-900", "p-2", "cursor-pointer", "hover:bg-black", "text-white", if *hide_side_menu {"opacity-100"} else {"opacity-0 rotate-90"})}
          onclick={|e| {}}
        >
          if *hide_side_menu {
            <MenuIcon />
          } else {
            <DownArrow />
          }
          
        </div>
      </div>
      <div
        id="menu-backdrop"
        class={classes!("md:hidden", "fixed", "top-0", "left-0", "h-full", "w-full", "z-[60]", "bg-gray-900/70", if *hide_side_menu {"hidden"} else {""})}
        onclick={|e| {}}
      />
    </>
    }
}


#[function_component]
pub fn ChatHistoryList() -> Html {
    let isHover = use_state(|| false);
    html! {
        <div
      class={classes!("flex-col flex-1 overflow-y-auto hide-scroll-bar border-b border-white/20", if *isHover {"bg-gray-800/40"} else {""})}
      onDrop={handleDrop}
      onDragOver={handleDragOver}
      onDragLeave={handleDragLeave}
      onDragEnd={handleDragEnd}
    >
      <ChatSearch filter={filter} setFilter={setFilter} />
      <div class="flex flex-col gap-2 text-gray-100 text-sm">
        {Object.keys(chatFolders).map((folderId) => (
          <ChatFolder
            folderChats={chatFolders[folderId]}
            folderId={folderId}
            key={folderId}
          />
        ))}
        {noChatFolders.map(({ title, index, id }) => (
          <ChatHistory title={title} key={`${title}-${id}`} chatIndex={index} />
        ))}
      </div>
      <div class="w-full h-10" />
    </div>
    }
}
