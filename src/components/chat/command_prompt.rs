use yew::prelude::*;
use yewdux::prelude::*;

use crate::{hooks::{hide_on_outside::use_hideon_outside_click, translation::use_translation}, store::PromptSlice};

#[derive(Debug, Properties, PartialEq)]
pub struct Props {
    pub set_content: Callback<String>
}

#[function_component]
pub fn CommandPrompt(Props {set_content}: &Props) -> Html {
    let t  = use_translation(vec![]);
    let (store, _) = use_store::<PromptSlice>();
    let prompts = use_state(|| store.prompts.clone());
    let myinput = use_state(|| "".to_string());
    let (dropdown, dropdown_ref) = use_hideon_outside_click();
    let input_ref = use_node_ref();
    let on_change = {
        let myinput = myinput.clone(); 
        move |e: Event| if let Some(input_element) = e.target_dyn_into::<web_sys::HtmlInputElement>() {
            let value = input_element.value();
            myinput.set(value);
          }
    };
    html! {
        <div class="relative max-wd-sm" ref={dropdown_ref}>
        <button
            class="btn btn-neutral btn-small"
            aria-label="prompt library"
            onclick={ let dropdown = dropdown.clone(); move |_e| dropdown.set(!*dropdown)}
        >
        {"/"}
        </button>
        <div
        class={classes!("absolute", "top-100", "bottom-100", "right-0", "z-10", "bg-white", "rounded-lg", "shadow-xl", "border-b", "border-black/10", "dark:border-gray-900/50", "text-gray-800", "dark:text-gray-100", "group", "dark:bg-gray-800", "opacity-90", if *dropdown {""} else {"hidden"})}
      >
        <div class="text-sm px-4 py-2 w-max">{t("promptLibrary".to_string(), None)}</div>
        <input
          ref={input_ref}
          type="text"
          class="text-gray-800 dark:text-white p-3 text-sm border-none bg-gray-200 dark:bg-gray-600 m-0 w-full mr-0 h-8 focus:outline-none"
          value={ (*myinput).clone() }
          placeholder={t("search".to_string(), None)}
          onchange={ on_change }
        />
        <ul class="text-sm text-gray-700 dark:text-gray-200 p-0 m-0 w-max max-w-sm max-md:max-w-[90vw] max-h-32 overflow-auto">
          {
            prompts.iter().map(|cp| {
                html! {
                    <li
                class="px-4 py-2 hover:bg-gray-100 dark:hover:bg-gray-600 dark:hover:text-white cursor-pointer text-start w-full"
                onclick={
                    let dropdown = dropdown.clone();
                    let set_content= set_content.clone();
                    move |_e| {
                        // TODO: Fix here
                        // _setContent((prev) => prev + cp.prompt);
                        dropdown.set(false);
                    }
                }
                key={cp.id.clone()}
            >
              {cp.name.clone()}
            </li>
                }
            }).collect::<Html>()
          }
        </ul>
      </div>
    </div>
    }
}
