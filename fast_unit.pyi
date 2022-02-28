import typing


class Unum:
    def __init__(self, val: float): ...

    def as_number(self, u: Unum) -> float: ...

    def __str__(self) -> str: ...
    __repr__ = __str__

    @typing.overload
    def __mul__(self, other: Unum) -> Unum: ...
    @typing.overload
    def __mul__(self, other: float) -> Unum: ...
    __rmul__ = __mul__

    @typing.overload
    def __div__(self, other: Unum) -> Unum: ...
    @typing.overload
    def __div__(self, other: float) -> Unum: ...
    __truediv__ = __rdiv__ = __rtruediv__ = __div__

    @typing.overload
    def __pow__(self, power: Unum, modulo=None) -> Unum: ...
    @typing.overload
    def __pow__(self, power: float, modulo=None) -> Unum: ...

    def __add__(self, other: Unum) -> Unum: ...
    def __sub__(self, other: Unum) -> Unum: ...

    def __pos__(self) -> Unum: ...
    def __neg__(self) -> Unum: ...

    def __abs__(self) -> Unum: ...

    def __iadd__(self, other: Unum) -> Unum: ...
    def __isub__(self, other: Unum) -> Unum: ...
    @typing.overload
    def __imul__(self, other: Unum) -> Unum: ...
    @typing.overload
    def __imul__(self, other: float) -> Unum: ...
    @typing.overload
    def __idiv__(self, other: Unum) -> Unum: ...
    @typing.overload
    def __idiv__(self, other: float) -> Unum: ...
    @typing.overload
    def __itruediv__(self, other: Unum) -> Unum: ...
    @typing.overload
    def __itruediv__(self, other: float) -> Unum: ...
    @typing.overload
    def __ipow__(self, other: Unum) -> Unum: ...
    @typing.overload
    def __ipow__(self, other: float) -> Unum: ...


def add_unit(name: str, long_name: str) -> Unum: ...