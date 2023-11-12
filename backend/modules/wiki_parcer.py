import config
import requests
from bs4 import BeautifulSoup
from modules.methods import alias_replacer
from modules import tts

headers = {
    "accept": "*/*",
    "user-agent": "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/101.0.4951.67 Safari/537.36 OPR/87.0.4390.45 (Edition Yx 05)",
}


def get_html(url):
    response = requests.get(url, headers=headers)
    return response.text


def get_data_items(html):
    dataList = []
    soup = BeautifulSoup(html, "lxml")
    objs = soup.findAll("p")

    for obj in objs:
        dataList.append(obj.text.strip())
        break

    return dataList


def wiki_parcer(data):
    replace_aliases = config.VA_WIKI["wiki_cmd"]
    alias_names = config.VA_ALIAS
    x = alias_replacer(alias_names, replace_aliases, data)

    x = " ".join(x)
    print(x.capitalize())
    url = f"https://ru.wikipedia.org/wiki/{x}"
    result = get_data_items(get_html(url))
    result = "".join(result)
    print(result)
    tts.va_speak(result)
