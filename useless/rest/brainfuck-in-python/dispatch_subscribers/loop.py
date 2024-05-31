from copy import deepcopy
import dispatcher
import storage
import seq_looper


OPEN = "["
CLOSE = "]"
met_opening_brackets_when_ignoring_all_commands = 0
labels = []


def stop_ignoring_cb(item):
    global met_opening_brackets_when_ignoring_all_commands

    if item == CLOSE:
        met_opening_brackets_when_ignoring_all_commands -= 1
    elif item == OPEN:
        met_opening_brackets_when_ignoring_all_commands += 1

    return met_opening_brackets_when_ignoring_all_commands == 0


def open_f(executor):
    global met_opening_brackets_when_ignoring_all_commands

    if storage.CurrentCell.get() != 0:
        executor(seq_looper.CommandGetCursor(labels.append))
    else:
        met_opening_brackets_when_ignoring_all_commands = 1
        executor(seq_looper.CommandIgnoreTokensUntil(stop_ignoring_cb))


def close_f(executor):
    global met_opening_brackets_when_ignoring_all_commands

    if storage.CurrentCell.get() != 0:
        executor(seq_looper.CommandSetCursor(lambda: deepcopy(labels[-1])))
    else:
        labels.pop()


dispatcher.register(OPEN, open_f)
dispatcher.register(CLOSE, close_f)
