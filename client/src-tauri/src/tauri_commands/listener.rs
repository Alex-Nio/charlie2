use crate::tauri_commands::TTS_PROCESS;
use log::info;
use once_cell::sync::OnceCell;
use rand::seq::SliceRandom;
use std::{
    sync::atomic::{AtomicBool, Ordering},
    thread,
    time::SystemTime,
};
use tauri::Manager;

use crate::{assistant_commands, config, events, recorder, tauri_commands, vosk, COMMANDS, DB};

// track listening state
static LISTENING: AtomicBool = AtomicBool::new(false);

// stop listening with Atomic flag (to make it work between different threads)
static STOP_LISTENING: AtomicBool = AtomicBool::new(false);

// store tauri app_handle
pub static TAURI_APP_HANDLE: OnceCell<tauri::AppHandle> = OnceCell::new();

#[tauri::command]
pub fn is_listening() -> bool {
    LISTENING.load(Ordering::SeqCst)
}

#[tauri::command]
pub fn stop_listening() {
    if is_listening() {
        STOP_LISTENING.store(true, Ordering::SeqCst);
        stop_recording();
    }

    // wait until listening stops
    while is_listening() {}
}

fn get_wake_word_engine() -> config::WakeWordEngine {
    let selected_wake_word_engine;
    if let Some(wwengine) = DB
        .lock()
        .unwrap()
        .get::<String>("selected_wake_word_engine")
    {
        // from db
        match wwengine.trim().to_lowercase().as_str() {
            "vosk" => selected_wake_word_engine = config::WakeWordEngine::Vosk,
            _ => selected_wake_word_engine = config::DEFAULT_WAKE_WORD_ENGINE,
        }
    } else {
        // default
        selected_wake_word_engine = config::DEFAULT_WAKE_WORD_ENGINE; // set default wake_word engine
    }

    selected_wake_word_engine
}

#[tauri::command(async)]
pub fn start_listening(app_handle: tauri::AppHandle) -> Result<bool, String> {
    // only one listener thread is allowed
    if is_listening() {
        return Err("Already listening.".into());
    }

    // keep app handle
    if TAURI_APP_HANDLE.get().is_none() {
        let _ = TAURI_APP_HANDLE.set(app_handle);
    }

    // call selected wake-word engine listener command
    match get_wake_word_engine() {
        config::WakeWordEngine::Vosk => {
            info!("Starting VOSK wake-word engine ...");
            return vosk_init();
        }
    }
}

fn keyword_callback(_keyword_index: i32) {
    // vars
    let mut start: SystemTime = SystemTime::now();
    let mut frame_buffer = vec![0; recorder::FRAME_LENGTH.load(Ordering::SeqCst) as usize];

    // play callback phrase
    events::play(
        config::ASSISTANT_CALLBACK_PHRASES
            .choose(&mut rand::thread_rng())
            .unwrap(),
        TAURI_APP_HANDLE.get().unwrap(),
        "callback",
    );

    // emit assistant greet event
    TAURI_APP_HANDLE
        .get()
        .unwrap()
        .emit_all(events::EventTypes::AssistantGreet.get(), ())
        .unwrap();

    // the loop
    while !STOP_LISTENING.load(Ordering::SeqCst) {
        recorder::read_microphone(&mut frame_buffer);

        // vosk part (partials included)
        if let Some(mut test) = vosk::recognize(&frame_buffer, false) {
            if !test.is_empty() {
                // some filtration
                test = test.to_lowercase();

                for tbr in config::ASSISTANT_PHRASES_TBR {
                    test = test.replace(tbr, "");
                }

                test = test.trim().into();

                // infer command
                if let Some((cmd_path, cmd_config)) =
                    assistant_commands::fetch_command(&test, &COMMANDS)
                {
                    // println!("Recognized (filtered): {}", test);
                    println!("[+] Команда найдена: {:?}", cmd_path);
                    println!("Исполнение ...");
                    println!("[+] Конфигурация команды: {:?}", cmd_config);

                    let cmd_result = assistant_commands::execute_command(
                        &cmd_path,
                        &cmd_config,
                        TAURI_APP_HANDLE.get().unwrap(),
                        &test,
                    );

                    match cmd_result {
                        Ok(chain) => {
                            println!("Command executed successfully!");

                            if chain {
                                // continue chaining commands
                                start = SystemTime::now(); // listen for more commands
                            } else {
                                // skip forward if chaining is not required
                                start = start
                                    .checked_sub(core::time::Duration::from_secs(500))
                                    .unwrap();
                            }

                            continue;
                        }
                        Err(error_message) => {
                            println!("Error executing command: {}", error_message);
                        }
                    }

                    TAURI_APP_HANDLE
                        .get()
                        .unwrap()
                        .emit_all(events::EventTypes::AssistantWaiting.get(), ())
                        .unwrap();

                    events::play(
                        config::ASSISTANT_WAIT_PHRASES
                            .choose(&mut rand::thread_rng())
                            .unwrap(),
                        TAURI_APP_HANDLE.get().unwrap(),
                        "wait",
                    );

                    break; // return to picovoice after command execution (no matter successful or not)
                } else {
                    tauri_commands::write_to_file(&test);

                    // ASSISTANT_STOP_PHRASES
                    if config::ASSISTANT_STOP_PHRASES.contains(&test.as_str()) {
                        let _ = tauri_commands::stop_tts();

                        println!("Выход из цикла. Остановка TTS");

                        events::play("stop", TAURI_APP_HANDLE.get().unwrap(), "default");

                        break;
                    }

                    match tauri_commands::read_output_text() {
                        Ok(output_text) => {
                            // Do something with the text
                            println!("Text from output.txt: {}", output_text);

                            // Создаем асинхронный runtime для вызова асинхронной функции
                            if !output_text.trim().is_empty() {
                                // tts_started
                                events::tts_started(TAURI_APP_HANDLE.get().unwrap());

                                println!("[+] Чтение текста из файла output.txt...");

                                // Явное создание асинхронного runtime
                                tauri_commands::write_to_file(&test);

                                // Чтение и обработка текста из файла в новом процессе
                                let handle = thread::spawn(|| {
                                    // Ваш код внутри потока
                                    let result = std::panic::catch_unwind(|| {
                                        tauri_commands::read_output_text_and_process();
                                    });

                                    if let Err(err) = result {
                                        eprintln!("Thread panicked: {:?}", err);
                                        // Здесь вы можете выполнить какие-то действия по обработке паники
                                    }
                                });

                                handle.join().expect("Thread panicked");
                            } else {
                                println!("[+] Файл output.txt пуст.");
                            }
                        }
                        Err(err) => {
                            eprintln!("Error reading output.txt: {}", err);
                        }
                    }
                }
            }
        }

        match start.elapsed() {
            Ok(elapsed) if elapsed > config::CMS_WAIT_DELAY => {
                let tts_process = TTS_PROCESS.lock().unwrap();

                println!("ТТС процесс запущен: {}", !tts_process.is_active());

                // Проверяем, было ли уже проиграно ожидание
                if !tts_process.is_active() {
                    // return to vosk after N seconds
                    TAURI_APP_HANDLE
                        .get()
                        .unwrap()
                        .emit_all(
                            events::EventTypes::AssistantWaiting.get(),
                            events::play(
                                config::ASSISTANT_WAIT_PHRASES
                                    .choose(&mut rand::thread_rng())
                                    .unwrap(),
                                TAURI_APP_HANDLE.get().unwrap(),
                                "wait",
                            ),
                        )
                        .unwrap();
                }

                break;
            }
            _ => (),
        }
    }
}

