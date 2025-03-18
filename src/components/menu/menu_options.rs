use std::collections::HashMap;

use yew::prelude::*;
use yewdux::prelude::*;
use crate::components::icons::{ArrowBottom, DeleteIcon, LogoutIcon, MoonIcon, PersonIcon, SunIcon};
use crate::components::import_export_chat::ImportExportChat;
use crate::components::menu::about::AboutMenu;
use crate::components::menu::api::Api;
use crate::components::menu::settings::{SettingsMenu, TotalTokenCostDisplay};

use crate::components::popup_modal::PopupModal;
use crate::hooks::translation::use_translation;
use crate::store::{ChatSlice, ConfigSlice};
use crate::types::chat::Theme;

#[function_component]
pub(crate) fn MenuOptions() -> Html {
  let (state, _dispatch) = use_store::<ConfigSlice>();
  let google_client_id = None::<String>;
  html! {
    <>
      <CollapseOptions />
      <div
        class={classes!("overflow-hidden","transition-all", if state.hide_menu_options {"max-h-0"} else {"max-h-full"})}
      >
        if state.count_total_tokens {
          <TotalTokenCostDisplay />
        }
        if let Some(id) = google_client_id {
          // <GoogleSync clientId={id.clone()} />
        }
        <AboutMenu />
        <ImportExportChat />
        <Api />
        <SettingsMenu />
      </div>
    </>
  }
}
#[function_component]
pub(crate) fn Logout() -> Html {
  html! {
    <a class="flex py-3 px-3 items-center gap-3 rounded-md hover:bg-gray-500/10 transition-colors duration-200 text-white cursor-pointer text-sm">
      <LogoutIcon />
      {"Log out"}
    </a>
  }
}
#[function_component]
pub(crate) fn Account() -> Html {
  html! {
    <a className="flex py-3 px-3 items-center gap-3 rounded-md hover:bg-gray-500/10 transition-colors duration-200 text-white cursor-pointer text-sm">
      <PersonIcon />
      {"My account"}
    </a>
  }
}

#[function_component]
fn CollapseOptions() -> Html {
  let (state, _dispatch) = use_store::<ConfigSlice>();

  html! {
    <div
      class="fill-white hover:bg-gray-500/10 transition-colors duration-200 px-3 rounded-md cursor-pointer flex justify-center"
      onclick={|e| {}}
    >
      <ArrowBottom class_name={ classes!("h-3", "w-3", "transition-all", "duration-100", if state.hide_menu_options {"rotate-180"} else {""})}
      />
    </div>
  }
}

#[function_component]
pub fn ClearConversation() -> Html {
  let is_modal_open = use_state(|| false);
  let (_state, dispath) = use_store::<ChatSlice>();
  let t = use_translation(vec![]);
  let handle_confirm = {
    let is_modal_open = is_modal_open.clone();
    let dispath = dispath.clone();
    move |_e| {
      is_modal_open.set(false);
      dispath.reduce_mut(|s| s.folders = HashMap::new());
    }
  };
  let set_is_modal_open = {
    let is_modal_open = is_modal_open.clone();
    move |value: bool| {
      is_modal_open.set(value);
    }
  };

  html! {
    <>
      <button
        class="btn btn-neutral"
        onclick={
          let is_modal_open = is_modal_open.clone();
          move |_e| is_modal_open.set(false)
        }
        aria-label={t("clearConversation".to_string(), None)}
      >
        <DeleteIcon />
        {t("clearConversation".to_string(), None)}
      </button>
      if *is_modal_open {
        <PopupModal
          set_is_modal_open={ set_is_modal_open }
          title={t("warning".to_string(), None)}
          message={Some(t("clearConversationWarning".to_string(), None))}
          handle_confirm={ handle_confirm }
        >
        <></>
        </PopupModal>
      }
    </>
  }
}

#[function_component]
pub fn ThemeSwitcher() -> Html {
  fn get_theme(theme: &Theme) -> Theme {
    match theme {
        Theme::Light => Theme::Dark,
        Theme::Dark => Theme::Light,
    }
  }
  let t  = use_translation(vec![]);
  let (config, _dispatch) = use_store::<ConfigSlice>();
  let switch_theme = {
    let _dispatch = _dispatch.clone();
    move |_e| {
      _dispatch.reduce_mut(|s| s.theme = get_theme(&s.theme))
    }
  };
  html! {
    <button
      class="items-center gap-3 btn btn-neutral"
      onclick={switch_theme}
      aria-label="toggle dark/light mode"
    >
      if config.theme == Theme::Dark {
          <SunIcon /> 
      } else {
        <MoonIcon />
      }
      {t(get_theme(&config.theme).to_string(), None)}
    </button>
  }
}
