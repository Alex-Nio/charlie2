use crate::events;
use crate::tauri_commands::TAURI_APP_HANDLE;
use tauri::Manager;
// the payload type must implement `Serialize` and `Clone`.
#[derive(Clone, serde::Serialize)]
pub struct Payload {
    pub data: String,
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
    TtsStoped,
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
            Self::TtsStoped => "tts-stoped",
        }
    }
}

pub fn play(phrase: &str, app_handle: &tauri::AppHandle) {
    // Воспроизводим аудио
    app_handle
        .emit_all(
            EventTypes::AudioPlay.get(),
            Payload {
                data: phrase.into(),
            },
        )
        .unwrap();
}

pub fn tts_started(app_handle: &tauri::AppHandle) {
    println!("Вызов emit tts started...");

    events::play("load", TAURI_APP_HANDLE.get().unwrap());

    app_handle
        .emit_all(EventTypes::TtsStarted.get(), Payload { data: "".into() })
        .unwrap();
}

pub fn tts_stopped(app_handle: &tauri::AppHandle) {
    println!("Вызов emit tts stoped...");

    app_handle
        .emit_all(EventTypes::TtsStoped.get(), Payload { data: "".into() })
        .unwrap();
}
