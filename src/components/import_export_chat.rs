use std::fmt::Debug;

use crate::components::icons::ExportIcon;
use crate::components::popup_modal::PopupModal;
use crate::hooks::translation::use_translation;
use crate::store::{ChatSlice, ToastSlice, ToastStatus};
use crate::types::export::{ExportBase, ExportV1, OpenAIChat};
use crate::utils::{download_file, get_today, import_openai_chat_export};
use wasm_bindgen::{JsCast, JsValue};
use web_sys::{FileReader, HtmlInputElement, ProgressEvent};
use yew::prelude::*;
use yewdux::use_store;

#[function_component]
pub(crate) fn ImportExportChat() -> Html {
    let t = use_translation(vec![]);
    let is_modal_open = use_state(|| false);
    let set_is_modal_open = {
        let is_modal_open = is_modal_open.clone();
        Callback::from(move |value: bool| {
            is_modal_open.set(value);
        })
    };
    html! {
      <>
        <a
          class="flex py-2 px-2 items-center gap-3 rounded-md hover:bg-gray-500/10 transition-colors duration-200 text-white cursor-pointer text-sm"
          onclick={|e| {} }
        >
          <ExportIcon />
          {format!("{} / {}", t("import".to_string(), None), t("export".to_string(), None))}
        </a>
        if *is_modal_open {
          <PopupModal
            title={format!("{} / {}", t("import".to_string(), None), t("export".to_string(), None))}
            set_is_modal_open={set_is_modal_open.clone()}
            cancel_button={false}
          >
            <div class="p-6 border-b border-gray-200 dark:border-gray-600">
              <ImportChat />
              <ExportChat />
              <div class="border-t my-3 border-gray-200 dark:border-gray-600" />
              <ImportChatOpenAI {set_is_modal_open} />
            </div>
          </PopupModal>
        }
      </>
    }
}

struct Alert {
    message: String,
    success: bool,
}

use serde_wasm_bindgen::from_value;
fn deserialize_js_value<T: serde::de::DeserializeOwned>(
    js_value: JsValue,
) -> Result<T, serde_wasm_bindgen::Error> {
    from_value::<T>(js_value)
}

#[function_component]
pub(crate) fn ImportChat() -> Html {
    let t = use_translation(vec![]);
    let alert = use_state(|| None::<Alert>);
    let input_ref = use_node_ref();
    let (store, dispatch) = use_store::<ChatSlice>();

    let handle_file_upload = {
        let input_ref = input_ref.clone();
        let store = store.clone();
        let alert = alert.clone();
        let dispatch = dispatch.clone();
        move |_e| {
            let input = input_ref.cast::<web_sys::HtmlInputElement>().unwrap();
            if input.files().is_none() {
                return;
            }

            if let Some(file) = input.files().unwrap().get(0) {
                let reader = FileReader::new().unwrap();
                let onload = {
                    let reader = reader.clone();
                    let alert = alert.clone();
                    let store = store.clone();
                    let dispatch = dispatch.clone();
                    wasm_bindgen::closure::Closure::wrap(Box::new(
                        move |e: web_sys::ProgressEvent| {
                            let parsed_data =
                                deserialize_js_value::<ExportV1>(reader.result().unwrap());
                            if let Err(e) = parsed_data {
                                alert.set(Some(Alert {
                                    message: format!("Parse error: {}", e),
                                    success: false,
                                }));
                                return;
                            }
                            let parsed_data = parsed_data.ok().unwrap();
                            match parsed_data.base.version {
                                1 => {
                                    let mut current_folders = store.folders.clone();
                                    let offset = parsed_data.folders.len() as i32;

                                    for folder in current_folders.values_mut() {
                                        folder.order += offset;
                                    }

                                    let merged_folders = parsed_data
                                        .folders
                                        .into_iter()
                                        .chain(current_folders)
                                        .collect();
                                    dispatch.reduce_mut(|d| d.folders = merged_folders);

                                    // Merge chats
                                    let current_chats = store.chats.clone();
                                    let merged_chats = parsed_data
                                        .chats
                                        .into_iter()
                                        .chain(current_chats)
                                        .collect();
                                    dispatch.reduce_mut(|f| f.chats = merged_chats);

                                    alert.set(Some(Alert {
                                        message: "Successfully imported!".into(),
                                        success: true,
                                    }));
                                }
                                _ => {
                                    alert.set(Some(Alert {
                                        message: "Unsupported version".into(),
                                        success: false,
                                    }));
                                }
                            }
                        },
                    )
                        as Box<dyn FnMut(web_sys::ProgressEvent)>)
                };

                reader.set_onload(Some(onload.as_ref().unchecked_ref()));
                let _ = reader.read_as_text(&file);
                onload.forget();

                let _ = reader.read_as_text(&file);
            }
        }
    };

    html! {
      <>
      <label class="block mb-2 text-sm font-medium text-gray-900 dark:text-gray-300">
        {format!("{} {}", t("import".to_string(), None), "(JSON)")}
      </label>
      <input
        class="w-full text-sm file:p-2 text-gray-800 file:text-gray-700 dark:text-gray-300 dark:file:text-gray-200 rounded-md cursor-pointer focus:outline-none bg-gray-50 file:bg-gray-100 dark:bg-gray-800 dark:file:bg-gray-700 file:border-0 border border-gray-300 dark:border-gray-600 placeholder-gray-900 dark:placeholder-gray-300 file:cursor-pointer"
        type="file"
        ref={input_ref}
      />
      <button
        class="btn btn-small btn-primary mt-3"
        onclick={handle_file_upload}
        aria-label={t("import".to_string(), None)}
      >
        { t("import".to_string(), None) }
      </button>
      if alert.is_some() {
          <div
              class={classes!("relative", "py-2", "px-3", "w-full", "mt-3", "border", "rounded-md", "text-gray-600", "dark:text-gray-100", "text-sm", "whitespace-pre-wrap", if alert.as_ref().unwrap().success {"border-green-500 bg-green-500/10"} else {"border-red-500 bg-red-500/10"}) }
          >
              {alert.as_ref().unwrap().message.clone()}
          </div>
      }
    </>
    }
}

