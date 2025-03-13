use yew::prelude::*;

use crate::components::icons::DownChevronArrow;

#[function_component]
pub(crate) fn LanguageSelector() -> Html {
  let dropdown = use_state(|| false);
  let selectableLanguages: Vec<String> = vec![];
  let languageCodeToName: Vec<String> = vec![];
  html! {
    <div class="prose dark:prose-invert relative">
      <button
        class="btn btn-neutral btn-small w-36 flex justify-between"
        type="button"
        onclick={|e| { dropdown.set(!*dropdown); }}
        aria-label="language selector"
      >
        // {languageCodeToName[i18n.language as keyof typeof languageCodeToName] ??
        //   i18n.language}
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
            selectableLanguages.into_iter().map(|lang| {
              html! {
                <li
                  class="px-4 py-2 hover:bg-gray-100 dark:hover:bg-gray-600 dark:hover:text-white cursor-pointer"
                  onclick={ |e| {
                    // i18n.changeLanguage(lang);
                    // dropdown.set(false);
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
