use yew::prelude::*;
use yewdux::prelude::*;
use crate::{store::ChatSlice, types::chat::ModelOptions};

#[function_component]
pub(crate) fn TokenCount() -> Html {
    let token_count = use_state(|| 0);
    let (state, _) = use_store::<ChatSlice>();
    let generating = state.generating;
    let messages = if !state.chats.is_empty() {
        state.chats[state.curr_chat_index as usize].messages.clone()
    } else {
        vec![]
    };
    let mut model = ModelOptions::Gpt35Turbo;
    if !state.chats.is_empty() {
        if let Some(config) = &state.chats[state.curr_chat_index as usize].config {
          model = config.model.clone();
        }        
    };
    let cost = {
      let token_count = token_count.clone();
      use_memo((model, token_count), |(model, token_count)| {
        // let price = model
        
        0
      })
    };

    {
      let token_count = token_count.clone();
      use_effect_with((messages, generating), move |(messages, generating)| {
        if *generating {
          token_count.set(1);
        }
      });
    }
    html! {
      <div class="absolute top-[-16px] right-0">
        <div class="text-xs italic text-gray-900 dark:text-gray-300">
          {format!("Tokens: {} ({})", *token_count, cost)}
        </div>
      </div>
    }
}
