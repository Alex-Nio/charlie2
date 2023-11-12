import keyboard
from colorama import Fore, Back, Style

# ? Вычисляем нужное количество нажатий клавиатуры
def keyboard_press_val(i, fun):
    try:
        print(Fore.BLUE + "Количество нажатий: " + Fore.CYAN + str(i) + Style.RESET_ALL)
        i = int(i)
        [fun("ctrl+w") for x in range(i)]
    except ValueError:
        fun("ctrl+w")


# ? Нажимаем нужную клавишу
def keyboard_press_key(key):
    print(Fore.BLUE + "Нажатые клавиши: " + Fore.CYAN + key + Style.RESET_ALL)
    keyboard.press_and_release(key)
