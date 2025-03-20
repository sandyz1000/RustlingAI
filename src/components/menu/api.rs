use yew::prelude::*;
use yewdux::use_store;

use crate::{
    components::{
        icons::{DownChevronArrow, PersonIcon},
        popup_modal::PopupModal,
        trans::Trans,
    }, constants::AVAILABLE_ENDPOINTS, hooks::{
        hide_on_outside::use_hideon_outside_click,
        translation::{use_translation, Namespace},
    }, store::AuthSlice
};

#[function_component]
pub(crate) fn Api() -> Html {
    let t = use_translation(vec![]);
    let is_modal_open = use_state(|| false);
    let set_is_modal_open = {
        let is_modal_open = is_modal_open.clone();
        move |val: bool| {
            is_modal_open.set(val);
        }
    };
    html! {
      <>
        <a
          class="flex py-2 px-2 items-center gap-3 rounded-md hover:bg-gray-500/10 transition-colors duration-200 text-white cursor-pointer text-sm"
          id="api-menu"
          onclick={
            let set_is_modal_open = set_is_modal_open.clone();
            move |_e| set_is_modal_open(true)
          }
        >
          <PersonIcon />
          {t("api".to_string(), None)}
        </a>
        if *is_modal_open {
          <ApiMenu set_is_modal_open={set_is_modal_open} />
        }
      </>
    }
}

#[derive(Debug, Properties, PartialEq)]
pub struct ApiMenuProps {
    pub set_is_modal_open: Callback<bool>,
}

pub const DEFAULT_API_ENDPOINT: &str = "";
#[function_component]
pub fn ApiMenu(ApiMenuProps { set_is_modal_open }: &ApiMenuProps) -> Html {
    let t = use_translation(vec!["main".to_string(), "api".to_string()]);
    let (_state, state_dispatch) = use_store::<AuthSlice>();
    let api_key = use_state(|| "".to_string());
    let api_endpoint = use_state(|| "".to_string());
    let custom_endpoint = use_state(|| false);

    let handle_save = {
        let api_key = api_key.clone();
        let api_endpoint = api_endpoint.clone();
        let set_is_modal_open = set_is_modal_open.clone();
        let _dispatch = state_dispatch.clone();
        move |_e| {
          _dispatch.reduce_mut(|d| {
            d.api_key = Some((*api_key).clone());
            d.api_endpoint = Some((*api_endpoint).clone());
            set_is_modal_open.emit(false);
          });
        }
    };

    let set_api_endpoint = {
        let api_endpoint = api_endpoint.clone();
        move |e: Event| {
            let input = e.target_unchecked_into::<web_sys::HtmlInputElement>();
            api_endpoint.set(input.value());
        }
    };

    let set_api_key_change = {
        let api_key = api_key.clone();
        move |e: Event| {
            let input = e.target_unchecked_into::<web_sys::HtmlInputElement>();
            api_key.set(input.value());
        }
    };

    let toggle_custom_endpoint = {
        let custom_endpoint = custom_endpoint.clone();
        let api_endpoint = api_endpoint.clone();
        move |_e| {
            if *custom_endpoint {
                api_endpoint.set(DEFAULT_API_ENDPOINT.to_string());
            } else {
                api_endpoint.set("".to_string());
            }
            custom_endpoint.set(!(*custom_endpoint));
        }
    };

    html! {
      <PopupModal
        title={t("api".to_string(), None)}
        {set_is_modal_open}
        handle_confirm={handle_save}
      >
        <div class="p-6 border-b border-gray-200 dark:border-gray-600">
          <label class="flex gap-2 text-gray-900 dark:text-gray-300 text-sm items-center mb-4">
            <input
              type="checkbox"
              checked={*custom_endpoint}
              class="w-4 h-4"
              onchange={toggle_custom_endpoint}
            />
            {t("customEndpoint".to_string(), Some(Namespace { ns: "api".to_string() }))}
          </label>

          <div class="flex gap-2 items-center mb-6">
            <div class="min-w-fit text-gray-900 dark:text-gray-300 text-sm">
              {t("apiEndpoint.inputLabel".to_string(), Some(Namespace { ns: "api".to_string() }))}
            </div>
            if *custom_endpoint {
              <input
                type="text"
                class="text-gray-800 dark:text-white p-3 text-sm border-none bg-gray-200 dark:bg-gray-600 rounded-md m-0 w-full mr-0 h-8 focus:outline-none"
                value={api_endpoint.to_string()}
                onchange={set_api_endpoint}
              />
            } else {
              <ApiEndpointSelector api_endpoint={api_endpoint.clone()} />
            }
          </div>

          <div class="flex gap-2 items-center justify-center mt-2">
            <div class="min-w-fit text-gray-900 dark:text-gray-300 text-sm">
              {t("apiKey.inputLabel".to_string(), Some(Namespace { ns: "api".to_string() }))}
            </div>
            <input
              type="text"
              class="text-gray-800 dark:text-white p-3 text-sm border-none bg-gray-200 dark:bg-gray-600 rounded-md m-0 w-full mr-0 h-8 focus:outline-none"
              value={api_key.to_string()}
              onchange={set_api_key_change}
            />
          </div>

          <div class="min-w-fit text-gray-900 dark:text-gray-300 text-sm flex flex-col gap-3 leading-relaxed">
            <p class="mt-4">
              <Trans
              i18n_key="apiKey.howTo"
                ns="api"
                components={vec![
                  html! {
                    <a
                      href="https://platform.openai.com/account/api-keys"
                      class="link"
                      target="_blank"
                    />
                  }
                ]}
              />
            </p>

            <p>{t("securityMessage".to_string(), Some(Namespace { ns: "api".to_string() }))}</p>

            <p>{t("apiEndpoint.description".to_string(), Some(Namespace { ns: "api".to_string() }))}</p>

            <p>{t("apiEndpoint.warn".to_string(), Some(Namespace { ns: "api".to_string() }))}</p>
          </div>
        </div>
      </PopupModal>
    }
}

