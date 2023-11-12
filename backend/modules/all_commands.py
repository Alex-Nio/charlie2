import subprocess
import datetime
import time
import webbrowser
import os
import sys
from threading import Thread
import pyautogui
import requests
from bs4 import BeautifulSoup
from num2t4ru import num2text
from modules import keyboard_iterator as KB
from modules import tts  # pylint: disable=ungrouped-imports
import config
from modules import num_checker  # pylint: disable=ungrouped-imports
from modules import time_declination
from modules import methods
from modules import jokes
from modules.sound import Sound
import settings
from deep_translator import GoogleTranslator
from modules.workflow_functions import check_wf


# ? Менеджер команд


def execute_cmd(cmd: str, voice: str, new_data, counter):
    try:
        #! Статус для Рико
        # ? Основные функции/Помощь
        if cmd == "help":
            text = "Я умею: ..."
            text += "произносить время ..."
            text += "управлять расположением ок+он ..."
            text += "открывать браузер ..."
            text += "закрывать вкладки в нужном количестве ..."
            text += "закрывать открытые о+кна ..."
            text += "открывать редактор кода ..."
            text += "переключать звук и сообщать погоду ..."
            text += "искать информацию в интернете ..."
            text += "открывать Телеграм ..."
            text += "запускать программы ..."
            text += "говорить что по распорядку дня ..."
            text += "для старта назови меня по  +имени и скажи команду ..."
            text += "для выхода скажи  Выход ..."
            text += "Пока это всё, что я умею, но мне нужно учиться"
            tts.va_speak(text)
        #! Озвучивание списка дел на День\Вечер
        elif cmd == "check_workflow":
            data_list = new_data
            check_wf(data_list)
            print("Команда check_workflow выполнена успешно!")
        elif cmd == "open_workflow":
            subprocess.Popen(settings.todo_list_path, shell=True)
            tts.va_speak("открыла")
        #! ОС Команды
        # ? Закрыть окно
        elif cmd == "escape_cmd":
            KB.keyboard_press_key("alt+f4")
            tts.va_speak("закрыла")
        elif cmd == "save_cmd":
            KB.keyboard_press_key("ctrl+s")
            tts.va_speak("сохранено")
        elif cmd == "save_and_exit_cmd":
            KB.keyboard_press_key("ctrl+s")
            KB.keyboard_press_key("alt+f4")
            tts.va_speak("готово!")
        # ? Время
        elif cmd == "time_cmd":
            now = datetime.datetime.now()
            text = (
                "Сей+час"
                + " "
                + num2text(now.hour)
                + " "
                + str(time_declination.show_hours(now.hour))
                + " "
                + num2text(now.minute)
                + " "
                + str(time_declination.show_minutes(now.minute))
            )
            tts.va_speak(text)
        # ? Окно налево
        elif cmd == "window_to_left":
            # KB.keyboard_press_key("ctrl+win+x")
            tts.va_speak("готово")
        # ? Окно направо
        elif cmd == "window_to_right":
            # KB.keyboard_press_key("ctrl+win+x")
            tts.va_speak("готово")
        # ? Фулскрин
        elif cmd == "window_full_screenOnn":
            KB.keyboard_press_key("win+up")
            tts.va_speak("готово")
        # ? Свернуть окно
        elif cmd == "window_full_screenOff":
            KB.keyboard_press_key("win+down")
            KB.keyboard_press_key("win+down")
            tts.va_speak("готово")
        #! Браузер
        # ? Открыть браузер
        elif cmd == "open_browser":
            webbrowser.open(settings.browser_start_page)
            tts.va_speak("открыла")
        # ? Открыть вк
        elif cmd == "open_vk":
            webbrowser.open(settings.vk_start_page)
            tts.va_speak("открыла")
        # ? Открыть Ютуб
        elif cmd == "open_youtube":
            webbrowser.open(settings.youtube_start_page)
            tts.va_speak("открыла")
        # ? Обновить страницу
        elif cmd == "page_upd_cmd":
            KB.keyboard_press_key("ctrl+f5")
            tts.va_speak("обновлено")
        # ? Закрыть вкладку
        elif cmd == "close_current_page_cmd":
            #! Выполняем количество голосовых задач
            KB.keyboard_press_val(counter, KB.keyboard_press_key)
            tts.va_speak("закрыла")
        # ? Новая вкладка
        elif cmd == "create_new_page_cmd":
            KB.keyboard_press_key("ctrl+t")
            tts.va_speak("готово")
        #! Программы
        # ? Запуск программ
        elif cmd == "work_cmd":
            subprocess.Popen(settings.zoom_path)
            subprocess.Popen(settings.telegram_path)
            # subprocess.Popen(settings.horizon_path)
            tts.va_speak("запускаю программы ... Приятной работы")
        elif cmd == "schedule_cmd":
            subprocess.Popen(settings.schedule_path, shell=True)
            tts.va_speak("открыла")
        elif cmd == "calculator_cmd":
            subprocess.Popen(settings.calculator_path, shell=True)
            tts.va_speak("открыла")
        # ? Запуск редактора кода
        elif cmd == "vs_open":
            subprocess.Popen(settings.vs_code_path)
            tts.va_speak("редактор запущен")
        # ? Телеграм
        elif cmd == "telegram_cmd":
            subprocess.Popen(settings.telegram_path)
            tts.va_speak("открыла")
        # ? Jokes
        elif cmd == "joke_cmd":
            try:
                joke = jokes.get_joke()
                tts.va_speak(joke)
            except Exception:
                joke = jokes.get_joke()
                tts.va_speak(joke)
        #! Плеер
        # ? Музыка
        elif cmd == "play_music_cmd":
            music_dir = settings.music_dir
            songs = os.listdir(music_dir)
            print(str(len(songs)) + "---треков")
            count = 0
            for i in songs:
                count += 1
                print(f"{count}.{i}")

            os.startfile(os.path.join(music_dir, songs[0]))
            tts.va_speak("Музыка запущена")
        # ? Следующий трек >>
        elif cmd == "next_track_cmd":
            tts.va_speak("переключаю")
            pyautogui.press("nexttrack")
        # ? Предыдущий трек <<
        elif cmd == "last_track_cmd":
            tts.va_speak("переключаю")
            pyautogui.press("prevtrack")
        # ? Пауза плеера ||
        elif cmd == "mute_player_cmd":
            pyautogui.press("playpause")
            tts.va_speak("пауза выполнена")
        # ? Запуск плеера ||
        elif cmd == "player_play_cmd":
            tts.va_speak("запускаю")
            pyautogui.press("playpause")
        # ? Установить звук в %
        elif cmd == "volume_set_cmd":
            Sound.volume_set(counter)
        #! Динамики / Наушники
        elif cmd == "speakers_cmd":
            KB.keyboard_press_key("alt+c")
            tts.va_speak("динамики включены")
        elif cmd == "headphones_cmd":
            KB.keyboard_press_key("alt+v")
            tts.va_speak("наушники включены")
        #! Погода
        elif cmd == "weather_cmd":
            url = "https://pogoda1.ru/beloozersky/"  # url
            response = requests.get(url)
            soup = BeautifulSoup(response.text, "lxml")
            data = soup.find("div", class_="weather-now-temp")
            weather_now_value = []  # type: list[str]

            # ? Берём данные о погоде
            def initiate_take_weather_data(data, weather_now_value):
                weather_now_value.append(data.text)
                return weather_now_value

            # ? Превращаем нужные данные в число
            def convert_weather_data(data):
                num = ""
                for i in data:
                    if i.isdigit():
                        num = num + i
                return int(num)

            weather_data = initiate_take_weather_data(data, weather_now_value)

            if weather_data[0][0] == "+":
                temp = "Плюс"
            elif weather_data[0][0] == "-":
                temp = "Минус"

            weather_data = str(weather_data)

            # print(weather_data + " Текущая погода") # ['+21°']
            current_weather = convert_weather_data(weather_data)

            # Результат склонения
            declension = methods.numeral_noun_declension(
                current_weather, "градус", "градуса", "градусов"
            )

            # Число в текст
            current_weather = num2text(current_weather)

            # Результат
            result = f"В Белоозёрском сейчас: {temp} {current_weather} {declension}."

            # Произносим погоду
            tts.va_speak(result)
        #! Расписание
        elif cmd == "time_management_cmd":
            tts.va_speak("Открываю расписание... ...")
            occ_check(time_list_data, time, current_occ)
        # ? Закрыть программу RICO
        if cmd == "exit_cmd":
            tts.va_speak("закрываюсь")
            os._exit(1)
    # ? Обработка ошибки если не выполнен запуск программы по ключевым словам
    except NameError:
        tts.va_speak("Произошла ошибка во время выполнения команды")
