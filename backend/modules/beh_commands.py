# Поведение *************************************

from modules import tts
from random import randint


def execute_beh_cmd(cmd: str):
    try:
        # ? Благодарность
        if cmd == "thanks_cmd":
            tts.va_speak("Пожалуйста!")
        # ? Проверка
        elif cmd == "hello_cmd":
            tts.va_speak("Привет!")
        elif cmd == "status_check_cmd":
            tts.va_speak("Да да! я здесь!")
        elif cmd == "praise_cmd":
            tts.va_speak("Спасибо! Стараюсь!")
        elif cmd == "admiration":
            random_answer = randint(0, 3)

            answers = [
                "Всё хорошо, что хорошо кончается",
                "А как же!",
                "А разве может быть ина́че?",
                "Уррааа",
            ]

            tts.va_speak(answers[random_answer])
        elif cmd == "rude":
            random_answer = randint(0, 9)

            answers = [
                "Как вам не стыдно... Эээххх...",
                "Не выражайтесь в присутствии дамы",
                "Ну это как посмотреть...",
                "Пустяки - дело житейское",
                "ОЙ! ОЙ!",
                "Бывает...",
                "Это пройдёт...",
                "Всё не так плохо, как кажется!",
                "Бум Бам Бакуган!",
                "Не ругайся насяльника!",
            ]

            tts.va_speak(answers[random_answer])
    # ? Обработка ошибки если не выполнен запуск программы по ключевым словам
    except NameError:
        tts.va_speak("Произошла ошибка во время выполнения команды")
