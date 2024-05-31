from __future__ import annotations

from dataclasses import dataclass
from properties import const


@dataclass(frozen=True)
class LessonDecoded:
    # Index in Subjects list
    idx: int


@dataclass(frozen=True)
class LessonsList:
    # <lessonData>;<nextLesson>;etc.
    str_repr: str

    def decode(self) -> list[LessonDecoded]:
        return [LessonDecoded(idx) for idx in self.str_repr.split(';') if idx != ""]


@dataclass(frozen=True)
class WeekDaySchedule:
    lessons: LessonsList


@dataclass(frozen=True)
class Schedule:
    wd1: WeekDaySchedule
    wd2: WeekDaySchedule
    wd3: WeekDaySchedule
    wd4: WeekDaySchedule
    wd5: WeekDaySchedule
    wd6: WeekDaySchedule
    wd7: WeekDaySchedule


@dataclass(frozen=True)
class SubjectDecoded:
    name: str


@dataclass(frozen=True)
class Subjects:
    # physics;math;biology <-etc.
    str_repr: str

    def decode(self) -> list[SubjectDecoded]:
        return [SubjectDecoded(name=s) for s in self.str_repr.split(';')]


@dataclass(frozen=True)
class HWSubjectPresent:
    homework: str


@dataclass(frozen=True)
class HWSubjectNoHomework:
    STR_REPR = "<not given>"


@dataclass(frozen=True)
class HWSubjectNotSetYet:
    pass


@dataclass(frozen=True)
class Homework:
    # hw1::hw2::hw3::etc.
    str_repr: str

    @staticmethod
    def default(nsubjects: int):
        return "::" * (nsubjects - 1)

    def decode(self) -> list[HWSubjectPresent|HWSubjectNotSetYet|HWSubjectNoHomework]:
        return [(HWSubjectPresent(homework=hw) if hw != HWSubjectNoHomework.STR_REPR else
                  (HWSubjectNoHomework() if hw != "" else HWSubjectNotSetYet())) for hw in self.str_repr.split('::')]

    @staticmethod
    def encode(lst: list[HWSubjectPresent|HWSubjectNotSetYet|HWSubjectNoHomework]):
        def one(hw):
            if isinstance(hw, HWSubjectPresent):
                return hw.homework
            elif isinstance(hw, HWSubjectNoHomework):
                return HWSubjectNoHomework.STR_REPR
            elif isinstance(hw, HWSubjectNotSetYet):
                return ""
            else:
                assert False, "ACHTUNG!!!"
        return Homework(sum(map(one, lst)))


@dataclass(frozen=True)
class HWUser:
    # Primary key
    uid: int
    wd1: Homework
    wd2: Homework
    wd3: Homework
    wd4: Homework
    wd5: Homework
    wd6: Homework
    wd7: Homework


@dataclass(frozen=True)
class User:
    # Primary key
    uid: int
    general_schedule: Schedule
    subjects: Subjects

    def get_hw_db(self):
        from .sqlite3 import SQLiteDB
        DEFAULT_FOR_NEW = {f'wd{i}': Homework.default(getattr(self.general_schedule, f"wd{i}").lessons.str_repr.count(';') + 1) for i in range(1, 8)}
        return SQLiteDB(const("dbMainPath"), f"hw{self.uid}", HWUser, default_values_for_new_users=DEFAULT_FOR_NEW)


class Database:
    def update_user(self, uid: int, **fields):
        raise NotImplementedError

    def fetch_user(self, uid: int):
        raise NotImplementedError
