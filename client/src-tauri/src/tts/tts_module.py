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
logger.info("Audio Devices: %s", audio_devices)

# Выберите идентификатор вашего микрофона
microphone_device_id = 0

# Функция для отключения микрофона
def disable_microphone():
    sd.stop()

# Функция для включения микрофона
def enable_microphone(sample_rate, channels):
    duration = 1.0  # Вы можете настроить продолжительность в зависимости от ваших потребностей
    myrecording = sd.rec(int(sample_rate * duration), channels=channels, dtype=np.int16, device=microphone_device_id)
    sd.wait()

async def tts_speak_async(words, speaker='eugene', save_file=False, sample_rate=48000):
    logger.info('Received words: %s', words)

    # Отключение микрофона перед воспроизведением речи
    disable_microphone()

    try:
        result = await neural_speaker.speak(words=words, speaker=speaker, save_file=save_file, sample_rate=sample_rate)
        logger.info('Обработка выходного текста...')
        return result
    except Exception as e:
        logger.error('Error processing text: %s', e)
        raise
    finally:
        # Включение микрофона после воспроизведения речи
        enable_microphone(sample_rate, channels=2)  # Передача параметров sample_rate и channels

if __name__ == "__main__":
    # Получение текста из командной строки
    text_from_command_line = sys.argv[1]
    try:
        import asyncio
        asyncio.run(tts_speak_async(text_from_command_line))
    except Exception as e:
        logger.error('Error speaking text: %s', e)
        sys.exit(1)
