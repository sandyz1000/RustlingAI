use wasm_bindgen::JsCast;
use yew::prelude::*;
use yewdux::use_store;
use crate::components::icons::CrossIcon;
use crate::components::popup_modal::PopupModal;
use crate::components::trans::Trans;
use crate::hooks::translation::{use_translation, Namespace};
use crate::store::slice::AuthSlice;

#[function_component]
fn ApiPopup() -> Html {
  let  translation  = use_translation(vec!["main".to_string(), "api".to_string()]);

  let (state, state_dispatch) = use_store::<AuthSlice>();
  let first_visit = use_state(|| false);
  let api_key = use_state(|| state.api_key.clone());
  let is_modal_open = use_state(|| state.api_key.is_none() && state.first_version );
  let error = use_state(|| "".to_string());

  let handle_confirm = {
    let api_key = api_key.clone();
    let error = error.clone();
    // let is_modal_open = is_modal_open.clone();
    let state_dispatch = state_dispatch.clone();
    let translation = translation.clone();
    move |_e| {
      if api_key.is_none() {
        error.set(translation("noApiKeyWarning".to_string(), Some(Namespace { ns: "api".to_string() })));
      } else {
        error.set("".to_string());
        state_dispatch.reduce_mut(|s| {
          if let Some(apikey) = &*api_key {
            s.api_key = Some(apikey.clone());
          }
        })
      }
    }
    
  };

  {
    let first_visit = first_visit.clone();
    use_effect(move || { first_visit.set(false); });
  }

  if *is_modal_open {
    return html! {
      <></>
    }
  }
  
  let change_api_key = {
    let api_key=  api_key.clone();
    move |e: Event| {
      let input = e.target_unchecked_into::<web_sys::HtmlInputElement>();
      api_key.set(Some(input.value()));
    }
  };

  let on_click = {
    let is_modal_open = is_modal_open.clone();
    Callback::from(move |_| {
        is_modal_open.set(false);
        if let Some(api_menu) = gloo_utils::document().get_element_by_id("api-menu") {
            let _ = api_menu.dyn_ref::<web_sys::HtmlElement>().unwrap().click();
        }
    })
};

  html! {
    <PopupModal
      title="Setup your API key"
      {handle_confirm}
      set_is_modal_open = {
        let is_modal_open = is_modal_open.clone();
        move |_e| {
          is_modal_open.set(!*is_modal_open);
        }
      }
      cancel_button={false}
    >
      <div class="p-6 border-b border-gray-200 dark:border-gray-600">
        <div class="flex gap-2 items-center justify-center mt-2">
          <div class="min-w-fit text-gray-900 dark:text-gray-300 text-sm">
            {translation("apiKey.inputLabel".to_string(), Some(Namespace{ ns: "api".to_string() }))}
          </div>
          <input
            type="text"
            class="text-gray-800 dark:text-white p-3 text-sm border-none bg-gray-200 dark:bg-gray-600 rounded-md m-0 w-full mr-0 h-8 focus:outline-none"
            value={api_key.as_ref().unwrap().clone()}
            onchange={ change_api_key }
          />
        </div>

        <div class="min-w-fit text-gray-900 dark:text-gray-300 text-sm mt-4">
          <Trans
          i18n_key="apiKey.howTo"
            ns="api"
            components={vec![
              html!{ 
                <a
                href="https://platform.openai.com/account/api-keys"
                class="link"
                target="_blank"
              />
              }
            ]}
          />
        </div>
        <div class="min-w-fit text-gray-900 dark:text-gray-300 text-sm mt-4">
          <Trans
          i18n_key="advancedConfig"
            ns="api"
            components={vec![
              html! {
                html! {
                  <a
                  class="link cursor-pointer"
                  onclick={ on_click }
                />
                }
              }
            ]}
          />
        </div>

        <div class="min-w-fit text-gray-900 dark:text-gray-300 text-sm mt-4">
          {translation("securityMessage".to_string(), Some(Namespace { ns: "api".to_string() }))}
        </div>

        if !error.is_empty() {
          <div class="relative py-2 px-3 w-full mt-3 border rounded-md border-red-500 bg-red-500/10">
            <div class="text-gray-600 dark:text-gray-100 text-sm whitespace-pre-wrap">
              {error.to_string()}
            </div>
            <div
              class="text-white absolute top-1 right-1 cursor-pointer"
              onclick={ let error= error.clone(); move |_e| error.set("".to_string())}
            >
              <CrossIcon />
            </div>
          </div>
        }
      </div>
    </PopupModal>
  }

}

