from pathlib import Path
from importlib import import_module
from os import listdir


CWD = Path(__file__).parent.resolve()

for path in listdir(CWD):
    # noinspection PyTypeChecker
    if path.startswith("__") or path.startswith("."):
        continue

    # noinspection PyTypeChecker
    path = path[:path.rfind(".")]

    # noinspection PyTypeChecker
    import_module('.' + path, package=CWD.stem)
