use crate::store::{AuthSlice, ChatSlice, ConfigSlice};
use crate::types::api::EventSourceData;
use crate::types::chat::{MessageInterface, ModelOptions, Role, TokenUsage};
use crate::{api::get_chat_completion_stream, types::chat::ConfigInterface};
use futures::StreamExt;
use std::future::Future;
use std::sync::Arc;
use tiktoken_rs::{ChatCompletionRequestMessage, cl100k_base, get_chat_completion_max_tokens};
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;
use yewdux::{Dispatch, use_store};
use crate::api::parse_event_source;

// type HandleAsyncFunc = Arc<dyn Fn() -> Box<dyn Future<Output = Option<String>> + Send + Sync>>;

#[hook]
pub fn use_submit() -> (Callback<()>, String) {
    let (auth_state, _) = use_store::<AuthSlice>();
    let (config_store, config_dispath) = use_store::<ConfigSlice>();
    let (state, chat_dispatch) = use_store::<ChatSlice>();
    let error = state.error.clone();

    let handle_submit = {
        let chat_dispatch = chat_dispatch.clone();
        let auth_state = auth_state.clone();
        let (config_store, config_dispath) = (config_store.clone(), config_dispath.clone());
        let state = state.clone();
        Callback::from(move |_| {
            let current_chat_index = state.curr_chat_index as usize;

            spawn_local({
                let chat_dispatch = chat_dispatch.clone();
                let auth_state = auth_state.clone();
                let (config_store, config_dispath) = (config_store.clone(), config_dispath.clone());
                async move {
                    chat_dispatch.reduce_mut(|s| {
                        if s.generating || s.chats.is_empty() {
                            return;
                        }

                        // Clone current chat state
                        let mut chats = s.chats.clone();
                        let chat = &mut chats[current_chat_index];

                        // Add assistant message placeholder
                        chat.messages.push(MessageInterface {
                            role: Role::Assistant,
                            content: String::new(),
                            folder: None,
                            messages: vec![],
                        });

                        s.chats = chats;
                        s.generating = true;
                    });

                    let result = async {
                        let state = chat_dispatch.get();
                        let chat = &state.chats[current_chat_index];

                        if chat.messages.is_empty() {
                            anyhow::bail!("No messages submitted!");
                        }

                        let config = if let Some(config) = &chat.config {
                            config.clone()
                        } else {
                            ConfigInterface::default()
                        };
                        let messages = limit_message_tokens(
                            &chat.messages,
                            config.max_tokens as usize,
                            &config.model.clone(),
                        );

                        if auth_state.api_endpoint.is_none() {
                            anyhow::bail!("No API endpoint defined!");
                        }
                        let endpoint = auth_state.api_endpoint.clone().unwrap();
                        
                        // Handle streaming response
                        let mut stream = get_chat_completion_stream(
                            endpoint,
                            &messages,
                            &config,
                            &auth_state.api_key,
                            None,
                        )
                        .await?;

                        while let Some(event) = stream.next().await {
                            let data = String::from_utf8(event.unwrap().to_vec()).unwrap();
                            let mut reading = true;
                            let data: Vec<EventSourceData> = parse_event_source(data);
                            if reading && state.generating {
                                if let EventSourceData::Done(_inner) = &data[0] {
                                    reading = false;
                                    break;
                                } 

                                let data = data.iter().fold(String::new(), |mut output, curr| {
                                    if let EventSourceData::Data(inner_data) = &curr {
                                        let delta = inner_data.choices.get(0).and_then(|choice| Some(choice.delta.clone())).unwrap();
                                        if let Some(content) = &delta.content {
                                            output.push_str(content);
                                        }
                                    }
                                    output
                                });

                                // Update the chat state 
                                chat_dispatch.reduce_mut(|s| {
                                    let chat = &mut s.chats[current_chat_index];
                                    if let Some(last_msg) = chat.messages.last_mut() {
                                        last_msg.content.push_str(&data);
                                    }
                                });
                            }

                            // If User cancelled the generation of token, cancel the stream
                            if !state.generating {
                                break;
                            }
                        }

                        // Update token usage in chatting
                        if !state.chats.is_empty() && config_store.count_total_tokens {
                            let model = state.chats[current_chat_index]
                                .config
                                .as_ref()
                                .unwrap()
                                .model
                                .clone();
                            let messages = state.chats[current_chat_index].messages.clone();
                            let last_message = messages.last().unwrap().clone();
                            update_total_token_used(
                                config_dispath.clone(),
                                model,
                                messages,
                                last_message,
                            );
                        }

                        // Generate title if needed
                        if config_store.auto_title && !state.chats[current_chat_index].title_set {
                            generate(chat_dispatch.clone(), config_dispath.clone(), current_chat_index).await?;
                        }

                        Ok(())
                    }
                    .await;

                    chat_dispatch.reduce_mut(|s| {
                        s.generating = false;
                        if let Err(e) = result {
                            s.error = e.to_string();
                        }
                    });
                }
            });
        })
    };

    (handle_submit, error)
}

