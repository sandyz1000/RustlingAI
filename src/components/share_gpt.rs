use yew::prelude::*;
use yewdux::use_store;

use crate::{components::popup_modal::PopupModal, hooks::translation::use_translation, store::ChatSlice};

#[function_component]
pub fn ShareGPT() -> Html {
    let is_modal_open = use_state(|| false);
    let t = use_translation(vec![]);
    let (state, _dispath) = use_store::<ChatSlice>();
    let set_is_modal_open = {
      let is_modal_open = is_modal_open.clone();
      move |e| {
        is_modal_open.set(e);
      }
    };
    let handle_confirm = {
      let chats = state.chats.clone();
      let curr_chat_index = state.curr_chat_index;
      move |e| {
        
      }
    };
    html! {
        <>
      <button
        class="btn btn-neutral"
        onclick={|e| {}}
        aria-label={t("postOnShareGPT.title".to_string(), None)}
      >
        {t("postOnShareGPT.title".to_string(), None)}
      </button>
      if *is_modal_open {
        <PopupModal
          { set_is_modal_open }
          title={t("postOnShareGPT.title".to_string(), None)}
          message={t("postOnShareGPT.warning".to_string(), None)}
          { handle_confirm }
        >
        <></>
        </PopupModal>
      }
    </>
    }
}
