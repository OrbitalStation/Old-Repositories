from random import randint, randrange

# Constants
MINW, MINH, MAXW, MAXH = 7, 7, 9, 9

MIN_MINES, MAX_MINES = 10, 15

SEPARATOR = "--------------------------"

CMD_PREFIX = "> "

CLOSED_CELL = "_"
OPENED_CELL = "."
FLAGGED_CELL = "F"
QUESTIONED_CELL = "?"


width, height = randint(MINW, MAXW), randint(MINH, MAXH)
mines_num = randint(MIN_MINES, MAX_MINES)


TOP_BAR = "** " + " ".join(map(str, range(1, width + 1)))
UNDER_TOP_BAR = "*" + "#" * (len(TOP_BAR) - 1)
LEFT_BAR = "".join(map(str, range(1, height + 1)))
RIGHTER_LEFT_BAR = "#" * len(LEFT_BAR)


# Types

class Field:
    def __init__(self, line, column):
        self._field = []
        self._remaining_fields = width * height - mines_num - 1
        self._remaining_flags = mines_num
        for _ in range(height):
            self._field.append([])
            for _ in range(width):
                self._field[-1].append(ClosedCell())

        self._field[line][column] = OpenedCell()

        remain = mines_num
        while remain > 0:
            ln = randrange(0, height)
            col = randrange(0, width)
            if issubclass(type(self._field[ln][col]), ClosedCellWithMine):
                continue
            if abs(line - ln) <= 1 and abs(column - col) <= 1:
                continue

            self._field[ln][col] = ClosedCellWithMine()

            remain -= 1

        self._open_cells_surrounding_this_if_they_are_clear(line, column)

    def _open_cells_surrounding_this_if_they_are_clear(self, line, column):
        def inner(_, y, x):
            if type(self._field[y][x]) is not ClosedCell:
                return
            self._remaining_fields -= 1
            mines_around = self._get_mines_around_cell(y, x)
            if mines_around == 0:
                self._field[y][x] = OpenedCell()
                self._open_cells_surrounding_this_if_they_are_clear(y, x)
            else:
                self._field[y][x] = OpenedCellWithMinesAround(mines_around)

        iterate_over_cell_neighbors(line, column, inner)

    def _get_mines_around_cell(self, line, column):
        def inner(acc, y, x):
            if issubclass(type(self._field[y][x]), ClosedCellWithMine):
                acc += 1
            return acc

        return iterate_over_cell_neighbors(line, column, inner, 0)

    def get(self, line, column):
        return self._field[line][column]

    def open(self, line, column):
        cell = self._field[line][column]

        if not issubclass(type(cell), ClosedCell):
            raise ValueError('already opened')
        if cell.is_marked():
            raise ValueError(f'marked with {cell.get_mark()}')

        if issubclass(type(cell), ClosedCellWithMine):
            raise StopIteration

        mines = self._get_mines_around_cell(line, column)
        self._remaining_fields -= 1
        if mines == 0:
            self._field[line][column] = OpenedCell()
            self._open_cells_surrounding_this_if_they_are_clear(line, column)
        else:
            self._field[line][column] = OpenedCellWithMinesAround(mines)

    def flag(self, line, column):
        if not issubclass(type(self._field[line][column]), ClosedCell):
            raise ValueError('not a closed cell; cannot mark it')
        if self._remaining_flags == 0:
            raise ValueError('able to be marked: no flags remain')
        self._field[line][column].flag()
        self._remaining_flags -= 1

    def question(self, line, column):
        if not issubclass(type(self._field[line][column]), ClosedCell):
            raise ValueError('not a closed cell; cannot mark it')
        self._field[line][column].question()

    def unmark(self, line, column):
        if not issubclass(type(self._field[line][column]), ClosedCell):
            raise ValueError('not a closed cell; cannot unmark it')
        if not self._field[line][column].is_marked():
            raise ValueError('not a marked cell; cannot unmark it')
        if self._field[line][column].get_mark() == FLAGGED_CELL:
            self._remaining_flags += 1
        self._field[line][column].unmark()

    def are_all_non_mined_cells_opened(self):
        return self._remaining_fields <= 0

    def get_remaining_flags(self):
        return self._remaining_flags

    def show_all_mines(self):
        for y in range(height):
            for x in range(width):
                if issubclass(type(self._field[y][x]), ClosedCellWithMine):
                    self._field[y][x].show()


class Cell:
    """ Base class for all cells """
    pass


class OpenedCell(Cell):
    """ An opened cell """
    def __str__(self):
        return OPENED_CELL


class OpenedCellWithMinesAround(OpenedCell):
    """ An opened cell with some mines around it """
    def __init__(self, mines: int):
        assert 1 <= mines <= 8
        self.mines = str(mines)

    def __str__(self):
        return self.mines


class ClosedCell(Cell):
    """ A closed cell with optional mark """

    def __init__(self):
        self._str = CLOSED_CELL

    def flag(self):
        self._str = FLAGGED_CELL

    def question(self):
        self._str = QUESTIONED_CELL

    def unmark(self):
        self._str = CLOSED_CELL

    def is_marked(self):
        return self._str != CLOSED_CELL

    def get_mark(self):
        return self._str

    def __str__(self):
        return self.get_mark()


class ClosedCellWithMine(ClosedCell):
    """ A closed cell with a mine and optional mark """

    def __init__(self):
        super().__init__()
        self._hide = True

    def show(self):
        self._hide = False

    def __str__(self):
        if self._hide:
            return super().__str__()
        return 'x'

    pass