pub fn data_callback(frame_buffer: &[i16]) {
    match get_wake_word_engine() {
        config::WakeWordEngine::Vosk => {
            // recognize & convert to sequence
            let recognized_phrase = vosk::recognize(&frame_buffer, true).unwrap_or("".into());

            if !recognized_phrase.trim().is_empty() {
                info!("Rec: {}", recognized_phrase);
                let recognized_phrases = recognized_phrase.split_whitespace();

                println!("[+] Input phrase: {:?}", recognized_phrase);

                for phrase in recognized_phrases {
                    let recognized_phrase_chars =
                        phrase.trim().to_lowercase().chars().collect::<Vec<_>>();

                    // compare
                    let compare_ratio = seqdiff::ratio(
                        &config::VOSK_FETCH_PHRASE.chars().collect::<Vec<_>>(),
                        &recognized_phrase_chars,
                    );
                    info!("OG phrase: {:?}", &config::VOSK_FETCH_PHRASE);
                    info!("Recognized phrase: {:?}", &recognized_phrase_chars);
                    info!("Compare ratio: {}", compare_ratio);

                    if compare_ratio >= config::VOSK_MIN_RATIO {
                        info!("Phrase activated.");
                        keyword_callback(0);
                        break;
                    }
                }
            }
        }
    }
}

fn start_recording() -> Result<bool, String> {
    // vars
    let frame_length: usize;

    // identity frame length
    match get_wake_word_engine() {
        config::WakeWordEngine::Vosk => {
            // start recording for Vosk
            frame_length = 128;
            recorder::FRAME_LENGTH.store(frame_length as u32, Ordering::SeqCst);
        }
    }

    // define frame buffer
    let mut frame_buffer: Vec<i16> = vec![0; frame_length];

    // init stuff
    recorder::init(); // init
    recorder::start_recording(); // start

    LISTENING.store(true, Ordering::SeqCst);
    info!("[+] Запуск слушателя...");

    // greet user
    events::play(
        config::ASSISTANT_GREET_PHRASES
            .choose(&mut rand::thread_rng())
            .unwrap(),
        TAURI_APP_HANDLE.get().unwrap(),
        "greet",
    );

    // record
    match recorder::RECORDER_TYPE.load(Ordering::SeqCst) {
        recorder::RecorderType::PvRecorder => {
            while !STOP_LISTENING.load(Ordering::SeqCst) {
                recorder::read_microphone(&mut frame_buffer);
                data_callback(&frame_buffer);
            }

            // stop
            stop_recording();

            Ok(true)
        }
    }
}

fn stop_recording() {
    // Stop listening
    recorder::stop_recording();

    LISTENING.store(false, Ordering::SeqCst);
    STOP_LISTENING.store(false, Ordering::SeqCst);
    info!("[+] Остановка слушателя...");
}

fn vosk_init() -> Result<bool, String> {
    start_recording()
}
