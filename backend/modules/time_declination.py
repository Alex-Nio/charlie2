# Склонение часов и минут


# ? Склонение часов
def plural_rus_variant(x):
    last_two_digits = x % 100
    tens = last_two_digits // 10
    if tens == 1:
        return 2
    ones = last_two_digits % 10
    if ones == 1:
        return 0
    if ones >= 2 and ones <= 4:
        return 1
    return 2


def show_hours(hours):
    suffix = ["час", "часа", "часов"][plural_rus_variant(hours)]
    return "{0} {1}".format(hours, suffix)


# ? Склонение минут
def conv(n):
    es = ["а", "ы", ""]
    n = n % 100
    if n >= 11 and n <= 19:
        s = es[2]
    else:
        i = n % 10
        if i == 1:
            s = es[0]
        elif i in [2, 3, 4]:
            s = es[1]
        else:
            s = es[2]
    return s


def show_minutes(minutes):
    return "{} минут{}".format(minutes, conv(minutes))
