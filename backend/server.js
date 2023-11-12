const express = require("express");
const { exec } = require("child_process");
const cors = require("cors");

const app = express();
const port = 3000;

app.use(cors()); // Добавьте эту строку для обработки CORS

const pathToScript = __dirname + "/script.py";
const pathToRecognize = __dirname + "/main.py";

// Скрипт взаимодействия с интерфейсом
app.get("/run-python-script", (req, res) => {
  exec(`python ${pathToScript}`, (error, stdout, stderr) => {
    if (error) {
      console.error(`Ошибка выполнения скрипта: ${error.message}`);
      res.status(500).send("Internal Server Error");
      return;
    }
    console.log(`Скрипт успешно выполнен: ${stdout}`);
    res.send(stdout);
  });
});

// Функция для распознавания речи
const recognizeSpeech = () => {
  exec(`set PYTHONIOENCODING=utf-8 && python ${pathToRecognize}`, (error, stdout, stderr) => {
    if (error) {
      console.error(`Ошибка выполнения скрипта: ${error.message}`);
      return;
    }
    console.log(`Скрипт успешно выполнен: ${stdout}`);
  });
};

// Вызываем функцию при инициализации сервера
recognizeSpeech();

app.get("/recognize-speech", (req, res) => {
  res.send("Распознавание речи выполнено при старте приложения");
});

app.listen(port, () => {
  console.log(`Сервер запущен на порту ${port}`);
});
