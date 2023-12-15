use log::{error, info, warn};
use rand::seq::SliceRandom;
use seqdiff::ratio;
use serde_yaml;
use std::path::Path;
use std::{fs, fs::File};

use core::time::Duration;
use std::path::PathBuf;
use std::process::{Child, Command};
// use tauri::Manager;

mod structs;
pub use structs::*;

use crate::config;
use crate::events;

pub fn parse_commands() -> Result<Vec<AssistantCommand>, String> {
    // collect commands
    let mut commands: Vec<AssistantCommand> = vec![];

    // read commands directories first
    if let Ok(cpaths) = fs::read_dir(config::COMMANDS_PATH) {
        for cpath in cpaths {
            // validate this command, check if required files exists
            let _cpath = cpath.unwrap().path();
            let cc_file = Path::new(&_cpath).join("command.yaml");

            if cc_file.exists() {
                // try parse config files
                let cc_reader = std::fs::File::open(&cc_file).unwrap();
                let cc_yaml: CommandsList;

                // try parse command.yaml
                match serde_yaml::from_reader::<File, CommandsList>(cc_reader) {
                    Ok(parse_result) => {
                        cc_yaml = parse_result;
                    }
                    Err(msg) => {
                        warn!(
                            "Can't parse {}, skipping ...\nCommand parse error is: {:?}",
                            &cc_file.display(),
                            msg
                        );
                        eprintln!(
                            "Can't parse {}, skipping ...\nCommand parse error is: {:?}",
                            &cc_file.display(),
                            msg
                        );
                        continue;
                    }
                }
                // everything seems to be Ok
                commands.push(AssistantCommand {
                    path: _cpath,
                    commands: cc_yaml,
                });
            }
        }

        if commands.len() > 0 {
            Ok(commands)
        } else {
            error!("No commands were found");
            Err("No commands were found".into())
        }
    } else {
        error!("Error reading commands directory");
        return Err("Error reading commands directory".into());
    }
}

pub fn fetch_command<'a>(
    phrase: &str,
    commands: &'a Vec<AssistantCommand>,
) -> Option<(&'a PathBuf, &'a Config)> {
    // Лучший найденный результат
    let mut best_match: Option<(&PathBuf, &Config)> = None;
    // Текущий максимальный коэффициент соответствия
    let mut current_max_ratio = config::CMD_RATIO_THRESHOLD;

    // Преобразование фразы для поиска в последовательность символов
    let fetch_phrase_chars = phrase.chars().collect::<Vec<_>>();

    // Перебор всех команд
    for cmd in commands {
        for scmd in &cmd.commands.list {
            for cmd_phrase in &scmd.phrases {
                let cmd_phrase_chars = cmd_phrase.chars().collect::<Vec<_>>();
                let ratio = ratio(&fetch_phrase_chars, &cmd_phrase_chars);

                // Обновление результатов при нахождении лучшего соответствия
                if ratio >= current_max_ratio {
                    best_match = Some((&cmd.path, &scmd));
                    current_max_ratio = ratio;
                }
            }
        }
    }

    // Вывод результатов, если они найдены
    if let Some((cmd_path, scmd)) = best_match {
        println!("Соотношение: {}", current_max_ratio);
        info!(
            "CMD: {cmd_path:?}, SCMD: {scmd:?}, Соотношение: {}",
            current_max_ratio
        );
        Some((&cmd_path, &scmd))
    } else {
        None
    }
}

pub fn execute_exe(exe: &str, args: &Vec<String>) -> std::io::Result<Child> {
    Command::new(exe).args(args).spawn()
}

pub fn execute_cli(cmd: &str, args: &Vec<String>) -> std::io::Result<Child> {
    println!("Spawning cmd as: cmd /C {} {:?}", cmd, args);

    if cfg!(target_os = "windows") {
        Command::new("cmd").arg("/C").arg(cmd).args(args).spawn()
    } else {
        Command::new("sh").arg("-c").arg(cmd).args(args).spawn()
    }
}

pub fn execute_command(
    cmd_path: &PathBuf,
    cmd_config: &Config,
    app_handle: &tauri::AppHandle,
    _speech_result: &str,
) -> Result<bool, String> {
    match cmd_config.command.action.as_str() {
        "search" => {
            println!("Выполнена новая команда: {:?}", &cmd_config);
            let random_cmd_sound = cmd_config
                .voice
                .sounds
                .choose(&mut rand::thread_rng())
                .unwrap();
            events::play(random_cmd_sound, app_handle);

            Ok(true)
        }
        "voice" => {
            // VOICE command type
            let random_cmd_sound = cmd_config
                .voice
                .sounds
                .choose(&mut rand::thread_rng())
                .unwrap();
            events::play(random_cmd_sound, app_handle);

            Ok(true)
        }
        "ahk" => {
            // AutoHotkey command type
            let exe_path_absolute = Path::new(&cmd_config.command.exe_path);
            let exe_path_local = Path::new(&cmd_path).join(&cmd_config.command.exe_path);

            if let Ok(_) = execute_exe(
                if exe_path_absolute.exists() {
                    exe_path_absolute.to_str().unwrap()
                } else {
                    exe_path_local.to_str().unwrap()
                },
                &cmd_config.command.exe_args,
            ) {
                let random_cmd_sound = cmd_config
                    .voice
                    .sounds
                    .choose(&mut rand::thread_rng())
                    .unwrap();
                events::play(random_cmd_sound, app_handle);

                Ok(true)
            } else {
                error!("AHK process spawn error (does exe path is valid?)");
                Err("AHK process spawn error (does exe path is valid?)".into())
            }
        }
        "cli" => {
            // CLI command type
            let cli_cmd = &cmd_config.command.cli_cmd;

            match execute_cli(cli_cmd, &cmd_config.command.cli_args) {
                Ok(_) => {
                    let random_cmd_sound = cmd_config
                        .voice
                        .sounds
                        .choose(&mut rand::thread_rng())
                        .unwrap();
                    events::play(random_cmd_sound, app_handle);

                    Ok(true)
                }
                Err(msg) => {
                    error!("CLI command error ({})", msg);
                    Err(format!("Shell command error ({})", msg).into())
                }
            }
        }
        "terminate" => {
            // TERMINATE command type
            let random_cmd_sound = cmd_config
                .voice
                .sounds
                .choose(&mut rand::thread_rng())
                .unwrap();
            events::play(random_cmd_sound, app_handle);

            std::thread::sleep(Duration::from_secs(2));
            std::process::exit(0);
        }
        "stop_chaining" => {
            // STOP_CHAINING command type
            let random_cmd_sound = cmd_config
                .voice
                .sounds
                .choose(&mut rand::thread_rng())
                .unwrap();
            events::play(random_cmd_sound, app_handle);

            Ok(false)
        }
        _ => {
            error!("Command type unknown");
            Err("Command type unknown".into())
        }
    }
}
