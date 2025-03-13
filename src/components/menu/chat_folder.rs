use crate::components::icons::{
    ColorPaletteIcon, CrossIcon, DeleteIcon, DownChevronArrow, EditIcon, FolderIcon, PlusIcon,
    RefreshIcon, TickIcon,
};
use crate::components::menu::chat_history::ChatHistory;
use crate::constants::FOLDER_COLOR_OPTIONS;
use crate::hooks::add_chat::use_add_chat;
use crate::hooks::translation::use_translation;
use crate::{
    hooks::hide_on_outside::use_hideon_outside_click, store::slice::ChatSlice,
    types::chat::ChatHistoryInterface,
};
use yew::prelude::*;
use yewdux::use_store;

#[derive(Debug, Properties, PartialEq)]
pub struct ChatFolderProps {
    folder_id: String,
    folder_chats: Vec<ChatHistoryInterface>,
}
#[function_component]
pub fn ChatFolder(
    ChatFolderProps {
        folder_id,
        folder_chats,
    }: &ChatFolderProps,
) -> Html {
    let (state, _dispatch) = use_store::<ChatSlice>();
    let _folder_name = state.folders[folder_id].name.clone();
    let folder_color = state.folders[folder_id].color.clone();
    let _is_expanded = state.folders[folder_id].expanded;
    let folderChats: Vec<ChatHistoryInterface> = vec![];
    let input_ref = use_node_ref();
    let folder_ref = use_node_ref();
    let gradient_ref = use_node_ref();
    let folder_name = use_state(|| _folder_name);
    let is_edit = use_state(|| false);
    let is_delete = use_state(|| false);
    let is_hover = use_state(|| false);
    let (pallete, pallete_ref) = use_hideon_outside_click();

    let edit_title = {
        let _dispatch = _dispatch.clone();
        let is_edit = is_edit.clone();
        let folder_name = folder_name.clone();
        move || {
            _dispatch.reduce_mut(|d| {
                if let Some(f) = d.folders.get_mut(folder_id) {
                    f.name = (*folder_name).clone();
                }
            });
            is_edit.set(false);
        }
    };
    let delete_folder = {
        let _dispatch = _dispatch.clone();
        let is_delete = is_delete.clone();
        move || {
            _dispatch.reduce_mut(|d| d.folders.remove(folder_id));
            is_delete.set(true);
        }
    };
    let update_color = {
        let _dispatch = _dispatch.clone();
        let pallete = pallete.clone();
        move |color: Option<String>| {
            _dispatch.reduce_mut(|s| {
                if color.is_some() {
                  if let Some(c) = s.folders.get_mut(folder_id) {
                    c.color = color;
                  }
                } else {
                    s.folders.remove(folder_id);
                }
            });
        }
    };
    let onclick_fn = {
        let update_color = update_color.clone();
        let color = folder_color.clone();
        // Callback::from(move |e| update_color(color))
        |e| {}
    };
    let handle_drop = { |e| {} };

    let handleDragOver = { |e| {} };

    let handleDragLeave = { |e| {} };
    let handleKeyDown = { |e| {} };
    let handleTick = { |e| {} };
    let handleCross = { |e| {} };
    let toggleExpanded = { |e| {} };
    html! {
        <div
      class={classes!("w-full", "transition-colors", "group/folder", if *is_hover {"bg-gray-800/40"} else {""})}
      ondrop={handle_drop}
      ondragover={handleDragOver}
      ondragleave={handleDragLeave}
    >
      <div
        // style={{ background: color || "" }}
        class={classes!("transition-colors", "flex", "py-2", "pl-2", "pr-1", "items-center", "gap-3", "relative", "rounded-md", "break-all", "cursor-pointer", "parent-sibling", if folder_color.clone().is_some()  {""} else {"hover:bg-gray-850"})}
        onclick={toggleExpanded}
        ref={folder_ref}
        onmouseenter={
            |e| {
                // if (color && folderRef.current) {
                //     folderRef.current.style.background = format!("{}dd", _color);
                // }
                // if (gradientRef.current) {
                //     gradientRef.current.style.width = "0px";
                // }
            }
        }
        onmouseleave={
            |e| {
                // if (color && folderRef.current) {
                //     folderRef.current.style.background = color;
                // }
                // if (gradientRef.current){ gradientRef.current.style.width = "1rem";}
            }
        }
      >
        <FolderIcon />
        <div class="flex-1 text-ellipsis max-h-5 overflow-hidden break-all relative">
          if *is_edit {
            <input
              type="text"
              class="focus:outline-blue-600 text-sm border-none bg-transparent p-0 m-0 w-full"
              value={(*folder_name).clone()}
              onchange={ |e| {}}
              onclick={|e| {}}
              onkeydown={handleKeyDown}
              ref={input_ref}
            />
          } else {
            <>
            {(*folder_name).clone()}
            </>
          }
          if *is_edit {
            <div
              ref={gradient_ref}
              class="absolute inset-y-0 right-0 w-4 z-10 transition-all"
                //   style={}
            />
          }
        </div>
        <div
          class="flex text-gray-300"
          onclick={|e| {}}
        >

          if *is_delete || *is_edit {
            <>
              <button
                class="p-1 hover:text-white"
                onclick={handleTick}
                aria-label="confirm"
              >
                <TickIcon />
              </button>
              <button
                class="p-1 hover:text-white"
                onclick={handleCross}
                aria-label="cancel"
              >
                <CrossIcon />
              </button>
            </>
          } else {
            <>
              <div
                class="relative md:hidden group-hover/folder:md:inline"
                ref={pallete_ref}
              >
                <button
                  class="p-1 hover:text-white"
                  onclick={
                    let pallete = pallete.clone();
                    move |e| pallete.set(false)
                  }
                  aria-label="folder color"
                >
                  <ColorPaletteIcon />
                </button>
                if *pallete {
                    <div class="absolute left-0 bottom-0 translate-y-full p-2 z-20 bg-gray-900 rounded border border-gray-600 flex flex-col gap-2 items-center">
                    <>
                    {
                      FOLDER_COLOR_OPTIONS.iter().map(|c| {
                        html! {
                            <button
                          key={*c}
                        //   style={{ background: c }}
                          class="hover:scale-90 transition-transform h-4 w-4 rounded-full"
                          onclick={
                            // let update_color = update_color.clone();
                            // move |e| update_color(Some((*c).to_string()))
                            |e| {}
                          }
                          aria-label={*c}
                        />
                        }
                      }).collect::<Html>()
                    }
                      <button
                        onclick={ onclick_fn}
                        aria-label="default color"
                      >
                        <RefreshIcon />
                      </button>
                    </>
                  </div>
                }


              </div>

              <button
                class="p-1 hover:text-white md:hidden group-hover/folder:md:inline"
                onclick={let is_edit = is_edit.clone(); move |w|  is_edit.set(true)}
                aria-label="edit folder title"
              >
                <EditIcon />
              </button>
              <button
                class="p-1 hover:text-white md:hidden group-hover/folder:md:inline"
                onclick={let is_delete = is_delete.clone(); move |w|  is_delete.set(true)}
                aria-label="delete folder"
              >
                <DeleteIcon />
              </button>
              <button
                class="p-1 hover:text-white"
                onclick={toggleExpanded}
                aria-label="expand folder"
              >
                <DownChevronArrow
                  class_name={classes!("transition-transform", if _is_expanded {"rotate-180"} else {""}) }
                />
              </button>
            </>
          }
        </div>
      </div>
      <div class="ml-3 pl-1 border-l-2 border-gray-700 flex flex-col gap-1 parent">
        if _is_expanded {
            <NewChat folder={Some(folder_id.clone())} />
            {
              folderChats.iter().map(|chat|
                html! {
                    <ChatHistory
                    title={chat.title.clone()}
                    chat_index={chat.index}
                    key={ format!("{}-{}", chat.title, chat.index)}
                    />
                }
              ).collect::<Html>()
            }
        }

      </div>
    </div>
    }
}

