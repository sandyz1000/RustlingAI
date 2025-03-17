use futures::Stream;
use reqwest::header::{HeaderMap, HeaderName, HeaderValue, CONTENT_TYPE};
use crate::types::{api::{EventSourceData, EventSourceDataInterface, ShareGPTSubmitBodyInterface}, chat::ModelOptions};
use std::collections::HashMap;
use reqwest::Error as ReqwestError;
use serde_json::Value;
use crate::types::chat::{ConfigInterface, MessageInterface};

pub async fn get_chat_completion(
    endpoint: String,
    messages: &Vec<MessageInterface>,
    config: &ConfigInterface,
    api_key: &Option<String>,
    custom_headers: Option<HashMap<String, String>>,
) -> Result<serde_json::Value, ReqwestError> {
    let mut headers = HeaderMap::new();
    headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));

    if let Some(api_key) = api_key {
        headers.insert("Authorization", HeaderValue::from_str(&format!("Bearer {}", api_key)).unwrap());
        headers.insert("api-key", HeaderValue::from_str(api_key).unwrap());
    }

    if let Some(custom_headers) = custom_headers {
        for (key, value) in custom_headers {
            headers.insert(HeaderName::from_bytes(key.as_bytes()).unwrap(), HeaderValue::from_str(&value).unwrap());
        }
    }

    let mut endpoint = endpoint.to_string();
    if is_azure_endpoint(&endpoint) && api_key.is_some() {
        
        // set api version to 2023-07-01-preview for gpt-4 and gpt-4-32k, otherwise use 2023-03-15-preview
        let api_version = if config.model == ModelOptions::Gpt4 || config.model == ModelOptions::Gpt432k {
            "2023-07-01-preview"
        } else {
            "2023-03-15-preview"
        };

        let path = format!("openai/deployments/{}/chat/completions?api-version={}", config.model, api_version);

        if !endpoint.ends_with(&path) {
            if !endpoint.ends_with('/') {
                endpoint.push('/');
            }
            endpoint.push_str(&path);
        }
    }

    let client = reqwest::Client::new();
    let response = client
        .post(endpoint)
        .headers(headers)
        .json(&serde_json::json!({
            "messages": messages,
            "temperature": config.temperature,
            "max_tokens": config.max_tokens,
        }))
        .send()
        .await?;

    match response.error_for_status() {
        Ok(res) => Ok(res.json().await?),
        Err(e) => Err(e),
    }
    
}

pub async fn get_chat_completion_stream(
    endpoint: String,
    messages: &Vec<MessageInterface>,
    config: &ConfigInterface,
    api_key: &Option<String>,
    custom_headers: Option<HashMap<String, String>>,
) -> Result<impl Stream<Item = reqwest::Result<bytes::Bytes>>, reqwest::Error> {
    let mut headers = HeaderMap::new();
    headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));

    if let Some(api_key) = api_key {
        headers.insert("Authorization", HeaderValue::from_str(&format!("Bearer {}", api_key)).unwrap());
        headers.insert("api-key", HeaderValue::from_str(api_key).unwrap());
    }

    if let Some(custom_headers) = custom_headers {
        for (key, value) in custom_headers {
            headers.insert(HeaderName::from_bytes(key.as_bytes()).unwrap(), HeaderValue::from_str(&value).unwrap());
        }
    }

    let mut endpoint = endpoint.to_string();
    if is_azure_endpoint(&endpoint) && api_key.is_some() {
        // set api version to 2023-07-01-preview for gpt-4 and gpt-4-32k, otherwise use 2023-03-15-preview
        let api_version = if config.model == ModelOptions::Gpt4 || config.model == ModelOptions::Gpt432k {
            "2023-07-01-preview"
        } else {
            "2023-03-15-preview"
        };

        let path = format!("openai/deployments/{}/chat/completions?api-version={}", config.model.to_string(), api_version);

        if !endpoint.ends_with(&path) {
            if !endpoint.ends_with('/') {
                endpoint.push('/');
            }
            endpoint.push_str(&path);
        }
    }

    let client = reqwest::Client::new();
    let response = client
        .post(endpoint)
        .headers(headers)
        .json(&serde_json::json!({
            "messages": messages,
            "temperature": config.temperature,
            "max_tokens": config.max_tokens,
            "stream": true,
        }))
        .send()
        .await?;
    let stream = response.bytes_stream();
    Ok(stream)
}

pub async fn submit_share_gpt(body: ShareGPTSubmitBodyInterface) -> Result<(), ReqwestError> {
    let client = reqwest::Client::new();
    // TODO: FixMe - ShareGPT is deprecated, using the OpenAI builtin share api
    let response = client
        .post("https://sharegpt.com/api/conversations")
        .header(CONTENT_TYPE, HeaderValue::from_static("application/json"))
        .json(&body)
        .send()
        .await?;

    match response.error_for_status() {
        Ok(response) => {
            if let Ok(json) = response.json::<serde_json::Value>().await {
                if let Some(id) = json.get("id").and_then(|id| id.as_str()) {
                    // Open the URL in a new browser tab (requires WASM environment)
                    gloo_utils::window()
                        .open_with_url_and_target(&format!("https://shareg.pt/{}", id), "_blank")
                        .ok();
                }
            }
    
            Ok(())
        }
        Err(e) => Err(e),
    }

}

// Helper function to check Azure endpoints
fn is_azure_endpoint(endpoint: &str) -> bool {
    endpoint.contains(".azure.com")
}


pub fn parse_event_source(data: String) -> Vec<EventSourceData> {
    data.split("\n\n")
        .filter(|chunk| !chunk.is_empty())
        .map(|chunk| {
            let json_string = chunk
                .lines()
                .map(|line| line.strip_prefix("data: ").unwrap_or(line)) // Remove "data: " prefix
                .collect::<String>();

            if json_string == "[DONE]" {
                EventSourceData::Done(json_string)
            } else {
                match serde_json::from_str::<EventSourceDataInterface>(&json_string) {
                    Ok(data) => {
                        EventSourceData::Data(data)
                    },
                    Err(_) => EventSourceData::Done(json_string)
                }
            }
        })
        .collect()
}
