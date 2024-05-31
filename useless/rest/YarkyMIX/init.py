from properties import *
from telebot import TeleBot


def init():
    load_properties("assets/properties")
    return TeleBot(const("token"))
