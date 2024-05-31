from telebot import types

from database import db
from telebot.types import ReplyKeyboardMarkup, KeyboardButton, InlineKeyboardMarkup, InlineKeyboardButton


def class_register(bot, message):
    markup = types.ReplyKeyboardMarkup(resize_keyboard=True)
    btn1 = types.KeyboardButton("Нет уроков")
    markup.add(btn1)
    reply = bot.send_message(message.chat.id, 'Привет! Чтобы у меня была возможность помогать тебе, мне надо '
                                              'узнать твоё расписание. Напиши мне своё расписание на понедельник. '
                                              'Пример:\nМатематика\nРусский\nФизра\nОБЖ\nАнглийский',
                             reply_markup=markup)
    bot.register_next_step_handler(reply, weekday(bot, [
        '',
        'воскресение',
        'субботу',
        'пятницу',
        'четверг',
        'среду',
        'вторник'
    ], []))


def weekday(bot, remains: list[str], subs):
    if len(remains) != 0:
        day = remains.pop()

    def inner(message):
        markup = types.ReplyKeyboardMarkup(resize_keyboard=True)
        btn1 = types.KeyboardButton("Нет уроков")
        markup.add(btn1)
        _subs_append(message, subs)
        reply = bot.send_message(message.chat.id, f'На {day}', reply_markup=markup)
        bot.register_next_step_handler(reply, weekday(bot, remains, subs))

    if len(remains) == 0:
        return lambda m: finish_weekdays(bot, m, subs)
    return inner


def finish_weekdays(bot, message, subs):
    subjects = _pre_handle(message, subs)
    _update_all_subjects(message, subjects)
    _add_hw_buttons(message, bot)
    _create_schedule(message, subs, subjects)


def _add_hw_buttons(message, bot):
    markup = ReplyKeyboardMarkup(resize_keyboard=True)
    btn1 = KeyboardButton("Просмотр ДЗ")
    btn2 = KeyboardButton("Запись ДЗ")
    markup.add(btn1, btn2)
    bot.send_message(message.chat.id, "Поздравляю! Вы заполнили расписание. Теперь можно записывать и "
                                      "просматривать дз", reply_markup=markup)


def _pre_handle(message, subs):
    _subs_append(message, subs)
    return list(set([item.replace(';', ' ') for lit in subs for item in lit]))


def _update_all_subjects(message, subs):
    db.update_user(message.from_user.id, subjects_str_repr=";".join(subs))


def _create_schedule(message, subs, subjects):
    for idx, day in enumerate(subs):
        dbname = f"general_schedule_wd{idx+1}_lessons_str_repr"
        dbdata = ";".join(map(lambda s: str(subjects.index(s)), day))
        db.update_user(message.from_user.id, **{dbname: dbdata})


def _subs_append(message, subs):
    if message.text != "Нет уроков":
        subs.append(message.text.strip().split("\n"))
    else:
        subs.append([])

