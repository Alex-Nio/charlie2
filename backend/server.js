const express = require("express");
const { spawn, exec } = require("child_process");
const path = require("path");
const cors = require("cors");
const fs = require("fs");

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

// Function to send recognition updates
const sendRecognitionUpdate = (res) => {
  try {
    const recognizedSpeech = fs.readFileSync("recognized_speech.txt", { encoding: "utf-8" });
    res.write(`data: ${recognizedSpeech}\n\n`);
  } catch (error) {
    console.error("Ошибка при чтении файла recognized_speech.txt:", error);
  }
};

// Script interaction with the interface
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

// Recognize speech endpoint
const recognizeSpeech = () => {
  runPythonScript(pathToRecognize, [], (error, result) => {
    if (error) {
      console.error(error);
    } else {
      console.log(`Скрипт успешно выполнен: ${result}`);
    }
  });
};

// Endpoint to get recognized speech
app.get("/get-recognized-speech", (req, res) => {
  try {
    const recognizedSpeech = fs.readFileSync("recognized_speech.txt", { encoding: "utf-8" });
    res.send(recognizedSpeech);
  } catch (error) {
    console.error("Ошибка при чтении файла recognized_speech.txt:", error);
    res.status(500).send("Internal Server Error");
  }
});

// Endpoint to set up server-sent events
app.get("/recognition-updates", (req, res) => {
  // Set headers for server-sent events
  res.setHeader("Content-Type", "text/event-stream");
  res.setHeader("Cache-Control", "no-cache");
  res.setHeader("Connection", "keep-alive");

  // Send a server-sent event when the file changes
  fs.watchFile("recognized_speech.txt", (curr, prev) => {
    sendRecognitionUpdate(res);
  });

  // // Send an empty event to establish the connection
  // res.write("");
});

app.listen(port, () => {
  console.log(`Сервер запущен на порту ${port}`);
  recognizeSpeech();
});
