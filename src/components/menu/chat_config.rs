use yew::prelude::*;
use yewdux::use_store;

use crate::{
    components::{chat::{FrequencyPenaltySlider, MaxTokenSlider, ModelSelector, PresencePenaltySlider, TemperatureSlider, TopPSlider}, popup_modal::PopupModal}, constants::DEFAULT_SYSTEM_MESSAGE, hooks::translation::use_translation, store::slice::ConfigSlice, types::chat::{ConfigInterface, ModelOptions}
};

#[function_component]
pub(crate) fn ChatConfigMenu() -> Html {
    let is_modal_open = use_state(|| false);
    let t = use_translation(vec!["model".to_string()]);
    let set_is_modal_open = {
        let is_modal_open = is_modal_open.clone();
        move |e| is_modal_open.set(e)
    };
    html! {
        <div>
      <button
        class="btn btn-neutral"
        onclick={ let set_is_modal_open = set_is_modal_open.clone(); move|e| set_is_modal_open(true) }
        aria-label={t("defaultChatConfig".to_string(), None)}
      >
        {t("defaultChatConfig".to_string(), None)}
      </button>
      if *is_modal_open {
        <ChatConfigPopup { set_is_modal_open } />
      }
    </div>
    }
}

#[derive(Debug, PartialEq, Properties)]
pub struct ConfigPopupProps {
    pub(crate) set_is_modal_open: Callback<bool>,
}

#[function_component]
pub(crate) fn ChatConfigPopup(ConfigPopupProps { set_is_modal_open }: &ConfigPopupProps) -> Html {
    let t = use_translation(vec!["model".to_string()]);
    let (state, _dispath) = use_store::<ConfigSlice>();

    let config = state.default_chat_config.clone();
    let system_message = use_state(|| state.default_system_message.clone());
    let model = use_state(|| ModelOptions::Gpt35Turbo);
    let max_token = use_state(|| config.max_tokens);
    let temperature = use_state(|| config.temperature);
    let top_p = use_state(|| config.top_p);
    let pres_penalty = use_state(|| config.presence_penalty);
    let freq_penalty = use_state(|| config.frequency_penalty);
    let handle_save = {
        let _dipatch = _dispath.clone();
        let model = model.clone();
        let max_tokens = max_token.clone();
        let temp = temperature.clone();
        let top_p = top_p.clone();
        let pres_penalty = pres_penalty.clone();
        let freq_penalty = freq_penalty.clone();
        let system_message = system_message.clone();
        move |val| {
            _dipatch.reduce_mut(|c| {
                c.default_chat_config = ConfigInterface {
                    model: (*model).clone(),
                    max_tokens: *max_tokens,
                    temperature: *temp,
                    presence_penalty: *pres_penalty,
                    top_p: *top_p,
                    frequency_penalty: *freq_penalty,
                };
                
                c.default_system_message = (*system_message).clone();
            });
        }
    };
    let handle_reset = {
        let model = model.clone();
        let max_tokens = max_token.clone();
        let temp = temperature.clone();
        let top_p = top_p.clone();
        let pres_penalty = pres_penalty.clone();
        let freq_penalty = freq_penalty.clone();
        let system_message = system_message.clone();
        move |e| {
            let default_chat_config = ConfigInterface::default();
            model.set(default_chat_config.model);
            max_tokens.set(default_chat_config.max_tokens);
            temp.set(default_chat_config.temperature);
            top_p.set(default_chat_config.top_p);
            pres_penalty.set(default_chat_config.presence_penalty);
            freq_penalty.set(default_chat_config.frequency_penalty);
            system_message.set(DEFAULT_SYSTEM_MESSAGE.to_string());
        }
    };
    html! {
        <PopupModal
      title={t("defaultChatConfig".to_string(), None)}
      {set_is_modal_open}
      handle_confirm={handle_save}
    >
      <div class="p-6 border-b border-gray-200 dark:border-gray-600 w-[90vw] max-w-full text-sm text-gray-900 dark:text-gray-300">
        <DefaultSystemChat {system_message} />
        <ModelSelector model={model.clone()} />
        <MaxTokenSlider
          max_token={max_token} model={model.clone()}
        />
        <TemperatureSlider {temperature} />
        <TopPSlider {top_p} />
        <PresencePenaltySlider
            presence_penalty={pres_penalty}
        />
        <FrequencyPenaltySlider {freq_penalty} />
        <div
          class="btn btn-neutral cursor-pointer mt-5"
          onclick={handle_reset}
        >
          {t("resetToDefault".to_string(), None)}
        </div>
      </div>
    </PopupModal>
    }
}

#[derive(Properties, PartialEq)]
struct SystemChatProps {
    system_message: UseStateHandle<String>,
}

#[function_component]
fn DefaultSystemChat(SystemChatProps { system_message } : &SystemChatProps) -> Html {
    let t = use_translation(vec!["model".to_string()]);
    let set_system_message = {
        let system_message = system_message.clone();
        move |e: Event| {
            if let Some(target) = e.target_dyn_into::<web_sys::HtmlTextAreaElement>() {
                let value = target.value();
                system_message.set(value);
            }
        }
    };
    
    let handle_input = Callback::from(|e: InputEvent| {
        if let Some(target) = e.target_dyn_into::<web_sys::HtmlTextAreaElement>() {
            // target.set_style("height", "auto");
            // target.set_style("height", &format!("{}px", target.scroll_height()));
            // target.set_style("max-height", &format!("{}px", target.scroll_height()));
        }
    });

    let handle_on_focus = Callback::from(|e: FocusEvent| {
        if let Some(target) = e.target_dyn_into::<web_sys::HtmlTextAreaElement>() {
            // target.set_style("height", "auto");
            // target.set_style("height", &format!("{}px", target.scroll_height()));
            // target.set_style("max-height", &format!("{}px", target.scroll_height()));
        }
    });

    let handle_on_blur = Callback::from(|e: FocusEvent| {
        if let Some(target) = e.target_dyn_into::<web_sys::HtmlTextAreaElement>() {
            // target.set_style("height", "auto");
            // target.set_style("max-height", "2.5rem");
        }
    });
    html! {
        <div>
      <div class="block text-sm font-medium text-gray-900 dark:text-white">
        {t("defaultSystemMessage".to_string(), None)}
      </div>
      <textarea
        class="my-2 mx-0 px-2 resize-none rounded-lg bg-transparent overflow-y-hidden leading-7 p-1 border border-gray-400/50 focus:ring-1 focus:ring-blue w-full max-h-10 transition-all"
        onfocus={handle_on_focus}
        onblur={handle_on_blur}
        onchange={set_system_message}
        oninput={handle_input}
        value={ system_message.to_string() }
        rows={1}
      ></textarea>
    </div>
    }
}
