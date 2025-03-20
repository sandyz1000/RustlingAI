use std::collections::HashMap;

use yew::prelude::*;

use crate::{
  components::icons::DownChevronArrow, constants::SELECTABLE_LANGUAGES,
  hooks::translation::use_translation,
};

#[function_component]
pub(crate) fn LanguageSelector() -> Html {
  let i18n = use_translation(vec![]);
  let dropdown = use_state(|| false);
  let lang_code_to_name: HashMap<String, String> = HashMap::new();

  fn get_language_name(lang_code_to_name: &HashMap<String, String>, language: &str) -> String {
    lang_code_to_name
      .get(language)
      .cloned() // Get the value as an owned `String`
      .unwrap_or_else(|| language.to_string()) // Fallback to the language code if not found
  }

  html! {
    <div class="prose dark:prose-invert relative">
      <button
        class="btn btn-neutral btn-small w-36 flex justify-between"
        type="button"
        onclick={ let dropdown=dropdown.clone(); move |_| { dropdown.set(!*dropdown); }}
        aria-label="language selector"
      >
        // { get_language_name(&lang_code_to_name,  i18n.language) }
        <DownChevronArrow />
      </button>
      <div
        id="dropdown"
        class={classes!("absolute", "top-100", "bottom-100", "z-10", "bg-white", "rounded-lg", "shadow-xl", "border-b", "border-black/10", "dark:border-gray-900/50", "text-gray-800", "dark:text-gray-100", "group", "dark:bg-gray-800", "opacity-90", "w-36", if *dropdown {""} else {"hidden"})}
      >
        <ul
          class="text-sm text-gray-700 dark:text-gray-200 p-0 m-0 max-h-72 overflow-auto"
          aria-labelledby="dropdownDefaultButton"
        >
          {
            SELECTABLE_LANGUAGES.into_iter().map(|lang| {
              html! {
                <li
                  class="px-4 py-2 hover:bg-gray-100 dark:hover:bg-gray-600 dark:hover:text-white cursor-pointer"
                  onclick={
                    let dropdown = dropdown.clone();
                    move |_e| {
                      // i18n.changeLanguage(lang);
                      dropdown.set(false);
                  }}
                  key={lang.to_string()}
                  lang={lang.to_string()}
                >
                  // {languageCodeToName[lang]}
                </li>
              }
            }).collect::<Html>()
          }
        </ul>
      </div>
    </div>
  }
}
