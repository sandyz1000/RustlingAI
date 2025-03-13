use yew::prelude::*;
use yewdux::use_store;

use crate::{
    components::{icons::{CalculatorIcon, SettingIcon}, language_selector::LanguageSelector, menu::{chat_config::ChatConfigMenu, menu_options::{ClearConversation, ThemeSwitcher}, prompt_library::PromptLibraryMenu}, popup_modal::PopupModal, toggle::Toggle},
    hooks::translation::{use_translation, Namespace}, store::slice::ConfigSlice,
};

#[function_component]
pub(crate) fn SettingsMenu() -> Html {
    let isModalOpen = use_state(|| false);
    let t = use_translation(vec![]);
    html! {
      <>
        <a
          class="flex py-2 px-2 items-center gap-3 rounded-md hover:bg-gray-500/10 transition-colors duration-200 text-white cursor-pointer text-sm"
          onclick={|e| { } }
        >
          <SettingIcon class_name="w-4 h-4" /> {t("setting".to_string(), None)}
        </a>
        if *isModalOpen {
          <PopupModal
            set_is_modal_open= { let isModalOpen = isModalOpen.clone(); Callback::from(|val: bool| { isModalOpen.set(val); }) }
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
  // let t  = use_translation(vec![]);
  let (state, _dispatch) = use_store::<ConfigSlice>();

  let totalCost = use_state(|| 0);

  // useEffect(() => {
  //   let updatedTotalCost = 0;

  //   Object.entries(totalTokenUsed).forEach(([model, tokenCost]) => {
  //     updatedTotalCost += tokenCostToCost(tokenCost, model as ModelOptions);
  //   });

  //   setTotalCost(updatedTotalCost);
  // }, [totalTokenUsed]);

  html! {
    <a class="flex py-2 px-2 items-center gap-3 rounded-md hover:bg-gray-500/10 transition-colors duration-200 text-white text-sm">
      <CalculatorIcon />
      { format!("USD: {}", totalCost.to_string()) }
    </a>
  }
    
}

#[function_component]
pub fn TotalTokenCostToggle() -> Html {
  let is_checked = use_state(|| false);
  let t = use_translation(vec![]);
  html! {
    <Toggle
      label={t("countTotalTokens".to_string(), None)} {is_checked}
    />
  }
}

struct CostMapping {
  model: String,
  cost: i32
}

#[function_component]
pub fn TotalTokenCost() -> Html {
  // let total_cost = costMapping
  //                 .reduce((prev, curr) => prev + curr.cost, 0)
  //                 .toPrecision(3);
  let total_cost = 0.0;
  let t = use_translation(vec![]);
  let cost_mapping = use_state(||Vec::<CostMapping>::new());
  let (state, _) = use_store::<ConfigSlice>();
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
                    <td class="px-4 py-2">{model}</td>
                    <td class="px-4 py-2">{cost.toPrecision(3)}</td>
                  </tr>
                }
              }).collect::<Html>()
            }
            <tr class="bg-white border-b dark:bg-gray-800 dark:border-gray-700 font-bold">
              <td class="px-4 py-2">{t("total".to_string(), Some(Namespace { ns: "main" }))}</td>
              <td class="px-4 py-2">
                { total_cost }
              </td>
            </tr>
          </tbody>
        </table>
      </div>
      <div class="btn btn-neutral cursor-pointer" onclick={resetCost}>
        {t("resetCost".to_string(), Some(Namespace{ ns: "main".to_string() }))}
      </div>
    </div>
    } else {
      <></>
    }
  }
}
