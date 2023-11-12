from modules import tts
from modules import wiki_parcer


def execute_wiki_cmd(cmd: str, voice: str, new_data, counter):
    try:
        if cmd == "wiki_cmd":
            wiki_parcer.wiki_parcer(new_data)
    # ? Обработка ошибки если не выполнен запуск программы по ключевым словам
    except NameError:
        tts.va_speak("Произошла ошибка во время выполнения команды")
