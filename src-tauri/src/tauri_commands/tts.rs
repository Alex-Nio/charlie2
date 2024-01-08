use crate::events;
use crate::tauri_commands::TAURI_APP_HANDLE;
use dotenv::dotenv;
use std::env;
// use std::os::windows::process::CommandExt;
use std::process::{Child, Command};
// use std::process::{Stdio};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

// Структура, чтобы хранить информацию о процессе TTS
pub struct TTSProcess {
    child: Option<Child>,
    stop_requested: bool,
    active: bool,
}

impl TTSProcess {
    fn new() -> Self {
        TTSProcess {
            child: None,
            stop_requested: false,
            active: false,
        }
    }

    pub fn is_active(&self) -> bool {
        self.active
    }

    fn stop_tts_static(process: &mut TTSProcess) {
        // Прерываем процесс TTS, если он запущен
        if let Some(mut child) = process.child.take() {
            events::tts_stopped(TAURI_APP_HANDLE.get().unwrap());

            // Посылаем сигнал завершения процесса
            let _ = child.kill();
            // Ждем завершения процесса
            let _ = child.wait();
        }

        // Добавьте небольшую задержку перед возвращением из функции
        // Это может дать процессу TTS немного времени на полное завершение
        thread::sleep(Duration::from_millis(200));

        process.active = false;
        events::tts_stopped(TAURI_APP_HANDLE.get().unwrap());
    }

    fn start_tts(&mut self, text: &str) -> Result<(), String> {
        println!("TTS процесс запущен");

        self.active = true;

        // Проверяем, не запущен ли уже процесс TTS
        if self.child.is_some() {
            // Ожидаем завершения предыдущего процесса
            if let Some(mut child) = self.child.take() {
                let _ = child.wait();
            }
        }

        // Проверяем, был ли запрошен останов TTS
        if self.stop_requested {
            self.stop_requested = false; // Сбрасываем флаг
            events::tts_stopped(TAURI_APP_HANDLE.get().unwrap());

            return Ok(());
        }

        // Загрузить переменные окружения из файла .env
        dotenv().ok();

        // Получить значение переменной PYTHON_PATH из переменных окружения
        let python_path = env::var("PYTHON_PATH").expect("PYTHON_PATH not set in .env");

        // Путь к исполняемому файлу Python
        let mut command = Command::new(&python_path);

        // Добавляем параметры для команды
        command
            .arg("src/tts/tts_module.pyw")
            .arg("--text")
            .arg(text);

        // Если цель - Windows, устанавливаем CREATE_NO_WINDOW
        // if cfg!(target_os = "windows") {
        //     let stdout = Stdio::null();
        //     let stderr = Stdio::null();

        //     // Устанавливаем флаги создания процесса
        //     command
        //         .stdout(stdout)
        //         .stderr(stderr)
        //         .creation_flags(winapi::um::winbase::CREATE_NO_WINDOW);
        // }

        println!("Подготовка TTS...");

        // Сохраняем Child-процесс для последующего управления им
        match command.spawn() {
            Ok(child) => {
                self.child = Some(child);

                Ok(())
            }
            Err(err) => Err(format!("Error running Python script: {}", err)),
        }
    }
}

// Общая переменная для использования внутри асинхронных функций
// Мьютекс (Mutex) используется для обеспечения безопасности доступа к структуре TTSProcess
lazy_static::lazy_static! {
    pub static ref TTS_PROCESS: Arc<Mutex<TTSProcess>> = Arc::new(Mutex::new(TTSProcess::new()));
}

#[tauri::command]
pub async fn speak_text(text: String) -> Result<(), String> {
    // Создаем копию текста
    let text_copy = text.clone();

    // Получаем доступ к общей переменной
    let tts_process = TTS_PROCESS.clone();

    // Запускаем TTS в отдельном потоке
    thread::spawn(move || {
        // Запускаем TTS
        let mut tts_process = tts_process.lock().unwrap();

        if let Err(err) = tts_process.start_tts(&text_copy) {
            eprintln!("Failed to start TTS: {}", err);
        }
    });

    Ok(())
}

#[tauri::command]
pub fn stop_tts() -> Result<(), String> {
    events::tts_stopped(TAURI_APP_HANDLE.get().unwrap());

    // Получаем доступ к общей переменной
    let tts_process = TTS_PROCESS.clone();

    // Останавливаем TTS в отдельном потоке
    thread::spawn(move || {
        let mut tts_process = tts_process.lock().unwrap();
        TTSProcess::stop_tts_static(&mut tts_process);
    });

    Ok(())
}
