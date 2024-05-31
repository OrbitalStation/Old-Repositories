from copy import deepcopy
from typing import Callable, Iterator
from input_ty import InputToken


class Command:
    """
    The base class of a command family
    """

    pass


class CommandGetCursor(Command):
    """
    Asks the executor to call `cb` with the current used input token iterator
    """

    def __init__(self, cb: Callable[[Iterator[InputToken]], None]):
        self.cb = cb


class CommandSetCursor(Command):
    """
    Asks the executor to set the current used input token iterator to a value returned by `cb`
    """

    def __init__(self, cb: Callable[[], Iterator[InputToken]]):
        self.cb = cb


class CommandIgnoreTokensUntil(Command):
    """
    Asks the executor to start ignoring all the tokens until `cb` returns True
    """

    def __init__(self, cb: Callable[[InputToken], bool]):
        self.cb = cb


Executor = Callable[[Command], None]


def loop_through(seq: Iterator[InputToken], callback: Callable[[InputToken, Executor], None]):
    """
    Loops through all the tokens of `seq`, calling `callback` on each one
    Behaviour of this looping may be modified by sending commands to an executor
        provided as an argument to `callback`

    :param seq: the sequence to loop through
    :param callback: a function to be called on each token
    """

    do_ignore = False
    ignore_stop_cb = None

    def executor(command: Command):
        nonlocal seq, do_ignore, ignore_stop_cb
        if isinstance(command, CommandGetCursor):
            command.cb(deepcopy(seq))
        elif isinstance(command, CommandSetCursor):
            seq = command.cb()
        elif isinstance(command, CommandIgnoreTokensUntil):
            do_ignore = True
            ignore_stop_cb = command.cb
        else:
            raise ValueError("unknown command")

    while (item := next(seq, None)) is not None:
        if do_ignore:
            # noinspection PyCallingNonCallable
            if ignore_stop_cb(item):
                do_ignore = False
        else:
            callback(item, executor)
