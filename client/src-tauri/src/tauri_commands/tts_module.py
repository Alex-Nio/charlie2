# tts_module.py
import os
import time
import torch
from NeuralSpeaker import NeuralSpeaker

neural_speaker = NeuralSpeaker()

def tts_speak(words, speaker='xenia', save_file=False, sample_rate=48000):
    return neural_speaker.speak(words=words, speaker=speaker, save_file=save_file, sample_rate=sample_rate)
