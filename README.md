# Py-TPOM

This package hijacks the system's clock, which is useful for testing. It is similar to [freezegun](https://github.com/spulec/freezegun) and [py-libfaketime](https://github.com/simon-weber/python-libfaketime), the main differences are: it will override clock access for _everything_ in the process (not just Python code) and it does not need files on disk (shared libraries or configuration files for `faketime`).

```python
from py_timekeeper import Freezer, is_overwritten
from datetime import datetime, timedelta

def test_overwritten():
    assert not is_overwritten()
    with Freezer(datetime.now()):
        assert is_overwritten()
    assert not is_overwritten()

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
```
