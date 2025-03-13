use wasm_bindgen::JsCast;
use yew::prelude::*;
use yewdux::prelude::*;

use std::{rc::Rc, str::FromStr};

use crate::{
    components::{icons::DownChevronArrow, popup_modal::PopupModal},
    constants::{MODEL_MAX_TOKEN, MODEL_OPTIONS},
    hooks::translation::use_translation,
    types::chat::{ConfigInterface, ModelOptions},
};

#[derive(Debug, Properties, PartialEq)]
pub(crate) struct ConfigMenuProps {
    pub(crate) set_is_modal_open: Callback<bool>,
    pub(crate) config: UseStateHandle<Option<ConfigInterface>>,
}

#[function_component]
pub(crate) fn ConfigMenu(
    ConfigMenuProps {
        set_is_modal_open,
        config,
    }: &ConfigMenuProps,
) -> Html {
    if config.is_none() {
        return html! {
          <></>
        };
    }
    let cfg_inner = config.as_ref().unwrap().clone();
    let max_token = use_state(|| cfg_inner.max_tokens);
    let model: UseStateHandle<ModelOptions> = use_state(|| cfg_inner.model.clone());
    let temperature = use_state(|| cfg_inner.temperature);
    let presence_penalty = use_state(|| cfg_inner.presence_penalty);
    let top_p = use_state(|| cfg_inner.top_p);
    let freq_penalty = use_state(|| cfg_inner.frequency_penalty);
    let xtrans = use_translation(vec!["model".to_string()]);

    let handle_confirm = {
        let config = config.clone();
        let set_is_modal_open = set_is_modal_open.clone();
        move |_e| {
            config.set(Some(ConfigInterface::default()));
            set_is_modal_open.emit(false);
        }
    };

    html! {
      <PopupModal
        title={ xtrans("configuration".to_string(), None) }
        set_is_modal_open={set_is_modal_open}
        handle_confirm={handle_confirm.clone()}
        handle_click_backdrop={handle_confirm.clone()}
      >
        <div class="p-6 border-b border-gray-200 dark:border-gray-600">
          <ModelSelector model={model.clone()}  />
          <MaxTokenSlider {max_token} {model} />
          <TemperatureSlider {temperature} />
          <TopPSlider {top_p} />
          <PresencePenaltySlider {presence_penalty} />
          <FrequencyPenaltySlider {freq_penalty} />
        </div>
      </PopupModal>
    }
}

#[derive(Debug, Properties, PartialEq)]
pub struct FreqPenaltyProps {
    pub freq_penalty: UseStateHandle<i32>,
}

#[function_component]
pub(crate) fn FrequencyPenaltySlider(FreqPenaltyProps { freq_penalty }: &FreqPenaltyProps) -> Html {
    let t = use_translation(vec!["model".to_string()]);
    html! {
      <div class="mt-5 pt-5 border-t border-gray-500">
        <label class="block text-sm font-medium text-gray-900 dark:text-white">
          {format!("{}:{}", t("frequencyPenalty.label".to_string(), None), freq_penalty.to_string()) }
        </label>
        <input
          id="default-range"
          type="range"
          value={freq_penalty.to_string()}
          onchange={
            let freq_penalty = freq_penalty.clone();
                move |e: Event| {
                let input = e.dyn_ref::<web_sys::HtmlInputElement>().unwrap();
                freq_penalty.set(input.value().parse::<i32>().unwrap());
            }
          }
          min={"-2"}
          max={2}
          step={0.1}
          class="w-full h-2 bg-gray-200 rounded-lg appearance-none cursor-pointer"
        />
        <div class="min-w-fit text-gray-500 dark:text-gray-300 text-sm mt-2">
          {t("frequencyPenalty.description".to_string(), None)}
        </div>
      </div>
    }
}

#[derive(Debug, PartialEq, Properties)]
pub(crate) struct TempSliderProps {
    pub temperature: UseStateHandle<i32>,
}

#[function_component]
pub(crate) fn TemperatureSlider(TempSliderProps { temperature }: &TempSliderProps) -> Html {
    let xtrans = use_translation(vec!["model".to_string()]);
    html! {
      <div class="mt-5 pt-5 border-t border-gray-500">
        <label class="block text-sm font-medium text-gray-900 dark:text-white">
          {format!("{}:{}", xtrans("temperature.label".to_string(), None), temperature.to_string()) }
        </label>
        <input
          id="default-range"
          type="range"
          value={(*temperature).to_string()}
          onchange={
                let temperature = temperature.clone();
                move |e: Event| {
                let input = e.dyn_ref::<web_sys::HtmlInputElement>().unwrap();
                temperature.set(input.value().parse::<i32>().unwrap());
            }
        }
          min={0}
          max={2}
          step={0.1}
          class="w-full h-2 bg-gray-200 rounded-lg appearance-none cursor-pointer"
        />
        <div class="min-w-fit text-gray-500 dark:text-gray-300 text-sm mt-2">
          {xtrans("temperature.description".to_string(), None)}
        </div>
      </div>
    }
}

#[derive(Debug, Properties, PartialEq)]
pub(crate) struct TopPSliderProps {
    pub(crate) top_p: UseStateHandle<i32>,
}

