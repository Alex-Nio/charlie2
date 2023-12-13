import os
import time
import sys
import logging
from NeuralSpeaker import NeuralSpeaker

# Добавлены библиотеки для работы с микрофоном
import sounddevice as sd
import numpy as np

# Настройка логгера
logging.basicConfig(level=logging.INFO)
logger = logging.getLogger(__name__)

neural_speaker = NeuralSpeaker()

# Получение информации об аудиоустройствах
audio_devices = sd.query_devices()

# Используйте следующую строку, чтобы увидеть доступные аудиоустройства и их параметры
# logger.info("Audio Devices: %s", audio_devices)

# Выберите идентификатор вашего микрофона
microphone_device_id = 0

# Флаг для прерывания TTS
stop_requested = False

# Функция для отключения микрофона
def disable_microphone():
    sd.stop()

# Функция для включения микрофона
def enable_microphone(sample_rate, channels):
    duration = 1.2  # Вы можете настроить продолжительность в зависимости от ваших потребностей
    myrecording = sd.rec(int(sample_rate * duration), channels=channels, dtype=np.int16, device=microphone_device_id)
    sd.wait()

async def tts_speak_async(words, speaker='eugene', save_file=False, sample_rate=48000):
    global stop_requested  # Используем глобальный флаг

    logger.info('Запрос к TTS: %s', words)

    # Убираем блок с кодом из текста
    words = remove_code_block(words)

    # Проверяем флаг остановки
    if stop_requested:
        # logger.info('Stop requested. Skipping TTS.')
        return

    try:
        result = await neural_speaker.speak(words=words, speaker=speaker, save_file=save_file, sample_rate=sample_rate)
        return result
    except Exception as e:
        # logger.error('Error processing text: %s', e)
        raise
    finally:
        pass

def remove_code_block(text):
    # Используем регулярное выражение для удаления блока кода между ``` и ```
    import re
    pattern = re.compile(r"```(?:[^`]+)```", re.MULTILINE | re.DOTALL)
    return pattern.sub('', text)

# Функция для запроса прерывания TTS
def request_stop():
    global stop_requested
    stop_requested = True

if __name__ == "__main__":
    # Получение текста из командной строки
    text_from_command_line = sys.argv[1]
    try:
        import asyncio
        asyncio.run(tts_speak_async(text_from_command_line))
    except Exception as e:
        # logger.error('Error speaking text: %s', e)
        sys.exit(1)