#[derive(Debug, PartialEq, Properties)]
struct NewChatProps {
    #[prop_or(None)]
    folder: Option<String>,
}

#[function_component]
pub fn NewChat(NewChatProps { folder }: &NewChatProps) -> Html {
    let folder = folder.clone();
    let t = use_translation(vec![]);
    let (state, _) = use_store::<ChatSlice>();
    let generating = state.generating;
    let add_chat = use_add_chat();
    html! {
        <a
      class={classes!("flex", "flex-1", "items-center", "rounded-md", "hover:bg-gray-500/10", "transition-all", "duration-200", "text-white", "text-sm", "flex-shrink-0", if generating {"cursor-not-allowed opacity-40"} else {"cursor-pointer opacity-100"}, if folder.is_some() {"justify-start"} else {"py-2 px-2 gap-3 mb-2 border border-white/20"}) }
      onclick={
        let add_chat = add_chat.clone();
        let folder = folder.clone();
        move |_e| if !generating { add_chat(folder.clone()) }
        }
      title={if folder.is_some() {t("newChat".to_string(), None)} else {"".to_string()}}
    >
        if folder.is_some() {
            <div class="max-h-0 parent-sibling-hover:max-h-10 hover:max-h-10 parent-sibling-hover:py-2 hover:py-2 px-2 overflow-hidden transition-all duration-200 delay-500 text-sm flex gap-3 items-center text-gray-100">
                <PlusIcon /> {t("newChat".to_string(), None)}
            </div>
        } else {
            <>
            <PlusIcon />
            <span class="inline-flex text-white text-sm">{t("newChat".to_string(), None)}</span>
            </>
        }

    </a>
    }
}
