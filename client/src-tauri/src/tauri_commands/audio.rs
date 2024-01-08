use pv_recorder::RecorderBuilder;
use crate::DB;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct AudioDeviceInfo {
    index: usize,
    name: String,
}

#[tauri::command]
pub fn pv_get_audio_devices() -> Vec<AudioDeviceInfo> {
    let audio_devices = RecorderBuilder::default().get_audio_devices();
    match audio_devices {
        Ok(audio_devices) => {
            let devices_info: Vec<AudioDeviceInfo> = audio_devices
                .iter()
                .enumerate()
                .map(|(index, name)| AudioDeviceInfo {
                    index,
                    name: name.clone(),
                })
                .collect();
            devices_info
        }
        Err(err) => {
            eprintln!("Failed to get audio devices: {}", err);
            panic!("Failed to get audio devices: {}", err)
        }
    }
}

#[tauri::command]
pub fn pv_get_audio_device_name(idx: i32) -> String {
    let audio_devices = RecorderBuilder::default().get_audio_devices();
    let mut first_device: String = String::new();
    match audio_devices {
        Ok(audio_devices) => {
            for (_idx, device) in audio_devices.iter().enumerate() {
                if idx as usize == _idx {
                    return device.to_string();
                }

                if _idx == 0 {
                    first_device = device.to_string()
                }
            }
        }
        Err(err) => panic!("Failed to get audio devices: {}", err),
    };

    // return first device as default, if none were matched
    first_device
}

use crate::recorder::get_selected_microphone_index;

#[tauri::command]
pub fn update_selected_microphone(index: usize) -> Result<(), String> {
    // Ваш код для обновления выбранного микрофона
    // Используйте полученный индекс для обновления значения в вашем хранилище
    let _ = DB.lock().unwrap().set("selected_microphone", &index.to_string());
    get_selected_microphone_index();

    Ok(())
}
