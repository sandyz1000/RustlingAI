use std::collections::HashMap;

use yew::prelude::*;
use yewdux::use_store;

use crate::{
    components::{
        icons::{CalculatorIcon, SettingIcon},
        language_selector::LanguageSelector,
        menu::{
            chat::config::ChatConfigMenu,
            menu_options::{ClearConversation, ThemeSwitcher},
            prompt_library::PromptLibraryMenu,
        },
        popup_modal::PopupModal,
        toggle::Toggle,
    },
    hooks::translation::{Namespace, use_translation},
    store::ConfigSlice,
    types::chat::{ModelOptions, TokenUsage},
};

#[function_component]
pub(crate) fn SettingsMenu() -> Html {
    let is_modal_open = use_state(|| false);
    let t = use_translation(vec![]);
    html! {
      <>
        <a
          class="flex py-2 px-2 items-center gap-3 rounded-md hover:bg-gray-500/10 transition-colors duration-200 text-white cursor-pointer text-sm"
          onclick={
                let is_modal_open = is_modal_open.clone();
                move |_e| is_modal_open.set(true)
            }
        >
          <SettingIcon class_name="w-4 h-4" /> {t("setting".to_string(), None)}
        </a>
        if *is_modal_open {
          <PopupModal
            set_is_modal_open= {
                let is_modal_open = is_modal_open.clone();
                move |val: bool| {
                    is_modal_open.set(val);
                }
            }
            title={t("setting".to_string(), None)}
            cancel_button={false}
          >
            <div class="p-6 border-b border-gray-200 dark:border-gray-600 flex flex-col items-center gap-4">
              <LanguageSelector />
              <ThemeSwitcher />
              <div class="flex flex-col gap-3">
                <AutoTitleToggle />
                <EnterToSubmitToggle />
                <InlineLatexToggle />
                <AdvancedModeToggle />
                <TotalTokenCostToggle />
              </div>
              <ClearConversation />
              <PromptLibraryMenu />
              <ChatConfigMenu />
              <TotalTokenCost />
            </div>
          </PopupModal>
        }
      </>
    }
}

#[function_component]
fn InlineLatexToggle() -> Html {
    let t = use_translation(vec![]);
    let is_checked = use_state(|| false);
    let (_config, dispatch) = use_store::<ConfigSlice>();
    {
        let is_checked = is_checked.clone();
        let dispatch = dispatch.clone();
        use_effect_with(is_checked, move |is_checked| {
            dispatch.reduce_mut(|d| d.inline_latex = **is_checked);
        });
    }
    html! {
      <Toggle
        label={t("inlineLatex".to_string(), None)}
        is_checked={is_checked}
      />
    }
}

#[function_component]
fn EnterToSubmitToggle() -> Html {
    let t = use_translation(vec![]);
    let is_checked = use_state(|| false);
    let (_config, dispatch) = use_store::<ConfigSlice>();
    {
        let is_checked = is_checked.clone();
        let dispatch = dispatch.clone();
        use_effect_with(is_checked, move |is_checked| {
            dispatch.reduce_mut(|d| d.enter_to_submit = **is_checked);
        });
    }
    html! {
      <Toggle
        label={t("enterToSubmit".to_string(), None)}
        is_checked={is_checked}
      />
    }
}

#[function_component]
fn AdvancedModeToggle() -> Html {
    let t = use_translation(vec![]);
    let is_checked = use_state(|| false);
    let (_config, dispatch) = use_store::<ConfigSlice>();
    {
        let is_checked = is_checked.clone();
        let dispatch = dispatch.clone();
        use_effect_with(is_checked, move |is_checked| {
            dispatch.reduce_mut(|d| d.advanced_mode = **is_checked);
        });
    }
    html! {
      <Toggle
        label={t("advancedMode".to_string(), None)}
        is_checked={is_checked}
      />
    }
}

#[function_component]
fn AutoTitleToggle() -> Html {
    let is_checked = use_state(|| false);
    let t = use_translation(vec![]);
    let (_config, dispatch) = use_store::<ConfigSlice>();
    {
        let is_checked = is_checked.clone();
        let dispatch = dispatch.clone();
        use_effect_with(is_checked, move |is_checked| {
            dispatch.reduce_mut(|d| d.auto_title = **is_checked);
        });
    }
    html! {
      <Toggle
        label={t("autoTitle".to_string(), None)}
        is_checked={is_checked}
      />
    }
}

