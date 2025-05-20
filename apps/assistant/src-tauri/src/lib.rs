mod chathandle;

pub use chathandle::{invoke_llm_chat, Message};

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
use reqwest::multipart;
use tokio::fs::File;
use tauri::Manager; // For app_handle.state(), app_handle.clone() etc.
use tauri::AppHandle; // Added for emitting events
use tauri::Emitter; // Added for emit_all
use tauri_plugin_shell::ShellExt; // For app_handle.shell()
use tauri_plugin_shell::process::CommandChild;
// Removed Sidecar import as it's not found
use std::sync::Mutex;
use std::path::PathBuf; // Added for PathBuf
use std::env; // Added for std::env::temp_dir()

// State to hold the child process handle
struct SttServiceHandle(Mutex<Option<CommandChild>>);

#[derive(Clone, serde::Serialize)]
pub struct ProcessingStageUpdatePayload { // Made public
    stage: String,
    message: Option<String>,
}

#[tauri::command]
async fn invoke_stt_transcription(app_handle: AppHandle, audio_data: Vec<u8>) -> Result<String, String> {
    // Changed emit_all to emit, to align with user preference and see if it resolves method not found
    if let Err(e) = app_handle.emit("processing_stage_update", ProcessingStageUpdatePayload {
        stage: "TRANSCRIBING".to_string(),
        message: Some("Transcribing voice...".to_string()),
    }) {
        eprintln!("Failed to emit processing_stage_update event: {}", e);
    }

    // TODO: Make the STT service URL configurable
    let url = "http://127.0.0.1:5000/transcribe";

    // Create a temporary file on the Rust side
    // Use std::env::temp_dir() to get the system's temporary directory
    let temp_dir_path = std::env::temp_dir();
    // Create a unique filename within the system's temp directory
    let temp_file_name = format!("recording_{}.wav", chrono::Utc::now().timestamp_millis());
    let temp_file_path = temp_dir_path.join(temp_file_name);

    // Write the audio data to the temporary file
    use tokio::io::AsyncWriteExt;
    let mut temp_file = match File::create(&temp_file_path).await {
        Ok(f) => f,
        Err(e) => return Err(format!("Failed to create temporary audio file: {:?} - {}", temp_file_path, e)),
    };
    if let Err(e) = temp_file.write_all(&audio_data).await {
        return Err(format!("Failed to write audio data to temporary file: {:?} - {}", temp_file_path, e));
    }

    // Ensure data is flushed to disk
    if let Err(e) = temp_file.sync_all().await {
        eprintln!("Warning: Failed to sync temporary audio file to disk: {:?} - {}",temp_file_path, e);
    }


    // Create multipart form data
    let part = multipart::Part::bytes(audio_data) // Use the received audio_data directly
        .file_name("audio.wav") // Use a generic filename
        .mime_str("audio/wav") // Assuming WAV format from frontend (TODO: Frontend conversion)
        .map_err(|e| format!("Failed to create multipart part: {}", e))?;

    let form = multipart::Form::new().part("audio", part);

    // Send the request to the Python STT service
    let client = reqwest::Client::new();
    let response = match client.post(url).multipart(form).send().await {
        Ok(res) => res,
        Err(e) => {
            // Clean up temporary file on error
            cleanup_temp_file(&temp_file_path.to_string_lossy()).await;
            return Err(format!("Failed to send request to STT service: {}", e));
        }
    };

    // Check if the request was successful
    if !response.status().is_success() {
        let status = response.status();
        let text = response.text().await.unwrap_or_else(|_| "No response body".to_string());
        // Clean up temporary file on error
        cleanup_temp_file(&temp_file_path.to_string_lossy()).await;
        return Err(format!("STT service returned error status {}: {}", status, text));
    }

    // Parse the JSON response
    let json_response: serde_json::Value = match response.json().await {
        Ok(json) => json,
        Err(e) => {
            // Clean up temporary file on error
            cleanup_temp_file(&temp_file_path.to_string_lossy()).await;
            return Err(format!("Failed to parse STT service response JSON: {}", e));
        }
    };

    // Extract the transcription
    let transcription = json_response["transcription"].as_str()
        .ok_or_else(|| "Transcription field not found in response".to_string())?
        .to_string();

    // Clean up the temporary audio file after successful transcription
    cleanup_temp_file(&temp_file_path.to_string_lossy()).await;

    Ok(transcription)
}

