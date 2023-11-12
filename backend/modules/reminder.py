from modules import tts  # pylint: disable=ungrouped-imports
from threading import Thread
import time as TIME
from num2t4ru import num2text
import config


# ? Напоминание
def execute_reminder_cmd(cmd: str, voice: str, new_data, counter):
    try:
        if cmd == "remind_cmd":
            remind_data_list = new_data
            remind_num = counter

            print(str(remind_data_list) + "---data")  # список
            print(str(remind_num) + "---remind number")  # число

            def reminder(data, remind_num):  # data = входящий голос / remind_num = число
                while True:
                    # Фильтруем из сообщения то что нужно напомнить т.е. "открыть\закрыть", "окно"
                    def filtration(data):
                        # Убираем название алиаса из стартовой строки
                        for x in data:
                            if x in config.VA_ALIAS:
                                data.remove(x)
                        # Убираем ключевое слово из стартовой строки
                        for z in data:
                            if z in config.VA_REMID['remind_cmd']:
                                data.remove(z)

                        # Убираем лишнее
                        if data[0] == 'и':
                            data.remove(data[0])

                        # Убираем лишнее
                        for c in data:
                            if c == 'мне':
                                data.remove(c)

                            print(str(data) + "---result data")

                            # ? Если есть слово через
                            try:
                                stop = data.index('через')
                                slice_object = slice(0, stop)
                                result = data[slice_object]
                                result = " ".join(result)
                                return result
                                # TODO: Если есть слово В
                            except ValueError:
                                result = " ".join(data)
                                tts.va_speak(
                                    f'Уточните через сколько вы хотите {result} повторив условие')
                                break

                    # Отфильтровываем то, что нужно напомнить
                    # Считаем время (часы, минуты или секунды)
                    condition = filtration(data)

                    def calc_total_time(i, flag):
                        if flag == 1:
                            x = (i * 60) * 60
                        elif flag == 2:
                            x = i * 60
                        elif flag == 3:
                            x = i
                        return x

                    # Определяем используемое время (часы, минуты или секунды)
                    def total_time_calculation(data, remind_num):
                        for item in data:
                            t_time = int
                            # Если часы
                            if item in ("час", "часа"):
                                r_time_hours = 1
                                t_time = calc_total_time(
                                    remind_num, r_time_hours)
                            # Если минуты
                            elif item in ("минут", "минуты"):
                                r_time_minutes = 2
                                t_time = calc_total_time(
                                    remind_num, r_time_minutes)
                            # Если секунды
                            elif item in ("секунд", "секунды"):
                                r_time_sec = 3
                                t_time = calc_total_time(
                                    remind_num, r_time_sec)
                        return t_time

                    # Тут мы подсчитали время
                    total_time = total_time_calculation(data, remind_num)

                    if condition is not None:
                        tts.va_speak('Хорошо, запомнила')
                        sleep_rec(total_time, condition)
                        break
                    else:
                        pass
                        break

            def sleep_rec(t_time, data):
                # Запуск таймера напоминания в потоке
                local_time = t_time
                TIME.sleep(local_time)
                t_time = num2text(t_time)
                tts.va_speak(
                    f'Напоминаю, нужно {data}, прошло {t_time} секунд')
            #! Тут производим передачу даты и запуск
            # # Создаём новый поток
            th = Thread(target=reminder, daemon=True,
                        args=(remind_data_list, remind_num,))
            th.start()
     # ? Обработка ошибки если не выполнен запуск программы по ключевым словам
    except NameError:
        tts.va_speak("Произошла ошибка во время выполнения команды")
