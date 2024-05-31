from cell_ty import CellType
from restricted_int import RestrictedInt


# Classic of brainfuck
ARRAY_LEN = 30_000

# Provides a storage of ARRAY_LEN elements with a movable cursor
_storage = [CellType() for _ in range(ARRAY_LEN)]
_cursor = RestrictedInt[ARRAY_LEN]


class CurrentCell:
    """
    A façade aiming at manipulating the current cell
    """

    @staticmethod
    def get() -> CellType:
        """
        :return: the current cell
        """
        return _storage[_cursor]

    @staticmethod
    def set(cell: CellType):
        """
        :param cell: a new value of the current cell
        """
        _storage[_cursor] = cell


class Cursor:
    """
    A façade aiming at manipulating the cursor
    """

    @staticmethod
    def forward():
        """
        Moves the cursor forward
        """
        global _cursor
        _cursor += 1

    @staticmethod
    def backward():
        """
        Moves the cursor backward
        """
        global _cursor
        _cursor -= 1
