// APP
pub enum WakeWordEngine {
    Rustpotter,
    Vosk,
    Porcupine,
}

pub const DEFAULT_WAKE_WORD_ENGINE: WakeWordEngine = WakeWordEngine::Vosk;

pub const DB_FILE_NAME: &str = "app.db";
pub const LOG_FILE_NAME: &str = "log.txt";
pub const APP_VERSION: Option<&str> = option_env!("CARGO_PKG_VERSION");
pub const AUTHOR_NAME: Option<&str> = option_env!("CARGO_PKG_AUTHORS");
pub const REPOSITORY_LINK: Option<&str> = option_env!("CARGO_PKG_REPOSITORY");

// RUSPOTTER
pub const RUSPOTTER_MIN_SCORE: f32 = 0.62;

// PICOVOICE
pub const COMMANDS_PATH: &str = "commands/";
pub const KEYWORDS_PATH: &str = "picovoice/keywords/";

// VOSK
// pub const VOSK_MODEL_PATH: &str = const_concat!(PUBLIC_PATH, "/vosk/model_small");
pub const VOSK_FETCH_PHRASE: &str = "чарли";
pub const VOSK_MODEL_PATH: &str = "ai/vosk/model_small";
pub const VOSK_MIN_RATIO: f64 = 70.0;

// ETC
pub const CMD_RATIO_THRESHOLD: f64 = 65f64;
pub const CMS_WAIT_DELAY: std::time::Duration = std::time::Duration::from_secs(15);

pub const ASSISTANT_GREET_PHRASES: [&str; 3] = ["greet1", "greet2", "greet3"];
pub const ASSISTANT_PHRASES_TBR: [&str; 17] = [
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
];