fn update_total_token_used(
    config_dispatch: Dispatch<ConfigSlice>,
    model: ModelOptions,
    prompt_msg: Vec<MessageInterface>,
    message: MessageInterface,
) {
    config_dispatch.reduce_mut(|d| {
        let current_usage = d
            .total_token_used
            .get(&model)
            .cloned()
            .unwrap_or(TokenUsage {
                prompt_tokens: 0,
                completion_tokens: 0,
            });

        let new_prompt_token = count_tokens(&prompt_msg, &model) as u32;
        let new_completion_token = count_tokens(&[message], &model) as u32;
        // Update the total token usage for the given model
        d.total_token_used.insert(
            model,
            TokenUsage {
                prompt_tokens: current_usage.prompt_tokens + new_prompt_token,
                completion_tokens: current_usage.completion_tokens + new_completion_token,
            },
        );
    });
}

async fn generate_title(messages: &[MessageInterface]) -> anyhow::Result<String> {
    // todo!()
    Ok("".to_string())
}

async fn generate(chat_dispatch: Dispatch<ChatSlice>, config_dispatch: Dispatch<ConfigSlice>, index: usize) -> anyhow::Result<()> {
    let state = chat_dispatch.get();
    let chat = &state.chats[index];

    let messages = &chat.messages;
    let user_message = messages[messages.len() - 2].content.clone();
    let assistant_message = messages[messages.len() - 1].content.clone();

    let prompt = format!(
        "Generate a title in less than 6 words for the following message:\n\
        User: {}\n\
        Assistant: {}",
        user_message, assistant_message
    );
    let message = MessageInterface {
        role: Role::User,
        content: prompt,
        folder: None,
        messages: vec![]
    };

    let title = generate_title(&[message.clone()]).await
        .unwrap_or_default()
        .trim_matches('"')
        .to_string();

    chat_dispatch.reduce_mut({
        let title = title.clone();
        move |s| {
            if let Some(chat) = s.chats.get_mut(index) {
                chat.title = Some(title);
                chat.title_set = true;
            }
        }
    });

    let config = config_dispatch.get();
    if config.count_total_tokens {
        let assistant_msg = MessageInterface { role: Role::Assistant, content: title.clone(), folder: None, messages: vec![] };
        let model = ConfigInterface::default().model;
        update_total_token_used(config_dispatch.clone(), model, vec![message.clone()], assistant_msg);
    }

    Ok(())
}

pub fn count_tokens(messages: &[MessageInterface], model: &ModelOptions) -> usize {
    if messages.is_empty() {
        return 0;
    }

    // Convert to chat completion format
    let chat_messages: Vec<ChatCompletionRequestMessage> = messages
        .iter()
        .map(|msg| ChatCompletionRequestMessage {
            role: msg.role.to_string(),
            content: Some(msg.content.clone()),
            name: None,
            function_call: None,
        })
        .collect();

    // Get appropriate model name string
    let model_name = model.to_string();

    get_chat_completion_max_tokens(&model_name, &chat_messages).unwrap_or(0)
}

pub fn limit_message_tokens(
    messages: &[MessageInterface],
    limit: usize,
    model: &ModelOptions,
) -> Vec<MessageInterface> {
    if messages.is_empty() {
        return vec![];
    }

    let mut limited_messages = Vec::new();
    let mut token_count = 0;

    let is_system_first = messages[0].role == Role::System;
    let mut retain_system = false;

    // Check system message fit
    if is_system_first {
        let system_tokens = count_tokens(&messages[0..1], model);
        if system_tokens < limit {
            token_count += system_tokens;
            retain_system = true;
        }
    }

    // Process messages in reverse (excluding first)
    // Iterate through messages in reverse order, adding them to the limitedMessages array
    // until the token limit is reached (excludes first message)
    for msg in messages.iter().rev().take(messages.len() - 1) {
        let msg_tokens = count_tokens(&[msg.clone()], model);
        if token_count + msg_tokens > limit {
            break;
        }
        token_count += msg_tokens;
        limited_messages.insert(0, msg.clone());
    }

    // Handle system message
    if retain_system {
        let insert_pos = limited_messages.len().saturating_sub(2);
        limited_messages.insert(insert_pos, messages[0].clone());
    } else if !is_system_first && !messages.is_empty() {
        let first_tokens = count_tokens(&messages[0..1], model);
        if token_count + first_tokens <= limit {
            limited_messages.insert(0, messages[0].clone());
        }
    }

    limited_messages
}
