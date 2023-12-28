// APP
pub enum WakeWordEngine {
    Vosk,
}

pub const DEFAULT_WAKE_WORD_ENGINE: WakeWordEngine = WakeWordEngine::Vosk;

pub const DB_FILE_NAME: &str = "app.db";
pub const LOG_FILE_NAME: &str = "log.txt";
pub const APP_VERSION: Option<&str> = option_env!("CARGO_PKG_VERSION");
pub const AUTHOR_NAME: Option<&str> = option_env!("CARGO_PKG_AUTHORS");
pub const REPOSITORY_LINK: Option<&str> = option_env!("CARGO_PKG_REPOSITORY");

pub const COMMANDS_PATH: &str = "commands/";

// VOSK
// pub const VOSK_MODEL_PATH: &str = const_concat!(PUBLIC_PATH, "/vosk/model_small");
pub const VOSK_FETCH_PHRASE: &str = "чарли";
pub const VOSK_MODEL_PATH: &str = "ai/vosk/model_small";
pub const VOSK_MIN_RATIO: f64 = 70.0;

// ETC
pub const CMD_RATIO_THRESHOLD: f64 = 65f64;
pub const CMS_WAIT_DELAY: std::time::Duration = std::time::Duration::from_secs(15);

pub const ASSISTANT_GREET_PHRASES: [&str; 2] = ["greet1", "greet2"];
pub const ASSISTANT_WAIT_PHRASES: [&str; 8] = [
    "wait1", "wait2", "wait3", "wait4", "wait5", "wait6", "wait7", "wait8",
];
pub const ASSISTANT_PHRASES_TBR: [&str; 18] = [
    "чарли",
    "сэр",
    "слушаю сэр",
    "всегда к услугам",
    "произнеси",
    "ответь",
    "покажи",
    "скажи",
    "давай",
    "да сэр",
    "к вашим услугам сэр",
    "всегда к вашим услугам сэр",
    "запрос выполнен сэр",
    "выполнен сэр",
    "есть",
    "загружаю сэр",
    "очень тонкое замечание сэр",
    "мы подключены и готовы",
];
