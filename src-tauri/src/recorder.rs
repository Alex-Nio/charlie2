// use once_cell::sync::OnceCell;
use atomic_enum::atomic_enum;
use log::warn;
use std::sync::atomic::{AtomicU32, Ordering};

mod pvrecorder;

use crate::DB;

#[atomic_enum]
#[derive(PartialEq)]
pub enum RecorderType {
    PvRecorder,
}

pub static RECORDER_TYPE: AtomicRecorderType = AtomicRecorderType::new(RecorderType::PvRecorder); // use pvrecorder as default
pub static FRAME_LENGTH: AtomicU32 = AtomicU32::new(0);

pub fn init() {
    match RECORDER_TYPE.load(Ordering::SeqCst) {
        RecorderType::PvRecorder => {
            match pvrecorder::init_microphone(
                get_selected_microphone_index(),
                FRAME_LENGTH.load(Ordering::SeqCst),
            ) {
                false => {
                    // Switch to CPAL recorder
                    warn!("Pv Recorder audio backend failed.");
                    init();
                }
                _ => (),
            }

            println!("[+] Init done for {}", get_selected_microphone_index());
        }
    }
}

pub fn read_microphone(frame_buffer: &mut [i16]) {
    match RECORDER_TYPE.load(Ordering::SeqCst) {
        RecorderType::PvRecorder => {
            pvrecorder::read_microphone(frame_buffer);
        }
    }
}

pub fn start_recording() {
    match RECORDER_TYPE.load(Ordering::SeqCst) {
        RecorderType::PvRecorder => {
            pvrecorder::start_recording(
                get_selected_microphone_index(),
                FRAME_LENGTH.load(Ordering::SeqCst),
            );
            println!("Recording started successfully");
        }
    }
}

pub fn stop_recording() {
    match RECORDER_TYPE.load(Ordering::SeqCst) {
        RecorderType::PvRecorder => {
            pvrecorder::stop_recording();
            println!("Recording stopped");
        }
    }
}

pub fn get_selected_microphone_index() -> i32 {
    let selected_microphone: i32;

    // Retrieve microphone index
    if let Some(smic) = DB.lock().unwrap().get::<String>("selected_microphone") {
        selected_microphone = smic.parse().unwrap_or(-1);
    } else {
        selected_microphone = -1;
    }

    println!("[+] selected_microphone: {}", selected_microphone);

    selected_microphone
}

#[tauri::command]
pub fn update_selected_microphone(index: usize) -> Result<(), String> {
    let _ = DB.lock().unwrap().set("selected_microphone", &index.to_string());
    get_selected_microphone_index();

    // Обновить используемый микрофон
    RECORDER_TYPE.store(RecorderType::PvRecorder, Ordering::SeqCst);

    // Перезапустить запись с новым микрофоном
    restart_audio_capture(Some(index));

    Ok(())
}

// Пример функции restart_audio_capture (адаптируйте под ваш код)
fn restart_audio_capture(new_microphone_index: Option<usize>) {
    // Остановить текущую запись, если она активна
    stop_recording();

    // Обновить настройки записи с новым микрофоном
    init();

    // Начать новую запись
    start_recording();
}
