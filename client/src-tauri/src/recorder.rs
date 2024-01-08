// use once_cell::sync::OnceCell;
use atomic_enum::atomic_enum;
use log::{info, warn};
use std::sync::atomic::{AtomicU32, Ordering};

mod pvrecorder;

use crate::DB;

#[atomic_enum]
#[derive(PartialEq)]
pub enum RecorderType {
    Cpal,
    PvRecorder,
    PortAudio,
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
        }
        RecorderType::PortAudio => {
            // Init PortAudio
            info!("Initializing PortAudio audio backend");
            todo!();
        }
        RecorderType::Cpal => {
            // Init CPAL
            info!("Initializing CPAL audio backend");
            todo!();
        }
    }
}

pub fn read_microphone(frame_buffer: &mut [i16]) {
    match RECORDER_TYPE.load(Ordering::SeqCst) {
        RecorderType::PvRecorder => {
            pvrecorder::read_microphone(frame_buffer);
        }
        RecorderType::PortAudio => {
            todo!();
        }
        RecorderType::Cpal => {
            panic!("Cpal should be used via callback assignment");
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
        }
        RecorderType::PortAudio => {
            todo!();
        }
        RecorderType::Cpal => {}
    }
}

pub fn stop_recording() {
    match RECORDER_TYPE.load(Ordering::SeqCst) {
        RecorderType::PvRecorder => {
            pvrecorder::stop_recording();
        }
        RecorderType::PortAudio => {
            todo!();
        }
        RecorderType::Cpal => {}
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
