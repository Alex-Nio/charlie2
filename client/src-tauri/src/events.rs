use crate::events;
use crate::tauri_commands::TAURI_APP_HANDLE;
use tauri::Manager;

// the payload type must implement `Serialize` and `Clone`.
#[derive(Clone, serde::Serialize)]
pub struct Payload {
    pub data: String,
    pub folder: String,
}

#[allow(dead_code)]
pub enum EventTypes {
    AudioPlay,
    AssistantWaiting,
    AssistantGreet,
    CommandStart,
    CommandInProcess,
    CommandEnd,
    TtsStarted,
    TtsStopped,
}

impl EventTypes {
    pub fn get(&self) -> &str {
        match self {
            Self::AudioPlay => "audio-play",
            Self::AssistantWaiting => "assistant-waiting",
            Self::AssistantGreet => "assistant-greet",
            Self::CommandStart => "command-start",
            Self::CommandInProcess => "command-in-process",
            Self::CommandEnd => "command-end",
            Self::TtsStarted => "tts-started",
            Self::TtsStopped => "tts-stopped",
        }
    }
}

pub fn play(phrase: &str, app_handle: &tauri::AppHandle, folder: &str) {
    // Воспроизводим аудио
    app_handle
        .emit_all(
            EventTypes::AudioPlay.get(),
            Payload {
                data: phrase.into(),
                folder: folder.into(),
            },
        )
        .unwrap();
}

pub fn tts_started(app_handle: &tauri::AppHandle) {
    println!("Вызов emit tts started...");

    events::play("load", TAURI_APP_HANDLE.get().unwrap(), "default");

    app_handle
        .emit_all(
            EventTypes::TtsStarted.get(),
            Payload {
                data: "".into(),
                folder: "".into(),
            },
        )
        .unwrap();
}

pub fn tts_stopped(app_handle: &tauri::AppHandle) {
    println!("Вызов emit tts stopped...");

    app_handle
        .emit_all(
            EventTypes::TtsStopped.get(),
            Payload {
                data: "".into(),
                folder: "".into(),
            },
        )
        .unwrap();
}
