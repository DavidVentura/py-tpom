from py_timekeeper import Freezer
from datetime import datetime, timedelta

def test_time_changes():
    target = datetime(2012, 1, 14, 1, 2, 3)
    assert datetime.now() != target
    with Freezer(target):
        assert datetime.now() == target
    assert datetime.now() != target

def test_dst_change():
    start = datetime(2012, 1, 14, 1, 59, 59)
    after_dst = datetime(2012, 1, 14, 0, 59, 59)
    with Freezer(start) as ft:
        assert datetime.now() == start
        ft.tick(timedelta(hours=-1))
        assert datetime.now() != start
        assert datetime.now() == after_dst
