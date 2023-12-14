use std::os::windows::process::CommandExt;
use std::process::{Child, Command};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;
use crate::events;

// Общая переменная для использования внутри асинхронных функций
// Мьютекс (Mutex) используется для обеспечения безопасности доступа к структуре TTSProcess
lazy_static::lazy_static! {
    static ref TTS_PROCESS: Arc<Mutex<TTSProcess>> = Arc::new(Mutex::new(TTSProcess::new()));
}

// Структура, чтобы хранить информацию о процессе TTS
struct TTSProcess {
    child: Option<Child>,
    stop_requested: bool,
}

impl TTSProcess {
    fn new() -> Self {
        TTSProcess {
            child: None,
            stop_requested: false,
        }
    }

    fn stop_tts_static(process: &mut TTSProcess) {
        // Прерываем процесс TTS, если он запущен
        if let Some(mut child) = process.child.take() {
            // Посылаем сигнал завершения процесса
            let _ = child.kill();
            // Ждем завершения процесса
            let _ = child.wait();
        }

        // Добавьте небольшую задержку перед возвращением из функции
        // Это может дать процессу TTS немного времени на полное завершение
        thread::sleep(Duration::from_millis(200));
    }

    fn start_tts(&mut self, text: &str) -> Result<(), String> {
        println!("TTS процесс запущен");

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
            events::tts_stoped(TAURI_APP_HANDLE.get().unwrap());

            return Ok(());
        }

        let mut command = if cfg!(target_os = "windows") {
            Command::new("python")
        } else {
            Command::new("python3")
        };

        // Добавляем параметры для команды
        command.arg("src/tts/tts_module.pyw").arg(text);

        // tts_started
        events::tts_started(TAURI_APP_HANDLE.get().unwrap());

        // Если цель - Windows, устанавливаем CREATE_NO_WINDOW
        // if cfg!(target_os = "windows") {
        //     command.creation_flags(winapi::um::winbase::CREATE_NO_WINDOW);
        // }

        println!("Подготовка TTS...");

        // Сохраняем Child-процесс для последующего управления им
        match command.spawn() {
            Ok(child) => {
                self.child = Some(child);

                // Ожидаем завершения процесса
                if let Some(mut child) = self.child.take() {
                    match child.wait() {
                        Ok(exit_status) => {
                            if exit_status.success() {
                                println!("TTS процесс успешно завершен");
                                events::tts_stoped(TAURI_APP_HANDLE.get().unwrap());
                            } else {
                                eprintln!("TTS процесс завершился с ошибкой: {:?}", exit_status);
                            }
                        }
                        Err(err) => {
                            eprintln!("Ошибка при ожидании завершения процесса: {}", err);
                        }
                    }
                }

                Ok(())
            }
            Err(err) => Err(format!("Error running Python script: {}", err)),
        }
    }

    #[allow(dead_code)]
    fn stop_tts(&mut self) {
        self.stop_requested = true;
        // Прерываем процесс TTS, если он запущен
        if let Some(mut child) = self.child.take() {
            events::tts_stoped(TAURI_APP_HANDLE.get().unwrap());

            // Посылаем сигнал завершения процесса
            let _ = child.kill();
            // Ждем завершения процесса
            let _ = child.wait();
        }
    }
}
use crate::tauri_commands::TAURI_APP_HANDLE;

#[tauri::command]
pub async fn speak_text(text: String) -> Result<(), String> {
    // Создаем копию текста
    let text_copy = text.clone();

    // Получаем доступ к общей переменной
    let tts_process = TTS_PROCESS.clone();

    // Запускаем TTS в отдельном потоке
    let handle = thread::spawn(move || {
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
    // Получаем доступ к общей переменной
    let tts_process = TTS_PROCESS.clone();

    // Останавливаем TTS в отдельном потоке
    thread::spawn(move || {
        let mut tts_process = tts_process.lock().unwrap();
        TTSProcess::stop_tts_static(&mut tts_process);
    });

    Ok(())
}
