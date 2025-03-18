use web_sys::HtmlTextAreaElement;
use yew::prelude::*;
use yewdux::use_store;

use crate::{
    components::{
        icons::{CrossIcon, PlusIcon},
        popup_modal::PopupModal,
    },
    hooks::translation::use_translation,
    store::PromptSlice,
    types::chat::Prompt,
};

#[function_component]
pub(crate) fn PromptLibraryMenu() -> Html {
    let t = use_translation(vec![]);
    let is_modal_open = use_state(|| false);
    let set_is_modal_open = {
        let is_modal_open = is_modal_open.clone();
        Callback::from(move |val: bool| {
            is_modal_open.set(val);
        })
    };
    html! {
      <div>
        <button
          class="btn btn-neutral"
          onclick={
            let set_is_modal_open = set_is_modal_open.clone();
            Callback::from(move |_| set_is_modal_open.emit(true))
          }
          aria-label={t("promptLibrary".to_string(), None)}
        >
          {t("promptLibrary".to_string(), None)}
        </button>
        if *is_modal_open {
          <PromptLibraryMenuPopUp { set_is_modal_open } />
        }
      </div>
    }
}

#[derive(Debug, Properties, PartialEq)]
pub(crate) struct PLMenuPopUpProps {
    pub(crate) set_is_modal_open: Callback<bool>,
}

#[function_component]
pub(crate) fn PromptLibraryMenuPopUp(
    PLMenuPopUpProps { set_is_modal_open }: &PLMenuPopUpProps,
) -> Html {
    let t = use_translation(vec![]);

    let (state, _) = use_store::<PromptSlice>();
    let prompts: UseStateHandle<Vec<Prompt>> = use_state(|| vec![]);
    let container = use_node_ref();
    let handle_save = {
        let prompts = prompts.clone();
        let set_is_modal_open = set_is_modal_open.clone();
        let state = state.clone();
        move |e| {
            prompts.set(state.prompts.clone());
            set_is_modal_open.emit(false);
        }
    };

    let add_prompt = {
        let prompts = prompts.clone();
        Callback::from(move |e: MouseEvent| {
            let p = Prompt {
                id: "".to_string(),
                name: "".to_string(),
                prompt: "".to_string(),
            };
            let mut new_prompts = (*prompts).clone();
            new_prompts.push(p);
            prompts.set(new_prompts);
        })
    };

    let delete_prompt = {
        let prompts = prompts.clone();
        Callback::from(move |index: usize| {
            let mut updated_prompts = (*prompts).clone();
            if index < updated_prompts.len() {
                updated_prompts.remove(index);
                prompts.set(updated_prompts);
            }
        })
    };

    fn handle_input(event: InputEvent) {
        if let Some(target) = event.target_dyn_into::<HtmlTextAreaElement>() {
            let scroll_height = target.scroll_height();
            let value = format!("height: 'auto {}px'; max-height: {}px ", &scroll_height, &scroll_height);
            let _ = target.set_attribute("style", &value);

        }
    }

    fn handle_on_focus(event: FocusEvent) {
        if let Some(target) = event.target_dyn_into::<HtmlTextAreaElement>() {
            let scroll_height = target.scroll_height();
            let value = format!("height: 'auto {}px'; max-height: {}px ", &scroll_height, &scroll_height);
            let _ = target.set_attribute("style", &value);
        }
    }

    fn handle_on_blur(event: FocusEvent) {
        if let Some(target) = event.target_dyn_into::<HtmlTextAreaElement>() {
            let value = format!("height: 'auto'; max-height: 2.5rem ");
            let _ = target.set_attribute("style", &value);
        }
    }

    html! {
      <PopupModal
        title={ t("promptLibrary".to_string(), None) }
        set_is_modal_open={set_is_modal_open.clone()}
        handle_confirm={handle_save}
      >
        <div class="p-6 border-b border-gray-200 dark:border-gray-600 w-[90vw] max-w-full text-sm text-gray-900 dark:text-gray-300">
          <div class="border px-4 py-2 rounded border-gray-200 dark:border-gray-600">
            <ImportPrompt />
            <ExportPrompt />
          </div>
          <div class="flex flex-col p-2 max-w-full" ref={container}>
            <div class="flex font-bold border-b border-gray-500/50 mb-1 p-1">
              <div class="sm:w-1/4 max-sm:flex-1">{t("name".to_string(), None)}</div>
              <div class="flex-1">{t("prompt".to_string(), None)}</div>
            </div>
            {
              prompts.iter().enumerate().map(|(index, prompt)| {
                html! {
                  <div
                    key={prompt.id.clone()}
                    class="flex items-center border-b border-gray-500/50 mb-1 p-1"
                  >
                    <div class="sm:w-1/4 max-sm:flex-1">
                      <textarea
                        class="m-0 resize-none rounded-lg bg-transparent overflow-y-hidden leading-7 p-1 focus:ring-1 focus:ring-blue w-full max-h-10 transition-all"
                        onfocus={handle_on_focus}
                        onblur={handle_on_blur}
                        onchange={|e| {}}
                        oninput={handle_input}
                        value={prompt.name.clone()}
                        rows={1}
                      ></textarea>
                    </div>
                    <div class="flex-1">
                      <textarea
                        class="m-0 resize-none rounded-lg bg-transparent overflow-y-hidden leading-7 p-1 focus:ring-1 focus:ring-blue w-full max-h-10 transition-all"
                        onfocus={handle_on_focus}
                        onblur={handle_on_blur}
                        onchange={|e| {}}
                        oninput={handle_input}
                        value={prompt.prompt.clone()}
                        rows={1}
                      ></textarea>
                    </div>
                    <div
                      class="cursor-pointer"
                      onclick={
                        let delete_prompt = delete_prompt.clone();
                        move |_e| delete_prompt.emit(index)
                      }
                    >
                      <CrossIcon />
                    </div>
                  </div>
                }
              }).collect::<Html>()
          }
          </div>
          <div class="flex justify-center cursor-pointer" onclick={add_prompt}>
            <PlusIcon />
          </div>
          <div class="flex justify-center mt-2">
            <div
              class="btn btn-neutral cursor-pointer text-xs"
              onclick={|e| {}}
            >
              {t("clearPrompts".to_string(), None)}
            </div>
          </div>
          <div class="mt-6 px-2">
            {t("morePrompts".to_string(), None)}
            <a
              href="https://github.com/f/awesome-chatgpt-prompts"
              target="_blank"
              class="link"
            >
              {"awesome-chatgpt-prompts"}
            </a>
          </div>
        </div>
      </PopupModal>
    }
}

