# input() may return more than a single char,
#   so we need this buffer to store and retrieve the rest
#   on the next getchar()'s
_getchar_buffer = ""


def getchar() -> str:
    """
    Inputs a single from the standard input(buffered)

    :return: a single char
    """
    global _getchar_buffer
    while len(_getchar_buffer) == 0:
        # append to the front instead of back so that
        #   previously inputted chars remain in back
        #   and the order of retrieving chars from the back
        #   is correct
        _getchar_buffer = input() + _getchar_buffer
    # no str.pop() method sadly :(
    char = _getchar_buffer[-1]
    _getchar_buffer = _getchar_buffer[:-1]
    return char


def putchar(char: str):
    """
    Outputs a single character to the standard output

    :param char: a string with the length of 1
    """
    assert len(char) == 1
    print(end=char, flush=True)
