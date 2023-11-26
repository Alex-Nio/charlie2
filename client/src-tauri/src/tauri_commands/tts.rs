use std::process::{Command, Child};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;


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
            return Ok(());
        }

        // Создаем команду для запуска Python-скрипта
        let mut command = if cfg!(target_os = "windows") {
            Command::new("python")
        } else {
            Command::new("python3")
        };

        println!("Подготовка TTS...");

        // Устанавливаем путь к скрипту и аргументы
        let command = command
            .arg("src/tts/tts_module.pyw")
            .arg(text)
            .spawn();

        // Сохраняем Child-процесс для последующего управления им
        match command {
            Ok(child) => {
                self.child = Some(child);
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
            // Посылаем сигнал завершения процесса
            let _ = child.kill();
            // Ждем завершения процесса
            let _ = child.wait();
        }
    }
}

// Общая переменная для использования внутри асинхронных функций
// Мьютекс (Mutex) используется для обеспечения безопасности доступа к структуре TTSProcess
lazy_static::lazy_static! {
    static ref TTS_PROCESS: Arc<Mutex<TTSProcess>> = Arc::new(Mutex::new(TTSProcess::new()));
}

#[tauri::command]
pub async fn speak_text(text: String) -> Result<(), String> {
    // Создаем копию текста
    let text_copy = text.clone();

    // Получаем доступ к общей переменной
    let tts_process = TTS_PROCESS.clone();

    // Запускаем TTS в отдельном потоке
    thread::spawn(move || {
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
