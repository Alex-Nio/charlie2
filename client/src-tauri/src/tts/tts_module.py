# tts_module.py
import os
import time
import sys
from NeuralSpeaker import NeuralSpeaker

neural_speaker = NeuralSpeaker()

def tts_speak(words, speaker='eugene', save_file=False, sample_rate=38000):
    print('Received words:', words)
    try:
        result = neural_speaker.speak(words=words, speaker=speaker, save_file=save_file, sample_rate=sample_rate)
        print('Processing output text...')
        return result
    except Exception as e:
        print(f'Error processing text: {e}')
        raise

if __name__ == "__main__":
    # Получение текста из командной строки
    text_from_command_line = sys.argv[1]
    try:
        tts_speak(text_from_command_line)
    except Exception as e:
        print(f'Error speaking text: {e}')
        sys.exit(1)
