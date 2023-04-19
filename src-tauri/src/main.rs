// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use reqwest::Client;
use serde::{ Deserialize, Serialize };
use serde_json::Value;
use async_std::task::block_on;

#[derive(Serialize, Deserialize)]
struct DataToSend {
    prompt: String,
}

// #[derive(Serialize, Deserialize)]
// struct ResponseData {
// 応答データのフィールドを定義します。
// }

async fn post_data(url: &str, data: DataToSend) -> Result<Value, reqwest::Error> {
    let client = Client::new();
    let response = client.post(url).json(&data).send().await?;

    let response_data = response.json().await?;
    Ok(response_data)
}

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn question(name: String) -> String {
    let url = "http://127.0.0.1:8000/completion";
    let data = DataToSend { prompt: name.to_string() };

    let result = block_on(async {
        match post_data(url, data).await {
            Ok(response_data) => { format!("{}", response_data) }
            Err(e) => {
                println!("error: {:?}", e);
                String::new()
            }
        }
    });

    println!("result: {}", result);

    result.replace("\"", "")
}

#[tauri::command]
fn send_conversation(blob: Vec<u8>) -> String {
    println!("blob: {:?}", blob);
    println!("ここまできました。123123123123123123123123");

    String::new()
}

#[tokio::main]
async fn main() {
    tauri::Builder
        ::default()
        .invoke_handler(tauri::generate_handler![question, send_conversation])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
