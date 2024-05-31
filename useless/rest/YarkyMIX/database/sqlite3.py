from __future__ import annotations

from properties import const
from sqlite3 import Cursor
from database.interface import Database, User
import dataclasses as dc
from typing import get_type_hints
import sqlite3


PY2SQL = {
    'int': 'INT',
    'str': 'TEXT',
    'bool': 'BOOLEAN'
}


def _annotations(duck: type):
    resolved = get_type_hints(duck)
    return map(lambda f: (f.name, resolved[f.name]), dc.fields(duck))


def _unfold_annotations(field: str, ty: type) -> dict[str, type]:
    if PY2SQL.get(ty.__name__) is not None:
        return {field: ty}
    if field != "":
        field += '_'
    result = {}
    for subfield, subty in _annotations(ty):
        result.update(_unfold_annotations(field + subfield, subty))
    return result


def _dataclass_from_sql(dataclass: type, sql: list):
    result = []
    for _field, ty in _annotations(dataclass):
        if PY2SQL.get(ty.__name__) is not None:
            result.append(sql.pop())
            continue
        result.append(_dataclass_from_sql(ty, sql))
    return dataclass(*result)


class SQLiteDB(Database):
    def __init__(self, path_p: str, name_p: str, user: type,
                 default_values_for_new_users: dict[str, str] | None = None):
        self._path = path_p
        self._name = name_p
        self.__path = None
        self.__name = None
        self._new_user_defaults = default_values_for_new_users
        self._UFIELDS = _unfold_annotations("", user)
        self._user_ty = user

        def table():
            if self.__CREATE_TABLE_SQL is None:
                fields = ', '.join(
                    [f'{field} {PY2SQL[value.__name__]}{" PRIMARY KEY" if field == "uid" else ""}'
                     for field, value in self._UFIELDS.items()])
                self.__CREATE_TABLE_SQL = f"CREATE TABLE IF NOT EXISTS {self.name()}({fields});"
            return self.__CREATE_TABLE_SQL

        self._CREATE_TABLE_SQL = table
        self.__CREATE_TABLE_SQL = None

        def userr():
            if self.__CREATE_USER_SQL is None:
                fields, values = zip(*[(field, ty()) for field, ty in self._UFIELDS.items() if field != 'uid'])
                placeholders = ('?,' * len(self._UFIELDS))[:-1]
                self.__CREATE_USER_SQL = f"INSERT INTO {self.name()}" \
                                         f" (uid, {', '.join(fields)}) VALUES({placeholders});", values
            return self.__CREATE_USER_SQL

        self._CREATE_USER_SQL = userr
        self.__CREATE_USER_SQL = None

    def path(self):
        if self.__path is None:
            self.__path = self._path() if callable(self._path) else self._path
            del self._path
        return self.__path

    def name(self):
        if self.__name is None:
            self.__name = self._name() if callable(self._name) else self._name
            del self._name
        return self.__name

    def update_user(self, uid: int, **kwargs):
        if _fetch_user_or_none_if_nonpresent(uid, self.name(), self.path(), self._CREATE_TABLE_SQL(),
                                             self._user_ty) is None:
            _create_user(uid, self.path(), self._CREATE_USER_SQL(), self._new_user_defaults, self)
        update = ", ".join([(field + ' = ' + '"' + value.replace('"', '""') + '"') for field, value in kwargs.items()])
        _mutate(f'UPDATE {self.name()} SET {update} WHERE uid = ?', self.path(), (uid,))

    def fetch_user(self, uid: int):
        if (fetched := _fetch_user_or_none_if_nonpresent(uid, self.name(), self.path(), self._CREATE_TABLE_SQL(),
                                                         self._user_ty)) is None:
            _create_user(uid, self.path(), self._CREATE_USER_SQL(), self._new_user_defaults, self)
            fetched = _fetch_user_or_none_if_nonpresent(uid, self.name(), self.path(), self._CREATE_TABLE_SQL(),
                                                        self._user_ty)
        return fetched


def _fetch_user_or_none_if_nonpresent(uid: int, name, path, TABLE_SQL, user_ty: type):
    _create_table_if_not_exists(path, TABLE_SQL)
    if (fetched := _fetch(f"SELECT * FROM {name} WHERE uid=?", path, (uid,)).fetchone()) is None:
        return
    return _dataclass_from_sql(user_ty, list(fetched)[::-1])


def _create_table_if_not_exists(path, TABLE_SQL):
    _mutate(TABLE_SQL, path)


def _create_user(uid: int, path, USER_SQL, defaults, db):
    _mutate(USER_SQL[0], path, (uid, *USER_SQL[1]))
    if defaults is not None:
        db.update_user(uid, **defaults)


def _mutate(request: str, path, *args, **kwargs):
    print(request)
    con = sqlite3.connect(path)
    cur = con.cursor()
    cur.execute(request, *args, **kwargs)
    con.commit()
    cur.close()


def _fetch(request: str, path, *args, **kwargs) -> Cursor:
    con = sqlite3.connect(path)
    cur = con.cursor()
    return cur.execute(request, *args, **kwargs)
