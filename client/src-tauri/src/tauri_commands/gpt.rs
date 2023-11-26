use regex::Regex;
use serde_json::{json, Value};
use std::{
    env,
    fs::File,
    io::{BufReader, Read, Write},
    sync::Mutex,
};
use std::fs::OpenOptions;
use tokio::{self, runtime::Runtime};
use crate::tauri_commands;

lazy_static! {
    static ref FILE_MUTEX: Mutex<()> = Mutex::new(());
}

#[tauri::command]
pub fn write_to_file(output: &str) {
    let _lock = FILE_MUTEX.lock().unwrap(); // блокируем мьютекс при записи в файл

    let mut file = File::create("output.txt").expect("Error creating file");
    writeln!(file, "{}", output.to_lowercase()).expect("Error writing to file");
}

#[tauri::command]
pub fn write_code_to_file(response: &str) {
    let json_response: Value = serde_json::from_str(response).unwrap(); // Распаковка JSON

    // Доступ к полю content
    if let Some(choices) = json_response
        .get("choices")
        .and_then(|choices| choices.as_array())
    {
        if let Some(first_choice) = choices.get(0) {
            if let Some(message) = first_choice.get("message") {
                if let Some(content) = message.get("content") {
                    if let Some(content_text) = content.as_str() {
                        // Используем регулярное выражение для поиска текста между "```"
                        let re = Regex::new(r"```([\s\S]+?)```").unwrap();

                        // Открываем файл в режиме записи, добавляя новый контент
                        let mut file = OpenOptions::new()
                            .write(true)
                            .truncate(true)
                            .create(true)
                            .open("code.txt")
                            .expect("Error opening file");

                        for cap in re.captures_iter(content_text) {
                            if let Some(code) = cap.get(1) {
                                if !code.as_str().is_empty() {
                                    writeln!(file, "{}", code.as_str()).expect("Error writing to file");
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

#[tauri::command]
pub fn read_output_text() -> Result<String, std::io::Error> {
    let _lock = FILE_MUTEX.lock().unwrap(); // блокируем мьютекс при чтении из файла

    let file = File::open("output.txt")?;
    let mut reader = BufReader::new(file);
    let mut output_text = String::new();
    reader.read_to_string(&mut output_text)?;

    Ok(output_text)
}

#[tauri::command]
pub fn read_output_text_and_process() {
    let output_text = match read_output_text() {
        Ok(output_text) => output_text,
        Err(err) => {
            eprintln!("Error reading output.txt: {}", err);
            return;
        }
    };

    // Создаем асинхронный runtime для вызова асинхронной функции
    let rt = Runtime::new().unwrap();

    // Запуск асинхронной задачи
    rt.block_on(async {
        process_chatgpt_response(output_text.into()).await;
    });
}

#[tauri::command]
pub async fn send_request_to_chatgpt_api_async(text: &str) -> Result<String, reqwest::Error> {
    if text.contains("стоп") {
        println!("Cancellation detected. Exiting API request.");
        return Ok(String::new()); // Возвращаем пустую строку, так как запрос прерван
    }

    send_request_to_chatgpt(text).await
}

use std::sync::atomic::{AtomicUsize, Ordering};

// Определение атомарной переменной для подсчета выполненных запросов
static REQUEST_COUNT: AtomicUsize = AtomicUsize::new(0);

#[tauri::command]
pub async fn send_request_to_chatgpt(text: &str) -> Result<String, reqwest::Error> {
    // Check if text contains only newline character
    if text == "\n" {
        // Code...
        panic!("Text contains only a newline character");
    }

    // Load environment variables from the .env file
    dotenv::dotenv().expect("Failed to load .env file");

    // Replace with your actual API key
    let api_key = env::var("API_KEY").expect("API_KEY not found in .env file. Please add it.");

    let url = "https://api.openai.com/v1/chat/completions";

    // Get the previous context from the static variable
    let previous_context = PREVIOUS_CONTEXT.lock().unwrap().clone();

    // Clone the previous context before adding the new text
    let previous_context_clone = previous_context.clone();

    // Create a JSON object with the request payload, including the previous context
    let request_payload = json!({
        "model": "gpt-3.5-turbo-1106",
        "messages": [
            {"role": "system", "content": "Ты голосовой ассистент по имени Чарли, отвечаешь на русском языке"},
            {"role": "user", "content": previous_context_clone + text}
        ],
        "max_tokens": 250,
        "n": 1,
        "stop": null
    });

    // Update the static variable with the current context
    *PREVIOUS_CONTEXT.lock().unwrap() = format!("{}{}", previous_context, text);

    // Увеличение счетчика выполненных запросов
    let request_count = REQUEST_COUNT.fetch_add(1, Ordering::SeqCst);

    // Если выполнено три запроса, сброс контекста
    if request_count >= 0 {
        *PREVIOUS_CONTEXT.lock().unwrap() = String::new();
        // Сброс счетчика запросов
        REQUEST_COUNT.store(0, Ordering::SeqCst);
    }

    // Send a POST request to the ChatGPT API
    let response = reqwest::Client::new()
        .post(url)
        .header("Authorization", format!("Bearer {}", api_key))
        .header("Content-Type", "application/json")
        .json(&request_payload)
        .send()
        .await;

    // Parse and return the response
    let response_text = response?.text().await?;
    Ok(response_text)
}

// Define a static variable to store the previous context
lazy_static::lazy_static! {
    static ref PREVIOUS_CONTEXT: Mutex<String> = Mutex::new(String::new());
}

#[tauri::command]
pub async fn process_chatgpt_response(text: String) {
    match send_request_to_chatgpt_api_async(&text).await {
        Ok(response) => {
            // Обработка ответа от ChatGPT API
            let json_response: Value = serde_json::from_str(&response).unwrap(); // Распаковка JSON

            // Доступ к полю content
            if let Some(choices) = json_response
                .get("choices")
                .and_then(|choices| choices.as_array())
            {
                if let Some(first_choice) = choices.get(0) {
                    if let Some(message) = first_choice.get("message") {
                        if let Some(content) = message.get("content") {
                            if let Some(content_text) = content.as_str() {
                                println!("Ответ от Api: {}", content_text);

                                // Записываем код в файл
                                if content_text.contains("javascript") {
                                    write_code_to_file(&response);
                                }

                                // Воспроизводим текст с использованием TTS
                                if let Err(err) = tauri_commands::speak_text((&content_text).to_string()).await {
                                    eprintln!("Error speaking text: {}", err);
                                }

                                println!("TTS End with Ok status");
                            }
                        }
                    }
                }
            }
        }
        Err(err) => {
            eprintln!("Error communicating with ChatGPT API: {}", err);
        }
    }
}

