# Разработка:
#! _________________________________________________________________________
#! Запись в список

# ? Дано: Команда "Рико, добавь в список дел на (день\вечер) (то что нужно добавить)"
# * Если команда create_workflow, то:
# 1 Часть функции (Определяем Список на ДЕНЬ или на ВЕЧЕР)
# * 1. Берём строку разбиваем её по словам, находим индекс слова "на"
# * 2. После "на" Если День, то флаг AM = True
# * 3. После "на" Если Вечер, то флаг PM = True
# * 4. Индекс последующего дейсвия будет равен индексу "на" + 1
# 2 Часть функции (Записываем полученную строку с действием в переменную)
# * 1. От этого индекса вырезаем остальное до конца списка
# * 2. Преобразуем список в строку
# * 3. записываем результат в переменную workflow
# 3 Часть (Работа с Exel):
# * 1. Берём столбец если ДЕНЬ, перебираем все ячейки
# * 2. Берём столбец если ВЕЧЕР, перебираем все ячейки
# * 3. Определяем первую попавшуюся пустую ячейку
# * 4. Записываем значение туда и сохраняем файл

#! _________________________________________________________________________
#! Озвучивание списка

# ? Дано: Команда "Рико, озвучь список дел на (день\вечер)"
# * Если команда check_workflow, то:

# * 1. Если День, то флаг AM = True
# То столбец 1
# * 2. Если Вечер, то флаг PM = True
# То столбец 2
# * 3. Перебираем все ячейки, и озвучиваем значение ячейки если оно не пустое.

#! _________________________________________________________________________
#! Код

import openpyxl  # Подключаем библиотеку Openpyxl
from modules import tts


def add_wf(data_list):
    # variables
    data = data_list
    trigger_item = "на"

    # Записываем значение в ячейку
    def change_cell_value(col, str):
        for cell in col:
            # Начиная с 3 ячейки и если ячейка пустая
            if col.index(cell) > 1 and cell.value == None:
                # print(cell.coordinate)  # название ячейки
                cell.value = str
                break

    def create_wf(string, marker):
        # Открываем тестовый Excel файл
        work_book = openpyxl.load_workbook(".\\documents\\todo-list.xlsx")
        worksheet = work_book["Лист1"]  # Делаем его активным

        if marker == "день":
            column = worksheet["B"]  # Указываем нужный столбец "B"
            change_cell_value(column, string)
            work_book.save(".\\documents\\todo-list.xlsx")
        elif marker == "вечер":
            column = worksheet["E"]  # Указываем нужный столбец "E"
            change_cell_value(column, string)
            work_book.save(".\\documents\\todo-list.xlsx")

    # Если есть 'на', то
    # Определяем старт
    # Определяем время суток
    if trigger_item in data:
        start = data.index(trigger_item)  # 5
        daytime_marker = data[start + 1]  # Время суток
        end = data.index(daytime_marker) + 1
        phrase = " ".join(data[end:])
        # print(daytime_marker)  # вечер
        if phrase != "":
            create_wf(phrase, daytime_marker)
            tts.va_speak("Добавила!")
        else:
            print("Ошибка. Нечего добавлять в список дел")
            tts.va_speak("Не поняла. Что нужно добавить?")

    # Если нет, то ищем индекс 'дел' и рандомно выбираем день/вечер
    else:
        from random import randint

        trigger_item = "дел"
        start = data.index(trigger_item)  # 4
        daytime_marker = randint(0, 1)
        end = start + 1
        phrase = " ".join(data[end:])

        # Выбираем День/Вечер самостоятельно
        if daytime_marker == 1:
            daytime_marker = "день"
        elif daytime_marker == 0:
            daytime_marker = "вечер"

        if phrase != "":
            create_wf(phrase, daytime_marker)
            tts.va_speak("Добавила!")
        else:
            print("Ошибка. Нечего добавлять в список дел")
            tts.va_speak("Не поняла. Что нужно добавить?")


#! _________________________________________________________________________
#! Код


def check_wf(data_list):
    # variables
    data = data_list
    trigger_item = "на"

    # Записываем значение в ячейку
    def tell_cell_value(col, str):
        for cell in col:
            # Начиная с 3 ячейки и если ячейка пустая
            if col.index(cell) > 1 and cell.value != None:
                tts.va_speak(cell.value)  # содержание ячейки

    def create_wf(string, marker):
        # Открываем тестовый Excel файл
        work_book = openpyxl.load_workbook(".\\documents\\todo-list.xlsx")
        worksheet = work_book["Лист1"]  # Делаем его активным

        if marker == "день":
            column = worksheet["B"]  # Указываем нужный столбец "B"
            tell_cell_value(column, string)
            work_book.save(".\\documents\\todo-list.xlsx")
        elif marker == "вечер":
            column = worksheet["E"]  # Указываем нужный столбец "E"
            tell_cell_value(column, string)
            work_book.save(".\\documents\\todo-list.xlsx")

    # Если есть 'на', то
    # Определяем старт
    # Определяем время суток
    if trigger_item in data:
        start = data.index(trigger_item)  # 5
        daytime_marker = data[start + 1]  # Время суток
        end = data.index(daytime_marker) + 1
        phrase = " ".join(data[end:])
        # print(daytime_marker)  # вечер
        create_wf(phrase, daytime_marker)

    # Если нет, то ищем индекс 'дел' и рандомно выбираем день/вечер
    else:
        from random import randint

        trigger_item = "дел"
        start = data.index(trigger_item)  # 4
        daytime_marker = randint(0, 1)
        end = start + 1
        phrase = " ".join(data[end:])

        # Выбираем День/Вечер самостоятельно
        if daytime_marker == 1:
            daytime_marker = "день"
        elif daytime_marker == 0:
            daytime_marker = "вечер"

        create_wf(phrase, daytime_marker)
