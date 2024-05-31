import telebot
import sqlite3
from telebot import types
from telebot.types import InlineKeyboardMarkup, InlineKeyboardButton, ReplyKeyboardMarkup, KeyboardButton
from init import init
from database import db
from commands.class_register import class_register

if __name__ == '__main__':
    bot = init()

lessons = list()
lessonsid = list()
day = "1"
s=None

days = ['понедельник',
        'вторник',
        'среду',
        'четверг',
        'пятницу',
        'субботу',
        'воскресенье']


@bot.message_handler(commands=['start'])
def send_welcome(message):
    markup = types.ReplyKeyboardMarkup(resize_keyboard=True)
    btn1 = types.KeyboardButton("👋 Зарегистрировать класс")
    markup.add(btn1)
    bot.reply_to(message,
                 "Привет, я яркий MIX средств для помощи в обучении. Если Ваш одноклассник уже зарегистрировал Ваш "
                 "класс, то оправьте мне код, его можно узнать у того, кто регистрировал класс. Если Ваш класс ещё не "
                 "зарегистрирован, то давайте это исправим!",
                 reply_markup=markup)


@bot.message_handler(commands=['help'])
def send_welcome(message):
    bot.reply_to(message, "im gay")


@bot.message_handler(func=lambda message: True)
def send_text(message):
    if message.text.strip() == '👋 Зарегистрировать класс':
        class_register(bot, message)
    if message.text.strip() == 'Запись ДЗ':
        print(message.chat.id)
        daykb = types.InlineKeyboardMarkup(row_width=2)
        button1 = types.InlineKeyboardButton("понедельник", callback_data='mondayentry')
        button2 = types.InlineKeyboardButton("вторник", callback_data='tuesdayentry')
        button3 = types.InlineKeyboardButton("среда", callback_data='wednesdayentry')
        button4 = types.InlineKeyboardButton("черверг", callback_data='thursdayentry')
        button5 = types.InlineKeyboardButton("пятница", callback_data='fridayentry')
        button6 = types.InlineKeyboardButton("суббота", callback_data='saturdayentry')
        button7 = types.InlineKeyboardButton("суббота", callback_data='sundayentry')
        daykb.add(button1, button2, button3, button4, button5, button6, button7)
        bot.send_message(message.chat.id, "Выберите день недели", reply_markup=daykb)
    if message.text.strip() == 'Просмотр ДЗ':
        daykbread = types.InlineKeyboardMarkup(row_width=2)
        button1 = types.InlineKeyboardButton("понедельник", callback_data='mondayread')
        button2 = types.InlineKeyboardButton("вторник", callback_data='tuesdayread')
        button3 = types.InlineKeyboardButton("среда", callback_data='wednesdayread')
        button4 = types.InlineKeyboardButton("черверг", callback_data='thursdayread')
        button5 = types.InlineKeyboardButton("пятница", callback_data='fridayread')
        button6 = types.InlineKeyboardButton("суббота", callback_data='saturdayread')
        button7 = types.InlineKeyboardButton("воскресенье", callback_data='sundayread')
        daykbread.add(button1, button2, button3, button4, button5, button6, button7)
        bot.send_message(message.chat.id, "Выберите день недели", reply_markup=daykbread)


@bot.callback_query_handler(func=lambda call: True)
def callback_inline(call):
    global lessons, lessonsid, day, s
    req = call.data.split('_')
    if req[0] == "adressentry":
        s = readhw(call.message)
        bot.edit_message_text(f"{s}", chat_id=call.message.chat.id, message_id=call.message.message_id)
        bot.send_message(call.message.chat.id, "Введите домашнее задание")
        bot.register_next_step_handler(call.message, entryhw)

    if req[0] == "weekentry":
        daykb = types.InlineKeyboardMarkup(row_width=2)
        button1 = types.InlineKeyboardButton("понедельник", callback_data='mondayentry')
        button2 = types.InlineKeyboardButton("вторник", callback_data='tuesdayentry')
        button3 = types.InlineKeyboardButton("среда", callback_data='wednesdayentry')
        button4 = types.InlineKeyboardButton("черверг", callback_data='thursdayentry')
        button5 = types.InlineKeyboardButton("пятница", callback_data='fridayentry')
        button6 = types.InlineKeyboardButton("суббота", callback_data='saturdayentry')
        button7 = types.InlineKeyboardButton("воскресенье", callback_data='sundayentry')
        daykb.add(button1, button2, button3, button4, button5, button6, button7)
        bot.edit_message_text(f"Выберите день недели", reply_markup=daykb, chat_id=call.message.chat.id,
                              message_id=call.message.message_id)
    if req[0] == "weekread":
        daykb = types.InlineKeyboardMarkup(row_width=2)
        button1 = types.InlineKeyboardButton("понедельник", callback_data='mondayread')
        button2 = types.InlineKeyboardButton("вторник", callback_data='tuesdayread')
        button3 = types.InlineKeyboardButton("среда", callback_data='wednesdayread')
        button4 = types.InlineKeyboardButton("черверг", callback_data='thursdayread')
        button5 = types.InlineKeyboardButton("пятница", callback_data='fridayread')
        button6 = types.InlineKeyboardButton("суббота", callback_data='saturdayread')
        button7 = types.InlineKeyboardButton("воскресенье", callback_data='sundayread')
        daykb.add(button1, button2, button3, button4, button5, button6, button7)
        bot.edit_message_text(f"Выберите день недели", reply_markup=daykb, chat_id=call.message.chat.id,
                              message_id=call.message.message_id)
    if req[0] == "adressread":
        s = readhw(call.message)
        markup = InlineKeyboardMarkup()
        markup.row_width = 1
        markup.add(InlineKeyboardButton(text=f"назад", callback_data=f"weekread"))
        bot.edit_message_text(f"{s}", chat_id=call.message.chat.id, message_id=call.message.message_id,
                              reply_markup=markup)

    if req[0] == "mondayentry":
        dayentry(call.message,1)
    if req[0] == "tuesdayentry":
        dayentry(call.message,2)
    if req[0] == "wednesdayentry":
        dayentry(call.message,3)
    if req[0] == "thursdayentry":
        dayentry(call.message,4)
    if req[0] == "fridayentry":
        dayentry(call.message,5)
    if req[0] == "saturdayentry":
        dayentry(call.message,6)
    if req[0] == "sundayentry":
        dayentry(call.message,7)
    if req[0] == "mondayread":
        dayread(call.message,1)
    if req[0] == "tuesdayread":
        dayread(call.message,2)
    if req[0] == "wednesdayread":
        dayread(call.message,3)
    if req[0] == "thursdayread":
        dayread(call.message,4)
    if req[0] == "fridayread":
        dayread(call.message,5)
    if req[0] == "saturdayread":
        dayread(call.message,6)
    if req[0] == "sundayread":
        dayread(call.message,7)


