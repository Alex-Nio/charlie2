# 𝕮𝖍𝖆𝖗𝖑𝖎𝖊
# Made by Alex.

# =================================================================
# IMPORTS
# =================================================================

import os
import sys
import config
from modules import all_commands, tts
from modules import stt
from modules import recognize
from modules import timer
from queue import Queue

recognized_speech_queue = Queue()

if __name__ == "__main__":
    tts.va_speak("Привет!")
    # начать прослушивание команд
    stt.va_listen(recognize.va_respond)
