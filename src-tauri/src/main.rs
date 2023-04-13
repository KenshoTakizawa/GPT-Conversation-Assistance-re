// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use reqwest::{ header, blocking::Client };
use std::collections::HashMap;
use maplit::hashmap;

use reqwest::Client;
use serde::{ Deserialize, Serialize };
use tokio::runtime::Runtime;

#[derive(Serialize, Deserialize)]
struct Prompt {
    prompt: String,
    max_tokens: u32,
    n: u32,
    temperature: f64,
    top_p: f64,
}

#[derive(Serialize, Deserialize)]
struct ApiResponse {
    id: String,
    object: String,
    created: u64,
    model: String,
    choices: Vec<Choice>,
}

#[derive(Serialize, Deserialize)]
struct Choice {
    text: String,
    index: usize,
    logprobs: Option<serde_json::Value>,
    finish_reason: String,
}

async fn call_gpt_api(prompt: &str) -> Result<String, reqwest::Error> {
    let api_key = "xxxx";

    let client = Client::new();
    let prompt = Prompt {
        prompt: prompt.to_string(),
        max_tokens: 50,
        n: 1,
        temperature: 0.8,
        top_p: 1.0,
    };

    let response = client
        .post("https://api.openai.com/v1/engines/davinci-codex/completions")
        .header("Authorization", format!("Bearer {}", api_key))
        .json(&prompt)
        .send().await?;

    let api_response: ApiResponse = response.json().await?;
    let response_text = api_response.choices.get(0).unwrap().text.clone();
    Ok(response_text)
}

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

fn main() {
    let rt = Runtime::new().unwrap();

    let prompt = "数学とはなにか、かんたんに教えて。";

    match rt.block_on(call_gpt_api(prompt)) {
        Ok(response) => println!("GPT Response: {}", response),
        Err(error) => eprintln!("Error calling GPT API: {}", error),
    }

    // let rt = Runtime::new().unwrap();

    // let req = RequestBuilder::new(
    //     CompletionModel::TextDavinci003,
    //     "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"
    // )
    //     .prompt("武田信玄についておしえて。")
    //     .build_completion();

    // let api_response = rt.block_on(req.send()).unwrap();
    // println!("{:?}", api_response);
    // let response_text = api_response.choices.get(0).unwrap().text.clone();
    // println!("My bot replied with: \"{}\"", response_text);

    let api_key = "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"; // ここにAPIキーを入力してください。

    // リクエストパラメーター
    let url = "https://api.openai.com/v1/chat/completions";
    let mut headers = header::HeaderMap::new();
    headers.insert(header::CONTENT_TYPE, header::HeaderValue::from_static("application/json"));
    headers.insert(
        header::AUTHORIZATION,
        header::HeaderValue::from_str(&format!("Bearer {}", api_key)).unwrap()
    );

    let messages = vec![
        hashmap! {
        "role" => "user",
        "content" => "Say this is a test!"
    }
    ];
    let params = HashMap::from([
        ("model", "gpt-3.5-turbo"),
        ("messages", "what is human?"),
        ("temperature", "0.7"),
    ]);
    let client = Client::new();
    let response = client.post(url).headers(headers).json(&params).send();

    // レスポンスの処理
    match response {
        Ok(res) => {
            if res.status().is_success() {
                let result: serde_json::Value = res.json().unwrap();
                println!("Response: {:?}", result);
            } else {
                println!("Request failed with status code: {}", res.status());
            }
        }
        Err(e) => {
            println!("Error: {}", e);
        }
    }

    tauri::Builder
        ::default()
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