#[tauri::command]
async fn synthesize_speech(app_handle: AppHandle, text: String, voice: Option<String>) -> Result<Vec<u8>, String> {
    // Changed emit_all to emit
    if let Err(e) = app_handle.emit("processing_stage_update", ProcessingStageUpdatePayload {
        stage: "SYNTHESIZING_VOICE".to_string(),
        message: None,
    }) {
        eprintln!("Failed to emit processing_stage_update event: {}", e);
    }

    let url = "http://localhost:8298/v1/audio/speech";
    let default_voice = "diep-chi".to_string(); // Default voice if none provided
    let selected_voice = voice.unwrap_or(default_voice);

    // Create JSON payload for the request
    let payload = serde_json::json!({
        "model": "tts-1",
        "input": text,
        "voice": selected_voice
    });

    // Send the request to the VietTTS service
    let client = reqwest::Client::new();
    let response = match client.post(url)
        .header("Content-Type", "application/json")
        .header("Authorization", "Bearer viet-tts")
        .json(&payload)
        .send().await {
            Ok(res) => res,
            Err(e) => return Err(format!("Failed to send request to VietTTS service: {}", e))
        };

    // Check if the request was successful
    if !response.status().is_success() {
        let status = response.status();
        let text = response.text().await.unwrap_or_else(|_| "No response body".to_string());
        return Err(format!("VietTTS service returned error status {}: {}", status, text));
    }

    // Get the audio data as bytes
    let audio_data = match response.bytes().await {
        Ok(bytes) => bytes.to_vec(),
        Err(e) => return Err(format!("Failed to retrieve audio data from VietTTS service: {}", e))
    };

    Ok(audio_data)
}

// Helper function to clean up the temporary file
async fn cleanup_temp_file(file_path: &str) {
    if let Err(e) = tokio::fs::remove_file(file_path).await {
        eprintln!("Failed to clean up temporary file {}: {}", file_path, e);
    } else {
        println!("Cleaned up temporary file: {}", file_path);
    }
}