#[function_component]
pub(crate) fn TotalTokenCostDisplay() -> Html {
    let t  = use_translation(vec![]);
    let (state, _dispatch) = use_store::<ConfigSlice>();

    let total_cost = use_state(|| 0);
    {
        let mut updated_total_cost = 0;
        let total_cost = total_cost.clone();
        let state = state.clone();
        use_effect_with(total_cost, move |total_cost| {
            for (model, token_cost) in state.total_token_used.iter() {
                let cost = token_cost_to_cost(token_cost, model);
                updated_total_cost += cost;
            }
            total_cost.set(updated_total_cost);
        });
    }

    html! {
      <a class="flex py-2 px-2 items-center gap-3 rounded-md hover:bg-gray-500/10 transition-colors duration-200 text-white text-sm">
        <CalculatorIcon />
        { format!("USD: {}", total_cost.to_string()) }
      </a>
    }
}

#[function_component]
pub fn TotalTokenCostToggle() -> Html {
    let t = use_translation(vec![]);
    let (config, dispatch) = use_store::<ConfigSlice>();
    let is_checked = use_state(|| config.count_total_tokens);
    {
        let dispatch = dispatch.clone();
        let is_checked = is_checked.clone();
        use_effect_with(is_checked, move |is_checked| {
            dispatch.reduce_mut(|f| f.count_total_tokens = **is_checked)
        });
    }
    html! {
      <Toggle
        label={t("countTotalTokens".to_string(), None)} {is_checked}
      />
    }
}

struct CostMapping {
    model: ModelOptions,
    cost: i32,
}

fn token_cost_to_cost(token_cost: &TokenUsage, model: &ModelOptions) -> i32 {
    // TODO: Fix me
    // todo!()
    0
}

#[function_component]
pub fn TotalTokenCost() -> Html {
    let t = use_translation(vec![]);
    let (state, store_dispatch) = use_store::<ConfigSlice>();
    let total_token = use_state(|| state.total_token_used.clone());
    // let count_total_token = use_state(|| state.count_total_tokens);
    let cost_mapping = use_state(|| Vec::<CostMapping>::new());

    let total_cost = 0.0;

    {
        let cost_mapping = cost_mapping.clone();
        use_effect_with(total_token, move |total_token| {
            let mut updated_cost: Vec<CostMapping> = vec![];
            for (model, token_cost) in total_token.iter() {
                let cost = token_cost_to_cost(token_cost, model);
                updated_cost.push(CostMapping {
                    model: model.clone(),
                    cost,
                });
            }
            cost_mapping.set(updated_cost);
        });
    }
    html! {
      if state.count_total_tokens {
        <div class="flex flex-col items-center gap-2">
        <div class="relative overflow-x-auto shadow-md sm:rounded-lg">
          <table class="w-full text-sm text-left text-gray-500 dark:text-gray-400">
            <thead class="text-xs text-gray-700 uppercase bg-gray-50 dark:bg-gray-700 dark:text-gray-400">
              <tr>
                <th class="px-4 py-2">{t("model".to_string(), Some(Namespace { ns: "model".to_string() }))}</th>
                <th class="px-4 py-2">{"USD"}</th>
              </tr>
            </thead>
            <tbody>
              {
                cost_mapping.iter().map(| CostMapping{ model, cost }|  {
                  html! {
                    <tr
                      key={model.to_string()}
                      class="bg-white border-b dark:bg-gray-800 dark:border-gray-700 hover:bg-gray-50 dark:hover:bg-gray-700"
                    >
                      <td class="px-4 py-2">{model.to_string()}</td>
                      <td class="px-4 py-2">{*cost}</td>
                    </tr>
                  }
                }).collect::<Html>()
              }
              <tr class="bg-white border-b dark:bg-gray-800 dark:border-gray-700 font-bold">
                <td class="px-4 py-2">{t("total".to_string(), Some(Namespace { ns: "main".to_string() }))}</td>
                <td class="px-4 py-2">
                  { total_cost }
                </td>
              </tr>
            </tbody>
          </table>
        </div>
        <div
            class="btn btn-neutral cursor-pointer"
            onclick={
                let store_dispatch = store_dispatch.clone();
                move |_e| store_dispatch.reduce_mut(|d| d.total_token_used = HashMap::new())
            }
        >
          {t("resetCost".to_string(), Some(Namespace{ ns: "main".to_string() }))}
        </div>
      </div>
      } else {
        <></>
      }
    }
}
