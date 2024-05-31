from copy import deepcopy


class _RestrictedInt(int):
    """
    A classic int with an upper bound when performing + and -

    Examples:
        RestrictedInt[2 ** 8] == Rust's u8
        RestrictedInt[2 ** 16] == Rust's u16
        RestrictedInt[2 ** 128] == Rust's u128
    """

    def __init__(self, value=0):
        int.__init__(value)
        self._bound = None

    def __getitem__(self, bound):
        self._bound = bound
        return deepcopy(self)

    def __add__(self, other):
        assert self._bound is not None
        other = int(other)
        return _RestrictedInt(super().__add__(other) % self._bound)[self._bound]

    def __sub__(self, other):
        assert self._bound is not None
        other = int(other)
        return _RestrictedInt(super().__sub__(other) % self._bound)[self._bound]

    def __call__(self, value=0):
        return self + value


RestrictedInt = _RestrictedInt()