def weekentrydays(message):
    daykb = types.InlineKeyboardMarkup(row_width=2)
    button1 = types.InlineKeyboardButton("понедельник", callback_data='mondayentry')
    button2 = types.InlineKeyboardButton("вторник", callback_data='tuesdayentry')
    button3 = types.InlineKeyboardButton("среда", callback_data='wednesdayentry')
    button4 = types.InlineKeyboardButton("черверг", callback_data='thursdayentry')
    button5 = types.InlineKeyboardButton("пятница", callback_data='fridayentry')
    button6 = types.InlineKeyboardButton("суббота", callback_data='saturdayentry')
    button7 = types.InlineKeyboardButton("воскресенье", callback_data='sundayentry')
    daykb.add(button1, button2, button3, button4, button5, button6, button7)
    bot.edit_message_text(f"Выберите день недели", reply_markup=daykb, chat_id=message.chat.id,
                          message_id=message.message_id)


def weekreaddays(message):
    daykb = types.InlineKeyboardMarkup(row_width=2)
    button1 = types.InlineKeyboardButton("понедельник", callback_data='mondayentry')
    button2 = types.InlineKeyboardButton("вторник", callback_data='tuesdayentry')
    button3 = types.InlineKeyboardButton("среда", callback_data='wednesdayentry')
    button4 = types.InlineKeyboardButton("черверг", callback_data='thursdayentry')
    button5 = types.InlineKeyboardButton("пятница", callback_data='fridayentry')
    button6 = types.InlineKeyboardButton("суббота", callback_data='saturdayentry')
    button7 = types.InlineKeyboardButton("воскресенье", callback_data='sundayentry')
    daykb.add(button1, button2, button3, button4, button5, button6, button7)
    bot.edit_message_text(f"Выберите день недели", reply_markup=daykb, chat_id=message.chat.id,
                          message_id=message.message_id)


def weekentry(message):
    markup = ReplyKeyboardMarkup(resize_keyboard=True)
    btn1 = KeyboardButton("Просмотр ДЗ")
    btn2 = KeyboardButton("Запись ДЗ")
    markup.add(btn1, btn2)
    bot.send_message(message.chat.id, "Домашнее задание записано, выберите что хотите сделать дальше",
                     reply_markup=markup)


def entryhw(message):
    global day,s
    s = message.text.strip()
    uid = message.chat.id
    user = db.fetch_user(uid)
    user.get_hw_db().update_user(1, **{f"wd{day}_str_repr": f"{s}"})
    weekentry(message)


def readhw(message):
    global day, s
    user = db.fetch_user(message.chat.id)
    print(day)
    s = getattr(user.get_hw_db().fetch_user(1), f"wd{day}")
    s = s.str_repr
    print(s)
    if s == "":
        s = "Не записано"
    return s


def dayentry(message, num):
    global days,day
    day = num
    lessonsid = db.fetch_user(message.chat.id).subjects.decode()
    lessons = getattr(db.fetch_user(message.chat.id).general_schedule, f"wd{num}").lessons.decode()
    markup = InlineKeyboardMarkup()
    markup.row_width = 2
    for i in range(0, len(lessons)):
        markup.add(InlineKeyboardButton(lessonsid[int(lessons[i].idx)].name, callback_data="adressentry"))
    markup.add(InlineKeyboardButton(text=f"назад", callback_data=f"weekentry"))
    bot.edit_message_text(f'расписание на {days[num-1]}', reply_markup=markup, chat_id=message.chat.id,
                          message_id=message.message_id)
def dayread(message, num):
    global days,day
    day = num
    lessonsid = db.fetch_user(message.chat.id).subjects.decode()
    lessons = getattr(db.fetch_user(message.chat.id).general_schedule, f"wd{num}").lessons.decode()
    markup = InlineKeyboardMarkup()
    markup.row_width = 2
    for i in range(0, len(lessons)):
        markup.add(InlineKeyboardButton(lessonsid[int(lessons[i].idx)].name, callback_data="adressread"))
    markup.add(InlineKeyboardButton(text=f"назад", callback_data=f"weekread"))
    bot.edit_message_text(f'расписание на {days[num-1]}', reply_markup=markup, chat_id=message.chat.id,
                          message_id=message.message_id)



bot.polling(none_stop=True)
