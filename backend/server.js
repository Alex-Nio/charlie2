const express = require("express");
const { exec } = require("child_process");
const cors = require("cors");

const app = express();
const port = 3000;

app.use(cors()); // Добавьте эту строку для обработки CORS

const pathToScript = __dirname + "/script.py";

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

app.listen(port, () => {
  console.log(`Сервер запущен на порту ${port}`);
});
