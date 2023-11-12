import requests
from bs4 import BeautifulSoup
import lxml
from random import randint
from colorama import Fore, Back, Style

headers = {
    "accept": "*/*",
    "user-agent": "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/101.0.4951.67 Safari/537.36 OPR/87.0.4390.45 (Edition Yx 05)",
}


def get_html(url):
    response = requests.get(url, headers=headers)
    return response.text


def get_data_items(html):
    soup = BeautifulSoup(html, "lxml")
    jokesList = []
    jokes = soup.findAll("p", attrs={"itemprop": "articleBody"})
    for joke in jokes:
        jokesList.append(joke.text.strip())

    return jokesList


def get_joke():
    x = randint(0, 2300)
    url = f"https://веселун.рф/anekdoty/{x}"
    result = get_data_items(get_html(url))
    x2 = randint(0, (len(result) - 1))
    result_joke = "".join(result[x2])
    print(Fore.GREEN + "Анекдот: " + Fore.CYAN + f"{result_joke}" + Style.RESET_ALL)
    return "".join(result[x2])
