from input_ty import InputSeq


_FILE = "code.bf"


def get_code() -> InputSeq:
    with open(_FILE, "r") as file:
        return file.read()
