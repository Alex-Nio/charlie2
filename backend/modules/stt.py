import vosk
import sys
import sounddevice as sd
import queue
import json
import os

# Путь к текущему скрипту
script_path = os.path.dirname(os.path.abspath(__file__))

# Абсолютный путь к папке model_small
model_path = os.path.join(script_path, "model_small")

model = vosk.Model(model_path)
samplerate = 16000
device = 1

q = queue.Queue()


def q_callback(indata, frames, time, status):
    if status:
        print(status, file=sys.stderr)
    q.put(bytes(indata))


def va_listen(callback):
    with sd.RawInputStream(samplerate=samplerate, blocksize=8000, device=device, dtype='int16', channels=1, callback=q_callback):

        rec = vosk.KaldiRecognizer(model, samplerate)
        while True:
            data = q.get()
            if rec.AcceptWaveform(data):
                recognized_speech = json.loads(rec.Result())["text"]

                # Log the recognized speech
                print(f"Currently listened: {recognized_speech}")

                callback(recognized_speech)
            # else:
            #    print(rec.PartialResult())
