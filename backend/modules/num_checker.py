from modules import recognize


# ? Проверка есть ли цифры в голосовом запросе
def check_num(data):

    data_to_check = data
    dwn = recognize.value_checker(data_to_check)
    dwn = list(filter(lambda x: type(x) is int, dwn))  # Data With Numbers
    num = sum(dwn)

    if num != 0:
        print(f'Сумма чисел равна: {num}')

    return num
