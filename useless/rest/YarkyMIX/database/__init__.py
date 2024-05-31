from database.sqlite3 import SQLiteDB
from database.interface import User
from properties import const


db = SQLiteDB(lambda: const("dbMainPath"), lambda: const("dbMainTableName"), User)
