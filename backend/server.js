const express = require("express");
const { spawn } = require("child_process");
const path = require("path");
const cors = require("cors");

const app = express();
const port = 3000;

app.use(cors());

const pathToScript = path.join(__dirname, "script.py");
const pathToRecognize = path.join(__dirname, "main.py");
const pythonExecutable = path.join(__dirname, "..", "backend", "venv", "Scripts", "python");

const runPythonScript = (scriptPath, args, callback) => {
  const process = spawn(pythonExecutable, [scriptPath, ...args]);

  let result = "";
  let errorOutput = "";

  process.stdout.on("data", (data) => {
    result += data.toString();
  });

  process.stderr.on("data", (data) => {
    errorOutput += data.toString();
  });

  process.on("close", (code) => {
    if (code === 0) {
      callback(null, result);
    } else {
      callback(`Скрипт завершился с кодом ошибки ${code}. Вывод: ${result}\nОшибка: ${errorOutput}`);
    }
  });

  process.on("error", (error) => {
    callback(`Ошибка при запуске скрипта: ${error.message}`);
  });
};

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

const recognizeSpeech = () => {
  runPythonScript(pathToRecognize, [], (error, result) => {
    if (error) {
      console.error(error);
    } else {
      console.log(`Скрипт успешно выполнен: ${result}`);
    }
  });
};

recognizeSpeech();

app.get("/recognize-speech", (req, res) => {
  res.send("Распознавание речи выполнено при старте приложения");
});

app.listen(port, () => {
  console.log(`Сервер запущен на порту ${port}`);
});
