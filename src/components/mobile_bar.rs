use crate::{
  components::icons::{MenuIcon, PlusIcon},
  hooks::add_chat::use_add_chat,
  store::{ChatSlice, ConfigSlice},
};
use yew::prelude::*;
use yewdux::prelude::*;

#[function_component]
pub(crate) fn MobileBar() -> Html {
  let (chat_state, _dispath) = use_store::<ChatSlice>();
  let (_, config_dispatch) = use_store::<ConfigSlice>();
  let chat_title = if !chat_state.chats.is_empty()
    && chat_state.curr_chat_index >= 0
    && chat_state.curr_chat_index < chat_state.chats.len() as i32
  {
    chat_state.chats[chat_state.curr_chat_index as usize]
      .title
      .clone()
  } else {
    Some("New Chat".to_string())
  };
  let generating = use_state(|| chat_state.generating);
  let add_chat = use_add_chat();

  html! {
    <div class="sticky top-0 left-0 w-full z-50 flex items-center border-b border-white/20 bg-gray-800 pl-1 pt-1 text-gray-200 sm:pl-3 md:hidden">
      <button
        type="button"
        class="-ml-0.5 -mt-0.5 inline-flex h-10 w-10 items-center justify-center rounded-md hover:text-gray-900 focus:outline-none focus:ring-2 focus:ring-inset focus:ring-white dark:hover:text-white"
        onclick={ 
          let config_dispatch = config_dispatch.clone(); 
          move |_e| config_dispatch.reduce_mut(|f| f.hide_side_menu = false) 
        }
        aria-label="open sidebar"
      >
        <span class="sr-only">{"Open sidebar"}</span>
        <MenuIcon />
      </button>
      <h1 class="flex-1 text-center text-base font-normal px-2 max-h-20 overflow-y-auto">
        {chat_title}
      </h1>
      <button
        type="button"
        class={classes!("px-3", "text-gray-400", "transition-opacity",
        if *generating {"cursor-not-allowed opacity-40"} else {"cursor-pointer opacity-100"}
      )}
        onclick={
          let generating = generating.clone();
          let add_chat = add_chat.clone();
          move |_| if !*generating {
            add_chat(None);
        }}
        aria-label="new chat"
      >
        <PlusIcon class_name={"h-6 w-6"} />
      </button>
    </div>
  }
}
