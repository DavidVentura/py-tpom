use chrono::{DateTime, Local, NaiveDate};
use lazy_static::lazy_static;
use pyo3::prelude::*;
use pyo3::types::{PyDateAccess, PyDateTime, PyDelta, PyDeltaAccess, PyTimeAccess};
use pyo3::Python;
use std::sync::RwLock;
use tpom::{ClockController, Time, TimeSpec, TimeVal};

lazy_static! {
    static ref NOW: RwLock<f64> = RwLock::new(0.0);
}

#[pyclass]
pub struct Freezer {
    now: f64,
}

fn py_datetime_to_naive_datetime(dt: &PyDateTime) -> DateTime<Local> {
    let nd = NaiveDate::from_ymd_opt(dt.get_year(), dt.get_month() as u32, dt.get_day() as u32)
        .unwrap()
        .and_hms_micro_opt(
            dt.get_hour() as u32,
            dt.get_minute() as u32,
            dt.get_second() as u32,
            dt.get_microsecond(),
        )
        .unwrap();
    DateTime::from_local(nd, Local::now().offset().clone())
}
#[pymethods]
impl Freezer {
    #[new]
    fn new(_py: Python<'_>, dt: &PyDateTime) -> PyResult<Self> {
        let d = py_datetime_to_naive_datetime(dt);
        let mut w = NOW.write().unwrap();
        *w = d.timestamp_millis() as f64 / 1000.0;
        Ok(Freezer {
            now: d.timestamp_millis() as f64 / 1000.0,
        })
    }

    pub fn move_to<'p>(&mut self, py: Python<'p>, dt: &PyDateTime) -> PyResult<&'p PyDateTime> {
        let d = py_datetime_to_naive_datetime(dt);
        self.now = d.timestamp_millis() as f64 / 1000.0;

        let mut w = NOW.write().unwrap();
        *w = self.now;

        PyDateTime::from_timestamp(py, self.now, None)
    }

    pub fn tick<'p>(&mut self, py: Python<'p>, delta: &PyDelta) -> PyResult<&'p PyDateTime> {
        let delta = f64::from(delta.get_days() * 86400)
            + f64::from(delta.get_seconds())
            + (f64::from(delta.get_microseconds()) / 1_000_000.0);
        self.now += delta;

        let mut w = NOW.write().unwrap();
        *w = self.now;

        PyDateTime::from_timestamp(py, self.now, None)
    }
    pub fn __enter__(slf: Py<Self>) -> Py<Self> {
        ClockController::overwrite(
            Some(|_| {
                let now = NOW.read().expect("clock_gettime").clone();
                let now_sec = now.trunc() as i64;
                let delta = now - now.trunc();
                let nanos = (delta * 1_000_000_000.0) as i64;
                TimeSpec {
                    seconds: now_sec,
                    nanos,
                }
            }),
            Some(|| {
                let now = NOW.read().expect("time").clone();
                let now_sec = now.trunc() as Time;
                now_sec
            }),
            None,
            Some(|| {
                let now = NOW.read().expect("gettimeofday").clone();
                let now_sec = now.trunc() as i64;
                let delta = now - now.trunc();
                let micros = (delta * 1_000_000.0) as i64;
                TimeVal {
                    seconds: now_sec,
                    micros,
                }
            }),
        );
        slf
    }
    pub fn __exit__(&mut self, _exc_type: PyObject, _exc_value: PyObject, _traceback: PyObject) {
        ClockController::restore()
    }
}

#[pyfunction]
fn is_overwritten() -> bool {
    ClockController::is_overwritten()
}
/// A Python module implemented in Rust.
#[pymodule]
fn py_timekeeper(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<Freezer>()?;
    m.add_wrapped(wrap_pyfunction!(is_overwritten))?;
    Ok(())
}
