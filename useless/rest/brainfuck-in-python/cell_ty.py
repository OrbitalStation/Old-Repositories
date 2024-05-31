from restricted_int import RestrictedInt

# A canonical 2 ** 8 variants brainfuck cell
CELL_MAX_VALUE = 256

# An alias to the actual int type
CellType = RestrictedInt[CELL_MAX_VALUE]