struct Alert {
    message: String,
    success: bool,
}

#[function_component]
pub(crate) fn ImportPrompt() -> Html {
    let t = use_translation(vec![]);
    let alert = use_state(|| None::<Alert>);
    let input_ref = use_node_ref();
    let handle_file_upload = {
        let input_ref = input_ref.clone();
        |e| {}
    };
    html! {
      <div>
        <label class="block mb-2 text-sm font-medium text-gray-900 dark:text-gray-300">
          {format!("{} {}", t("import".to_string(), None), "(CSV)")}
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
          {t("import".to_string(), None)}
        </button>
        if alert.is_some() {
          <div class={classes!("relative", "py-2", "px-3", "w-full", "mt-3", "border", "rounded-md", "text-gray-600", "dark:text-gray-100", "text-sm", "whitespace-pre-wrap", if alert.as_ref().unwrap().success {"border-green-500 bg-green-500/10"} else {"border-red-500 bg-red-500/10"})} >
            {alert.as_ref().unwrap().message.clone()}
          </div>
        }
      </div>
    }
}

#[function_component]
pub(crate) fn ExportPrompt() -> Html {
    let t = use_translation(vec![]);
    let (state, _) = use_store::<PromptSlice>();
    html! {
      <div class="mt-4">
        <div class="block mb-2 text-sm font-medium text-gray-900 dark:text-gray-300">
          {format!("{} {}", t("export".to_string(), None), "(CSV)") }
        </div>
        <button
          class="btn btn-small btn-primary"
          onclick={|e| {}}
          aria-label={ t("export".to_string(), None) }
        >
          {t("export".to_string(), None)}
        </button>
      </div>
    }
}
