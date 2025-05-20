// Chat handling module for Mivis Desktop Assistant
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::env;
use std::error::Error; // Import the Error trait
use tokio::time;
use tauri::{AppHandle, Emitter}; // Added Emitter

// WorkflowTimings will be managed by the orchestrating function

use super::ProcessingStageUpdatePayload; // Import from lib.rs

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Message {
    pub role: String,
    pub content: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<String>, // For Task 2 (text or voice)
}

#[tauri::command]
pub async fn invoke_llm_chat(app_handle: AppHandle, messages: Vec<Message>) -> Result<String, String> {
    // Correcting the emit call to use emit_all, assuming Emitter trait is correctly in scope
    if let Err(e) = app_handle.emit("processing_stage_update", ProcessingStageUpdatePayload {
        stage: "PROCESSING_API".to_string(),
        message: Some("Processing request...".to_string()),
    }) {
        eprintln!("Failed to emit processing_stage_update event: {}", e);
    }

    let client = Client::new();
    let api_key = env::var("XAI_API_KEY").map_err(|e| format!("Missing API key: {}", e))?;

    // Create a mutable copy of messages to prepend the system prompt
    let mut messages_with_system_prompt = messages.clone(); // Assuming Message is Cloneable, or manually clone

    // Prepend the system prompt
    messages_with_system_prompt.insert(0, Message {
        role: "system".to_string(),
        content: 
        "Mày là Arisu có tính cách ngổ ngáo và tinh nghịch, 
        khi mày trả lời thì mày phải sử dụng ngôn ngữ tự nhiên 
        và bằng tiếng Việt, theo phong cách hội thoại. 
        Mày rất hỗn láo và không sợ làm mất lòng người khác.
        Vì thế nên câu trả lời của mày cụt lủn, 
        không cần có chủ ngũ vị ngữ, thể hiện sự mất dạy của mày.".to_string(),
        source: None,
    });
    
    // Log input source for debugging if available
    if let Some(last_msg) = messages_with_system_prompt.last() { // Use the modified list for logging if appropriate
        if let Some(source) = &last_msg.source {
            println!("Input source: {}", source);
        }
    }
    
    // Direct request to get completion response
    let mut attempts = 0;
    let max_attempts = 3;
    let mut delay = time::Duration::from_secs(2);
    
    let response = loop {
        attempts += 1;
        match client
            .post("https://api.x.ai/v1/chat/completions")
            .header("Authorization", format!("Bearer {}", api_key))
            .header("Content-Type", "application/json")
            .json(&serde_json::json!({
                "model": "grok-3-mini-beta",
                "messages": &messages_with_system_prompt // Use the messages with the system prompt
            }))
            .send()
            .await
        {
            Ok(response) => break response,
            Err(e) => {
                if attempts >= max_attempts {
                    let mut error_details = format!("API call failed after {} attempts: {}", attempts, e);
                    if let Some(source) = e.source() {
                        error_details.push_str(&format!("\nSource error: {}", source));
                        let mut current_source = source;
                        while let Some(next_source) = current_source.source() {
                            error_details.push_str(&format!("\nCaused by: {}", next_source));
                            current_source = next_source;
                        }
                    }
                    return Err(error_details);
                }
                time::sleep(delay).await;
                delay *= 2;
            }
        }
    };
    
    if !response.status().is_success() {
        let status = response.status();
        let text = response.text().await.unwrap_or_else(|_| "No response body".to_string());
        return Err(format!("xAI API returned error status {}: {}", status, text));
    }
    
    // Parse the response to extract the content
    let completion_data: serde_json::Value = response.json().await
        .map_err(|e| format!("Failed to parse response: {}", e))?;
    let content = completion_data["choices"][0]["message"]["content"].as_str()
        .ok_or_else(|| "Content field not found in response".to_string())?
        .to_string();
    
    return Ok(content);
}