#[function_component]
pub(crate) fn TopPSlider(TopPSliderProps { top_p }: &TopPSliderProps) -> Html {
    let t = use_translation(vec!["model".to_string()]);
    html! {
      <div class="mt-5 pt-5 border-t border-gray-500">
        <label class="block text-sm font-medium text-gray-900 dark:text-white">
          {format!("{}: {}", t("topP.label".to_string(), None), top_p.to_string())}
        </label>
        <input
          id="default-range"
          type="range"
          value={top_p.to_string()}
          onchange={
            let top_p = top_p.clone();
                move |e: Event| {
                let input = e.dyn_ref::<web_sys::HtmlInputElement>().unwrap();
                top_p.set(input.value().parse::<i32>().unwrap());
            }
          }
          min={0}
          max={1}
          step={0.05}
          class="w-full h-2 bg-gray-200 rounded-lg appearance-none cursor-pointer"
        />
        <div class="min-w-fit text-gray-500 dark:text-gray-300 text-sm mt-2">
          {t("topP.description".to_string(), None)}
        </div>
      </div>
    }
}

#[derive(Debug, Properties, PartialEq)]
pub(crate) struct PenaltySliderProps {
    pub(crate) presence_penalty: UseStateHandle<i32>,
}

#[function_component]
pub(crate) fn PresencePenaltySlider(
    PenaltySliderProps { presence_penalty }: &PenaltySliderProps,
) -> Html {
    let t = use_translation(vec!["model".to_string()]);
    html! {
      <div class="mt-5 pt-5 border-t border-gray-500">
        <label class="block text-sm font-medium text-gray-900 dark:text-white">
          {format!("{}: {}", t("presencePenalty.label".to_string(), None), presence_penalty.to_string())}
        </label>
        <input
          id="default-range"
          type="range"
          value={(*presence_penalty).to_string()}
          onchange={
            let presence_penalty = presence_penalty.clone();
                move |e: Event| {
                let input = e.dyn_ref::<web_sys::HtmlInputElement>().unwrap();
                presence_penalty.set(input.value().parse::<i32>().unwrap());
            }
          }
          min={"-2".to_string()}
          max={2}
          step={0.1}
          class="w-full h-2 bg-gray-200 rounded-lg appearance-none cursor-pointer"
        />
        <div class="min-w-fit text-gray-500 dark:text-gray-300 text-sm mt-2">
          {t("presencePenalty.description".to_string(), None)}
        </div>
      </div>
    }
}

#[derive(Debug, Properties, PartialEq)]
pub(crate) struct ModelSelectorProps {
    pub(crate) model: UseStateHandle<ModelOptions>,
}

#[function_component]
pub(crate) fn ModelSelector(ModelSelectorProps { model }: &ModelSelectorProps) -> Html {
    let dropdown = use_state(|| false);
    html! {
      <div class="mb-4">
        <button
          class="btn btn-neutral btn-small flex gap-1"
          type="button"
          onclick={let dropdown = dropdown.clone(); move |_e| dropdown.set(!*dropdown)}
          aria-label="model"
        >
          {model.to_string()}
          <DownChevronArrow />
        </button>
        <div
          id="dropdown"
          class={classes!(if *dropdown {""} else {"hidden"}, "absolute", "top-100", "bottom-100", "z-10", "bg-white", "rounded-lg", "shadow-xl", "border-b", "border-black/10", "dark:border-gray-900/50", "text-gray-800", "dark:text-gray-100", "group", "dark:bg-gray-800", "opacity-90")}
        >
          <ul
            class="text-sm text-gray-700 dark:text-gray-200 p-0 m-0"
            aria-labelledby="dropdownDefaultButton"
          >
          {
            MODEL_OPTIONS.iter().map(|m| {
              html! {
                <li
                  class="px-4 py-2 hover:bg-gray-100 dark:hover:bg-gray-600 dark:hover:text-white cursor-pointer"
                  onclick={
                    let model = model.clone();
                    let dropdown = dropdown.clone();
                    move |_e| {
                        model.set(ModelOptions::from_str(m).unwrap());
                        dropdown.set(false);
                    }
                }
                  key={m.to_string()}
                >
                  {m.to_string()}
                </li>
              }
            }).collect::<Html>()
          }
          </ul>
        </div>
      </div>
    }
}

#[derive(Debug, Properties, PartialEq)]
pub(crate) struct TokenSliderProps {
    pub(crate) max_token: UseStateHandle<i32>,
    pub(crate) model: UseStateHandle<ModelOptions>,
}

#[function_component]
pub(crate) fn MaxTokenSlider(TokenSliderProps { max_token, model }: &TokenSliderProps) -> Html {
    let xtrans = use_translation(vec!["model".to_string()]);
    let input_ref = use_node_ref();
    let model_max_token = MODEL_MAX_TOKEN[&*model.to_string()];
    let set_max_token = {
        let max_token = max_token.clone();
        move |e: Event| {
            let input = e.dyn_ref::<web_sys::HtmlInputElement>().unwrap();
            max_token.set(input.value().parse::<i32>().unwrap());
        }
    };
    html! {
      <div>
        <label class="block text-sm font-medium text-gray-900 dark:text-white">
          {format!("{}: {}", xtrans("token.label".to_string(), None), (*max_token).to_string())}
        </label>
        <input
          type="range"
          ref={input_ref}
          value={(*max_token).to_string()}
          onchange={ set_max_token }
          min={0}
          max={model_max_token.to_string()}
          step={1}
          class="w-full h-2 bg-gray-200 rounded-lg appearance-none cursor-pointer"
        />
        <div class="min-w-fit text-gray-500 dark:text-gray-300 text-sm mt-2">
          {xtrans("token.description".to_string(), None)}
        </div>
      </div>
    }
}
