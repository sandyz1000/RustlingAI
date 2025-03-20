use yew::prelude::*;
use yew_hooks::prelude::*;
use yewdux::use_store;

use crate::{components::search_bar::SearchBar, store::ChatSlice};
#[derive(Debug, Properties, PartialEq)]
pub struct ChatSearchProp {
    pub filter: UseStateHandle<String>,
}

#[function_component]
pub fn ChatSearch(ChatSearchProp { filter }: &ChatSearchProp) -> Html {
    let filter = filter.clone();
    let (state, _) = use_store::<ChatSlice>();
    let generating = state.generating;
    let value = use_state(|| "".to_string());
    let handle_change = {
        let filter = filter.clone();
        move |e| filter.set(e)
    };
    // TODO: Revisit this impl
    let debounced = {
        let value = value.clone();
        let filter = filter.clone();
        use_debounce(
            move || {
                filter.set((*value).clone());
            },
            500,
        )
    };

    use_effect(move || debounced.run());

    html! {
        <SearchBar
            value={filter.to_string()}
            {handle_change}
            class_name="h-8 mb-2"
            disabled={generating}
        />
    }
}
