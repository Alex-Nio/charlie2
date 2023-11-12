import win32api
import win32gui


# * cmd_content = текст голосовой команды
# * alias_content = имена голосового помощника
# * return data = Всё что идёт после голосовой команды
def alias_replacer(alias_names_list, cmd_content_list, data):
    for name in alias_names_list:
        if name in data:
            data.remove(name)

    for item in data:
        for config_items in cmd_content_list:
            config_items = config_items.split()
            for config_item in config_items:
                if config_item in data:
                    data.remove(config_item)
    return data


# Cклонение
# пример: 22, "градус", "градуса", "градусов"
def numeral_noun_declension(
    number, nominative_singular, genetive_singular, nominative_plural
):
    return (
        (number in range(5, 20))
        and nominative_plural
        or (1 in (number, (diglast := number % 10)))
        and nominative_singular
        or ({number, diglast} & {2, 3, 4})
        and genetive_singular
        or nominative_plural
    )


# Переключение раскладки клавиатуры
# ---------------------------------------------------------
def setCyrillicLayout():
    window_handle = win32gui.GetForegroundWindow()
    result = win32api.SendMessage(window_handle, 0x0050, 0, 0x04190419)
    return result


def setEngLayout():
    window_handle = win32gui.GetForegroundWindow()
    result = win32api.SendMessage(window_handle, 0x0050, 0, 0x04090409)
    return result


def check_keyboard_lang(lang):
    if lang == 68748313:
        k_l = "rus"
    elif lang == 67699721:
        k_l = "en"
    return k_l
