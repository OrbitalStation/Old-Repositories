def match(pattern, data, err):
    i = 0
    one = 1 / len(pattern)
    cur = 0
    while i < len(pattern):
        if i >= len(data):
            cur += one
        else:
            try:
                if data[i] != pattern[i]:
                    cur += one
            except IndexError:
                cur += one
        if cur > err:
            return False
        i += 1
    return True


def see(pattern, string):
    for i in range(len(string.lower())):
        if match(pattern, string[i:], 0.1):
            return True
    return False


dataset = [
    'я продаю',
    'я представитель',
    'введите',
    'скажите код на обратной стороне',
    'вам зачислено',
    'вашу карту взломали',
    'я из службы безопасности',
    'кредит',
    'банк',
    'соцопрос'
]


def check(data):
    for i in dataset:
        if see(i, data):
            return True
    return False
