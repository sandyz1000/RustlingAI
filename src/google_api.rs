use reqwest::{Client, multipart::{Form, Part}};
use serde::{Deserialize, Serialize};
use anyhow::{Result, anyhow};
use std::io::Read;
use tokio::time::{sleep, Duration};

use crate::types::api::{GoogleFileList, GoogleFileResource, GoogleTokenInfo};

#[derive(Debug, Serialize, Deserialize)]
pub struct PersistStorageState {
    // Define your persisted state fields
}

// Multipart helper
async fn create_multipart_body(
    metadata: &serde_json::Value,
    file_path: &str,
    boundary: &str,
) -> Result<Form> {
    // Serialize metadata into JSON string
    let metadata_json = serde_json::to_string(&metadata)?;

    // Create metadata part
    let metadata_part = Part::text(metadata_json)
        .mime_str("application/json; charset=UTF-8")?;

    // Read the file contents
    let mut file = std::fs::File::open(std::path::Path::new(file_path))?;
    let mut file_contents = Vec::new();
    file.read_to_end(&mut file_contents)?;

    // Create file part
    let file_part = Part::bytes(file_contents)
        .mime_str("application/octet-stream")?; // Adjust MIME type as needed

    // Construct multipart form with boundary
    let form = Form::new()
        .text(format!("--{}", boundary), "".to_string()) // Start boundary
        .part("metadata", metadata_part)
        .text(format!("--{}", boundary), "".to_string()) // File boundary
        .part("file", file_part)
        .text(format!("--{}--", boundary), "".to_string()); // End boundary

    Ok(form)
}

// API functions
pub async fn create_drive_file(
    file_path: &str,
    access_token: &str,
) -> Result<GoogleFileResource> {
    let client = Client::new();
    let boundary = "better_chatgpt";
    
    let metadata = serde_json::json!({
        "name": std::path::Path::new(file_path)
            .file_name()
            .unwrap()
            .to_string_lossy(),
        "mimeType": mime_guess::from_path(file_path)
            .first_or_octet_stream()
            .to_string()
    });

    let form = create_multipart_body(&metadata, file_path, boundary).await?;

    let response = client
        .post("https://www.googleapis.com/upload/drive/v3/files?uploadType=multipart")
        .bearer_auth(access_token)
        .multipart(form)
        .send()
        .await?;

    if response.status().is_success() {
        Ok(response.json().await?)
    } else {
        Err(anyhow!("Error uploading file: {}", response.status()))
    }
}

pub async fn get_drive_file<S: serde::de::DeserializeOwned>(
    file_id: &str,
    access_token: &str,
) -> Result<S> {
    let client = Client::new();
    let response = client
        .get(&format!("https://content.googleapis.com/drive/v3/files/{}?alt=media", file_id))
        .bearer_auth(access_token)
        .send()
        .await?;

    Ok(response.json().await?)
}

pub async fn list_drive_files(access_token: &str) -> Result<GoogleFileList> {
    let client = Client::new();
    let response = client
        .get("https://www.googleapis.com/drive/v3/files?orderBy=createdTime desc")
        .bearer_auth(access_token)
        .send()
        .await?;

    Ok(response.json().await?)
}

pub async fn update_drive_file(
    file_path: &str,
    file_id: &str,
    access_token: &str,
) -> Result<GoogleFileResource> {
    let client = Client::new();
    let response = client
        .patch(&format!("https://www.googleapis.com/upload/drive/v3/files/{}", file_id))
        .bearer_auth(access_token)
        .body(tokio::fs::read(file_path).await?)
        .send()
        .await?;

    Ok(response.json().await?)
}

pub async fn update_drive_file_name(
    file_name: &str,
    file_id: &str,
    access_token: &str,
) -> Result<GoogleFileResource> {
    let client = Client::new();
    let response = client
        .patch(&format!("https://www.googleapis.com/drive/v3/files/{}", file_id))
        .bearer_auth(access_token)
        .json(&serde_json::json!({ "name": file_name }))
        .send()
        .await?;

    Ok(response.json().await?)
}

pub async fn delete_drive_file(
    file_id: &str,
    access_token: &str,
) -> Result<()> {
    let client = Client::new();
    let response = client
        .delete(&format!("https://www.googleapis.com/drive/v3/files/{}", file_id))
        .bearer_auth(access_token)
        .send()
        .await?;

    if response.status().is_success() {
        Ok(())
    } else {
        Err(anyhow!("Error deleting file: {}", response.status()))
    }
}

pub async fn validate_google_oauth2_token(
    access_token: &str,
) -> Result<GoogleTokenInfo> {
    let client = Client::new();
    let response = client
        .get(&format!("https://oauth2.googleapis.com/tokeninfo?access_token={}", access_token))
        .send()
        .await?;

    Ok(response.json().await?)
}

pub async fn update_drive_file_debounced(
    file_path: &str,
    file_id: &str,
    access_token: &str,
) -> Result<GoogleFileResource> {
    sleep(Duration::from_secs(5)).await;
    update_drive_file(file_path, file_id, access_token).await
}