#[function_component]
pub(crate) fn ExportChat() -> Html {
    let t = use_translation(vec![]);
    let (store, _) = use_store::<ChatSlice>();

    let on_click = {
        let store = store.clone();
        move |_| {
            let file_data = ExportV1 {
                chats: store.chats.clone(),
                folders: store.folders.clone(),
                base: ExportBase { version: 1 },
            };
            download_file(&file_data, get_today());
        }
    };
    html! {
      <div class="mt-6">
        <div class="block mb-2 text-sm font-medium text-gray-900 dark:text-gray-300">
          {format!("{} {}", t("export".to_string(), None), "(JSON)")}
        </div>
        <button
          class="btn btn-small btn-primary"
          onclick={on_click}
          aria-label={ t("export".to_string(), None) }
        >
          {t("export".to_string(), None)}
        </button>
      </div>
    }
}

#[derive(Debug, Properties, PartialEq)]
pub(crate) struct ChatOpenAIProps {
    set_is_modal_open: Callback<bool>,
}

#[function_component]
pub(crate) fn ImportChatOpenAI(ChatOpenAIProps { set_is_modal_open }: &ChatOpenAIProps) -> Html {
    let t = use_translation(vec![]);
    let input_ref = use_node_ref();
    let (_toast, toast_dispatch) = use_store::<ToastSlice>();
    let (_, chat_dispatch) = use_store::<ChatSlice>();
    let handle_file_upload = {
        let input_ref = input_ref.clone();
        // let toast = toast.clone();
        let toast_dispatch = toast_dispatch.clone();
        let chat_dispatch = chat_dispatch.clone();
        let set_is_modal_open = set_is_modal_open.clone();
        move |_e| {
            let input_element = input_ref.cast::<HtmlInputElement>();
            let toast_dispatch = toast_dispatch.clone();
            let chat_dispatch = chat_dispatch.clone();
            let reader: FileReader = web_sys::FileReader::new().unwrap();
            if input_element.is_none() {
                return;
            }
            let file_reader = reader.clone();
            let set_is_modal_open = set_is_modal_open.clone();
            let file = input_element.unwrap().files().unwrap().get(0);
            if file.is_none() {
                return;
            }
            let file = file.unwrap();
            let onload = {
                wasm_bindgen::closure::Closure::wrap(Box::new(move |e: ProgressEvent| {
                    let parsed_data =
                        deserialize_js_value::<Vec<OpenAIChat>>(file_reader.result().unwrap());

                    if let Err(e) = parsed_data {
                        log::error!("Error:");
                        toast_dispatch.reduce_mut(|d| {
                            d.status = ToastStatus::Error;
                            d.message = "Invalid format!".to_string();
                            d.show = true;
                        });

                        return;
                    }
                    let chats = import_openai_chat_export(parsed_data.unwrap());
                    chat_dispatch.reduce_mut(|c| c.chats.extend_from_slice(&chats));

                    toast_dispatch.reduce_mut(|d| {
                        d.status = ToastStatus::Success;
                        d.message = "Imported successfully!".to_string();
                        d.show = true;
                    });
                    set_is_modal_open.emit(false);
                })
                    as Box<dyn FnMut(ProgressEvent)>)
            };
            reader.set_onload(Some(onload.as_ref().unchecked_ref()));

            let _ = reader.read_as_text(&file);
            onload.forget();
        }
    };
    html! {
        <>
      <div class="text-lg font-bold text-gray-900 dark:text-gray-300 text-center mb-3">
        {format!("{} {} {}", t("import".to_string(), None), "OpenAI ChatGPT", t("export".to_string(), None))}
      </div>
      <label class="block mb-2 text-sm font-medium text-gray-900 dark:text-gray-300">
        {format!("{} {}", t("import".to_string(), None), "(JSON)")}
      </label>
      <input
        class="w-full text-sm file:p-2 text-gray-800 file:text-gray-700 dark:text-gray-300 dark:file:text-gray-200 rounded-md cursor-pointer focus:outline-none bg-gray-50 file:bg-gray-100 dark:bg-gray-800 dark:file:bg-gray-700 file:border-0 border border-gray-300 dark:border-gray-600 placeholder-gray-900 dark:placeholder-gray-300 file:cursor-pointer"
        type="file"
        ref={input_ref}
      />
      <button
        class="btn btn-small btn-primary mt-3"
        onclick={ handle_file_upload }
        aria-label={ t("import".to_string(), None) }
      >
        { t("import".to_string(), None) }
      </button>
    </>
    }
}
