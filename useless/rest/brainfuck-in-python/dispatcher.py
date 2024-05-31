from input_ty import InputToken
from typing import Dict, Callable, Any, List


CallbackType = Callable[[List[Any]], None]

_registered: Dict[InputToken, CallbackType] = {}


def register(token: InputToken, cb: CallbackType):
    """
    Registers a callback to be invoked in the dispatching process
        when a token is found

    :param token: a trigger for the callback to happen
    :param cb: the callback
    """
    _registered[token] = cb


def dispatch(token: InputToken, *args, **kwargs):
    """
    Dispatches a single input token, notifying a registered callback
        and forwarding any extra arguments to it
    If no matching callback is found then the token is silently ignored

    :param token: a token to dispatch
    :param args: args to be passed to a callback
    :param kwargs: kwargs to be passed to a callback
    """

    try:
        _registered[token](*args, **kwargs)
    except KeyError:
        pass