# Functions
def separated(*args, **kwargs):
    print(SEPARATOR)
    print(*args, **kwargs)
    print(SEPARATOR)
    print()


def validate(s, name, bound):
    try:
        result = int(s)
    except ValueError:
        print(f'ERROR: <{name}> is not a valid integer')
        raise

    if not (1 <= result <= bound):
        print(f'ERROR: <{name}> is out of bounds [1..{bound}]')
        raise ValueError()

    return result - 1


def iterate_over_cell_neighbors(line, column, job, init=None):
    for opx, opy in [
        ('-1', '-1'),
        ('-1', ''),
        ('-1', '+1'),
        ('', '-1'),
        ('', '+1'),
        ('+1', '-1'),
        ('+1', ''),
        ('+1', '+1')
    ]:
        y, x = eval(f'line {opx}'), eval(f'column {opy}')
        if not (0 <= y < height) or not (0 <= x < width):
            continue
        init = job(init, y, x)
    return init


def print_field(fake=False):
    print(TOP_BAR)
    print(UNDER_TOP_BAR)
    for y in range(height):
        print(LEFT_BAR[y], RIGHTER_LEFT_BAR[y], end=" ", sep="")
        for x in range(width):
            if fake:
                print(CLOSED_CELL, end=" ")
            else:
                print(field.get(y, x), end=" ")
        print()
    print()


# Manual
def _help():
    print("Welcome to the Minesweeper game!")
    print("Author: Roman Tarasenko.")
    print("Each turn you'll see a field and a command line.")
    print(f"Field is of size [{MINW}..{MAXW}]x[{MINH}..{MAXH}].")
    print(f"Current field size if {width}x{height}.")
    print("A cell of the field is one of '_.F?' or a 1 to 8 number.")
    print(f"    `{CLOSED_CELL}` -- cell is closed, i.e. may contain a mine.")
    print(f"    `{OPENED_CELL}` -- cell is opened, i.e. does not contain a mine.")
    print(f"    `{FLAGGED_CELL}` -- closed cell marked with a flag sign.")
    print(f"    `{QUESTIONED_CELL}` -- closed cell marked with a question sign.")
    print("    number -- opened cell, indicates number of active mines")
    print("      around the cell.")
    print("`F` and `?` marks prevent you from opening a cell,")
    print("  you must remove them first.")
    print("Note that amount of cells you can mark with `F` is limited to number of mines,")
    print("  but amount of question marks is unlimited.")
    print("You can type commands after the `>` sign.")
    print("Turn is finished as soon as a command is typed and executed.")
    print("After you typed a command, press `Enter` to execute it.")
    print("Available commands:")
    print("    `open <line> <column>` -- opens a cell at line <line>")
    print("       and column <column>.")
    print("       Fails if this cell contains a mark or is already opened.")
    print("       The game is lost if a mine was opened.")
    print()
    print("    `flag <line> <column>` -- marks a cell with the `F` sign.")
    print("       Fails if this cell is not closed.")
    print()
    print("    `question <line> <column>` -- marks a cell with the `?` sign.")
    print("       Fails if this cell is not closed.")
    print()
    print("    `unmark <line> <column>` -- removes mark from the cell.")
    print("       Fails if this cell does not contain any mark.")
    print()
    print("    `exit` -- exits the game.")
    print()
    print("    `help` -- shows this info.")
    print("Both <line> and <column> in each command start with 1 and should not exceed maximum bound.")
    print()
    print("The game is won as soon as all non-mine-containing cells were opened.")
    print(SEPARATOR)
    print("Type `Enter` after you have read the information above.")
    input()


if __name__ == '__main__':
    separated("STARTING GAME")

    print('NOTE: Type `help` to get help')
    print()

    field = None

    # Game loop
    turn = 1
    while True:
        separated("TURN", turn)

        if field is None:
            print_field(fake=True)
        else:
            print_field()
            print('REMAINING FLAGS:', field.get_remaining_flags())

        while True:
            cmd = input(CMD_PREFIX)

            args = cmd.split()
            if len(args) == 0:
                continue

            cmd, args = args[0], args[1:]

            if cmd == 'exit':
                print('Exiting...')
                exit()
            if cmd == 'help':
                _help()

            if len(args) != 2:
                print('ERROR: expected 2 arguments: <line> <column>')
                continue

            try:
                line = validate(args[0], 'line', height)
                column = validate(args[1], 'column', width)
            except ValueError:
                continue

            try:
                if cmd == 'open':
                    if field is None:
                        field = Field(line, column)
                    else:
                        field.open(line, column)

                    if field.are_all_non_mined_cells_opened():
                        separated('FINISHING GAME')
                        print('Yay, you have won the game!')
                        print(SEPARATOR)
                        field.show_all_mines()
                        print_field()
                        exit()

                elif cmd in ('flag', 'question', 'unmark'):
                    if field is None:
                        raise NameError
                    else:
                        getattr(field, cmd)(line, column)
                else:
                    print('ERROR: unknown command!')
                    continue
            except ValueError as e:
                print(f'ERROR: cell at {line + 1}:{column + 1} is ', *e.args)
                continue
            except StopIteration:
                separated("FINISHING GAME")
                print('Whoops, it looks like you stepped on a mine!')
                print(SEPARATOR)
                field.show_all_mines()
                print_field()
                exit()
            except NameError:
                print(f'ERROR: open some cell first!')
                continue

            break

        turn += 1