#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_fs::init()) // Initialize the filesystem plugin
        .plugin(tauri_plugin_shell::init()) // Initialize the shell plugin
        .manage(SttServiceHandle(Default::default())) // Add state to manage the child process
        .setup(|app| {
            let app_handle = app.handle().clone(); // app_handle is 'static and can be moved
            
            tauri::async_runtime::spawn(async move {
                // Get State inside the async block using the 'static app_handle
                let stt_service_state_in_async = app_handle.state::<SttServiceHandle>();

                // Path to the bundled run_stt_service.bat script
                let script_path = app_handle.path()
                    .resolve("run_stt_service.bat", ::tauri::path::BaseDirectory::Resource) 
                    .expect("Failed to resolve resource path for run_stt_service.bat");
                
                // Determine the correct working directory for stt_service.py (i.e., packages/stt/src)
                // CARGO_MANIFEST_DIR points to apps/assistant/src-tauri for this crate
                let cargo_manifest_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
                // Navigate up from apps/assistant/src-tauri to the project root d:/_project/mivis
                let project_root = cargo_manifest_dir.parent().and_then(|p| p.parent()).and_then(|p| p.parent())
                    .expect("Failed to navigate to project root from CARGO_MANIFEST_DIR");
                let stt_service_working_dir = project_root.join("packages").join("stt").join("src");

                if !stt_service_working_dir.exists() {
                    eprintln!("Error: STT service working directory does not exist: {:?}", stt_service_working_dir);
                    return; // Or handle error appropriately
                }
                if !stt_service_working_dir.is_dir() {
                     eprintln!("Error: STT service working directory is not a directory: {:?}", stt_service_working_dir);
                    return; // Or handle error appropriately
                }


                println!("Launching STT service script: {:?}", script_path);
                
                // Canonicalize and then convert to string for current_dir, attempting to avoid UNC path issues with cmd.exe
                let canonical_working_dir = stt_service_working_dir.canonicalize()
                    .expect("Failed to canonicalize STT service working dir for command");
                let working_dir_str = canonical_working_dir.to_string_lossy().to_string();
                // If the path starts with \\?\, try to remove it for cmd.exe compatibility.
                let final_working_dir_str = if working_dir_str.starts_with("\\\\?\\") {
                    working_dir_str.trim_start_matches("\\\\?\\").to_string()
                } else {
                    working_dir_str
                };

                println!("STT service working directory (final for cmd): {:?}", final_working_dir_str);
                let script_path_str = script_path.to_string_lossy().to_string();
                let final_script_path_str = if script_path_str.starts_with("\\\\?\\") {
                    script_path_str.trim_start_matches("\\\\?\\").to_string()
                } else {
                    script_path_str
                };

                let command_to_run = app_handle.shell() 
                    .command("cmd.exe")
                    .args(["/C", &final_script_path_str]) // Use /C to execute the script
                    .current_dir(PathBuf::from(final_working_dir_str.clone())); 

                println!("Attempting to spawn sidecar command: cmd.exe /C {:?} in {:?}", final_script_path_str, final_working_dir_str);

                match command_to_run.spawn() {
                     Ok(child_tuple) => { 
                        println!("Sidecar command spawned. PID: {:?}", child_tuple.1.pid()); 
                        
                        *stt_service_state_in_async.0.lock().unwrap() = Some(child_tuple.1); 

                        let mut receiver = child_tuple.0; 
                        tauri::async_runtime::spawn(async move {
                            while let Some(event) = receiver.recv().await { 
                                match event {
                                    tauri_plugin_shell::process::CommandEvent::Stdout(line_bytes) => {
                                        println!("[Sidecar STDOUT]: {}", String::from_utf8_lossy(&line_bytes));
                                    }
                                    tauri_plugin_shell::process::CommandEvent::Stderr(line_bytes) => {
                                        eprintln!("[Sidecar STDERR]: {}", String::from_utf8_lossy(&line_bytes));
                                    }
                                    tauri_plugin_shell::process::CommandEvent::Error(message) => {
                                        eprintln!("[Sidecar ERROR]: {}", message);
                                    }
                                    tauri_plugin_shell::process::CommandEvent::Terminated(payload) => {
                                        println!("[Sidecar Terminated]: Code: {:?}, Signal: {:?}", payload.code, payload.signal);
                                    }
                                    _ => {} 
                                }
                            }
                            println!("[Sidecar] Event stream ended.");
                        });
                    }
                    Err(e) => {
                        eprintln!("Failed to spawn STT service sidecar: {}", e);
                    }
                }
            });
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![invoke_stt_transcription, synthesize_speech, invoke_llm_chat])
        .build(tauri::generate_context!())
        .expect("error while building tauri application")
        .run(|_app_handle, event| match event { // Handle exit
            tauri::RunEvent::ExitRequested { api: _, .. } => { // Fix unused 'api'
                // If you need to prevent exit or do cleanup before exit
                // _api.prevent_exit(); 
            }
            tauri::RunEvent::Exit => {
                // This event fires when the app is actually exiting
                // We need to get the handle from somewhere or have passed it
                // This part is tricky as _app_handle here might not have the state
                // A better way is to use the main window's on_close_requested
                println!("Tauri app is exiting. STT service should be cleaned up if managed.");
            }
            _ => {}
        });
}
