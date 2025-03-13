use yew::prelude::*;

use crate::hooks::translation::use_translation;

#[derive(Debug, Properties, PartialEq)]
pub(crate) struct SearchBarProps {
    pub(crate) value: String,
    pub(crate) handle_change: Callback<String>,
    pub(crate) class_name: String,
    pub(crate) disabled: bool,
}

#[function_component]
pub(crate) fn SearchBar(
    SearchBarProps {
        value,
        handle_change,
        class_name,
        disabled,
    }: &SearchBarProps,
) -> Html {
    let t = use_translation(vec![]);
    let on_change = {
        let cb = handle_change.clone();
        Callback::from(|e| {
          // cb(e);
        })
    };
    html! {
      <div class={class_name}>
        <input
          disabled={*disabled}
          type="text"
          class="text-gray-800 dark:text-white p-3 text-sm bg-transparent disabled:opacity-40  disabled:cursor-not-allowed transition-opacity m-0 w-full h-full focus:outline-none rounded border border-white/20"
          placeholder={t("search".to_string(), None) }
          value={value.clone()}
          onchange={ on_change }
        />
      </div>
    }
}
