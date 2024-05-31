def ignore(fun, *outer_args, **outer_kwargs):
    """
    Returns a wrapper around `fun` that ignores any arguments
        provided and calls `fun` with only arguments given here

    :param fun: the original function
    :param outer_args: args to be passed to the fun
    :param outer_kwargs: kwargs to be passed to the fun
    :return:
    """

    def wrapper(*_args, **_kwargs):
        return fun(*outer_args, **outer_kwargs)
    return wrapper
