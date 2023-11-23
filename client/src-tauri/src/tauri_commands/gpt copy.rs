use serde_json::{json, Value};
use std::{
    env,
    fs::File,
    io::{BufReader, Read, Write},
    sync::Mutex,
};
use tokio::{self, runtime::Runtime};
use tts_rust::tts::GTTSClient;
use tts_rust::languages::Languages;

//* GPT
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
    send_request_to_chatgpt(text).await
}

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

    println!("Запрос к API: {:?}", text);

    // Create a JSON object with the request payload
    let request_payload = json!({
        "model": "gpt-3.5-turbo-1106",
        "messages": [
            {"role": "system", "content": "Your system message here"},
            {"role": "user", "content": text}
        ],
        "max_tokens": 100,
        "n": 1,
        "stop": null
    });

    // println!("API Request: {:?}", request_payload);

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

                                // Воспроизводим текст с использованием TTS
                                if let Err(err) = speak_text(content_text).await {
                                    eprintln!("Error speaking text: {}", err);
                                }
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

// Асинхронная функция для воспроизведения текста с использованием TTS
async fn speak_text(text: &str) -> Result<(), tts::Error> {
    let narrator: GTTSClient = GTTSClient {
        volume: 1.0,
        language: Languages::Russian,
        tld: "com",
    };

    println!("TEXT For TTS: {}", text);
    // Ваш код для воспроизведения текста с использованием библиотеки TTS
    // Например, используйте tts::say(text).await; или аналогичный код
    // tts::say(text).await?;
    narrator.speak(text);

    Ok(())
}
