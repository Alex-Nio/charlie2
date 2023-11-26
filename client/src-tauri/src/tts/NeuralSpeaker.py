import os
import time
import asyncio
import aiohttp
import torch
import numpy as np
import simpleaudio as sa
from transliterate import translit
from num2words import num2words
import re
from pydub import AudioSegment

def play_audio(audio_data, sample_rate):
    audio = sa.WaveObject(audio_data.tobytes(), 1, 2, sample_rate)
    play_obj = audio.play()
    play_obj.wait_done()

async def play_audio_async(audio_data, sample_rate):
    loop = asyncio.get_event_loop()
    await loop.run_in_executor(None, play_audio, audio_data, sample_rate)

class NeuralSpeaker:
    def __init__(self):
        # print('Initializing neural model')
        start = time.time()
        device = torch.device('cpu')
        torch.set_num_threads(12)
        local_file = 'model.pt'
        if not os.path.isfile(local_file):
            torch.hub.download_url_to_file('https://models.silero.ai/models/tts/ru/v3_1_ru.pt', local_file)
        self.__model = torch.package.PackageImporter(local_file).load_pickle("tts_models", "model")
        self.__model.to(device)
        end = time.time()
        print(f'Model ready in {round(end - start, 2)} seconds')

    @staticmethod
    def __num2words_ru(match):
        clean_number = match.group().replace(',', '.')
        return num2words(clean_number, lang='ru')

    async def speak(self, words, speaker='xenia', save_file=False, sample_rate=48000):
        words = translit(words, 'ru')
        words = re.sub(r'-?[0-9][0-9,._]*', self.__num2words_ru, words)
        # print(f'text after translit and num2words {words}')
        if len(words) > 3:
            possible_speaker = words[0:2]
        else:
            return
        if possible_speaker == '!1':
            speaker = 'aidar'
        elif possible_speaker == '!2':
            speaker = 'baya'
        elif possible_speaker == '!3':
            speaker = 'ksenia'
        elif possible_speaker == '!4':
            speaker = 'xenia'
        elif possible_speaker == '!5':
            speaker = 'eugene'
        elif possible_speaker == '!0':
            speaker = 'random'

        example_text = f'{words}'
        if sample_rate not in [48000, 24000, 8000]:
            sample_rate = 48000
        if speaker not in ['aidar', 'baya', 'kseniya', 'xenia', 'eugene', 'random']:
            speaker = 'xenia'

        start = time.time()
        print(f'[+] TTS модель проинициализирована...')
        try:
            audio = self.__model.apply_tts(text=example_text,
                                           speaker=speaker,
                                           sample_rate=sample_rate, )
        except ValueError:
            print('Bad input')
            return
        end = time.time()
        time_elapsed = round(end - start, 2)
        print(f'Время инициализации заняло {time_elapsed} секунд')
        audio = audio.numpy()
        audio *= 22767 / np.max(np.abs(audio))
        audio = audio.astype(np.int16)

        if not save_file:
            await play_audio_async(audio, sample_rate)
            return time_elapsed
        else:
            return audio.tobytes()
