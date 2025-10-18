use reqwest::Client;
use serde::{Deserialize, Serialize};
use tauri::Manager;
use futures_util::StreamExt;
use tauri::Emitter;

#[derive(Serialize)]
struct OllamaRequest {
    model: String,
    prompt: String,
    stream: bool,
}

#[derive(Deserialize)]
struct OllamaResponse {
    response: String,
    done: bool,
}

#[tauri::command]
pub async fn ask_mistral(app_handle: tauri::AppHandle, prompt: String) -> Result<(), String> {
    let client = Client::new();
    let request = OllamaRequest {
        model: "mistral".to_string(),
        prompt,
        stream: true,
    };

    let mut stream = client
        .post("http://localhost:11434/api/generate")
        .json(&request)
        .send()
        .await
        .map_err(|e| e.to_string())?.bytes_stream();

    while let Some(item) = stream.next().await {
        let chunk = item.map_err(|e| e.to_string())?;
        let lines = chunk.split(|&b| b == b'\n');

        for line in lines {
            if line.is_empty() {
                continue;
            }
            match serde_json::from_slice::<OllamaResponse>(line) {
                Ok(ollama_response) => {
                    app_handle.emit("llm-token", ollama_response.response).map_err(|e| e.to_string())?;
                    if ollama_response.done {
                        break;
                    }
                }
                Err(e) => {
                    eprintln!("Failed to deserialize ollama response: {}", e);
                }
            }
        }
    }

    Ok(())
}
