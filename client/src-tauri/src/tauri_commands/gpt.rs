// функция для отправки запроса в ChatGPT
async fn send_request_to_chatgpt(input: &str) -> Result<String, String> {
  // Замените URL на ваш URL ChatGPT API
  let url = "https://api.chatgpt.com/v1/chat"; // замените на ваш URL

  // Определите ваш токен авторизации для ChatGPT API
  let authorization_token = "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"; // замените на ваш токен

  // Создайте JSON-объект для запроса
  let request_body = json!({
      "messages": [
          {
              "role": "system",
              "content": "user"
          },
          {
              "role": "user",
              "content": input
          }
      ]
  });

  // Отправьте POST-запрос к ChatGPT API
  let client = reqwest::blocking::Client::new();
  let response = client
      .post(url)
      .header("Authorization", format!("Bearer {}", authorization_token))
      .header("Content-Type", "application/json")
      .json(&request_body)
      .send();

  // Обработка ответа
  match response {
      Ok(resp) => {
          if resp.status().is_success() {
              let body = resp.text().map_err(|e| format!("Error reading response body: {}", e));
              Ok(body?)
          } else {
              Err(format!("ChatGPT API returned an error: {}", resp.status()))
          }
      }
      Err(err) => Err(format!("Error sending request to ChatGPT: {}", err)),
  }
}