#[derive(Debug, Properties, PartialEq)]
pub struct EndpointSelectorProps {
    pub api_endpoint: UseStateHandle<String>,
}

#[function_component]
pub(crate) fn ApiEndpointSelector(
    EndpointSelectorProps { api_endpoint }: &EndpointSelectorProps,
) -> Html {
    let (drop_down, drop_down_ref) = use_hideon_outside_click();
    html! {
      <div class="w-[40vw] relative flex-1">
        <button
          class="btn btn-neutral btn-small flex justify-between w-full"
          type="button"
          aria-label="expand api menu"
          onclick={
            let drop_down = drop_down.clone();
            move |_e| drop_down.set(!(*drop_down))
          }
        >
          <span class="truncate">{ api_endpoint.to_string() }</span>
          <DownChevronArrow />
        </button>
        <div
          id="dropdown"
          ref={drop_down_ref}
          class={classes!("absolute", "top-100", "bottom-100", "z-10", "bg-white", "rounded-lg", "shadow-xl", "border-b", "border-black/10", "dark:border-gray-900/50", "text-gray-800", "dark:text-gray-100", "group", "dark:bg-gray-800", "opacity-90", "w-32", "w-full", if *drop_down {""} else {"hidden"}) }
        >
          <ul
            class="text-sm text-gray-700 dark:text-gray-200 p-0 m-0"
            aria-labelledby="dropdownDefaultButton"
          >
            {
              AVAILABLE_ENDPOINTS.iter().map(|endpoint| {
                html! {
                  <li
                    class="px-4 py-2 hover:bg-gray-100 dark:hover:bg-gray-600 dark:hover:text-white cursor-pointer truncate"
                    onclick={
                      let drop_down = drop_down.clone();
                      let api_endpoint = api_endpoint.clone();
                      move |_e| {
                        api_endpoint.set(endpoint.to_string());
                        drop_down.set(false);
                      }
                    }
                    key={endpoint.to_string()}
                  >
                    {endpoint.to_string()}
                  </li>
                }
              }).collect::<Html>()
           }
          </ul>
        </div>
      </div>
    }
}
