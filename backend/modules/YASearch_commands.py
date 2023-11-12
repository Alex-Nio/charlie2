import webbrowser
import config
import requests
from modules import methods
from modules import tts

#! Поиск Яндекс
def execute_search_cmd(cmd: str, voice: str, new_data, counter):
    try:
        if cmd == "search_cmd":
            # ? YA Search
            # ? Подставляем запрос в урл
            data = new_data
            replace_aliases = config.VA_YASearch["search_cmd"]
            alias_names = config.VA_ALIAS
            search_str = methods.alias_replacer(alias_names, replace_aliases, data)
            search_str = " ".join(search_str)
            url = "http://yandex.ru/yandsearch?text="

            print(url + str(search_str) + " " + " --> Сформированный URL")
            print("Поисковый запрос: " + str(search_str))

            requests.get(url + search_str)
            webbrowser.open(url + search_str)

            tts.va_speak("нашла")
    # ? Обработка ошибки если не выполнен запуск программы по ключевым словам
    except NameError:
        tts.va_speak("Произошла ошибка во время выполнения команды")
