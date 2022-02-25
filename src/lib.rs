#![feature(once_cell)]
use std::{lazy::Lazy, sync::Mutex};
use std::hash::Hash;
use pyo3::prelude::*;
use pyo3::exceptions::PyTypeError;

struct BaseUnit {
    name: String,
    long_name: String
}

#[pyfunction]
fn add_unit(name: String, long_name: String) -> PyResult<Unum> {
    let len: usize;

    unsafe {
        let mut units = UNITS.lock().unwrap();
        units.push(BaseUnit { name, long_name });
        len = units.len()
    }

    let mut u = vec![0; len];
    u[len - 1] = 1;

    Ok(
        Unum {
            val: 1f64,
            unit: NumberUnit {
                u
            }
        }
    )
}

static mut UNITS: Lazy<Mutex<Vec<BaseUnit>>> = Lazy::new(|| Mutex::new(vec![]));

fn current_unit_count() -> usize {
    unsafe {
        *&UNITS.lock().unwrap().len()
    }
}

#[inline]
fn unwrap_unum(obj: &PyAny) -> Unum {
    match obj.extract() {
        Ok(u) => u,
        Err(_) => {
            Unum {
                val: obj.extract().unwrap(),
                unit: NumberUnit{ u: vec![] }
            }
        }
    }
}

#[derive(Clone, PartialEq)]
struct NumberUnit {
    u: Vec<i16>
}

impl ToString for NumberUnit {
    fn to_string(&self) -> String {
        let mut numerator: String = "".to_string();
        let mut denominator: String = "".to_string();
        for (i, p) in self.u.iter().enumerate() {
            if *p == 0 { continue; }
            unsafe {
                let n = &UNITS.lock().unwrap()[i].name;
                if *p == 1 { numerator += n }
                else if *p == -1 { denominator += n }
                else if *p > 1 { numerator += &*(n.to_owned() + &p.to_string()) }
                else { denominator += &*(n.to_owned() + &(-p).to_string()) }
            }
        }
        return if denominator == "" {
            numerator
        } else {
            numerator + "/" + &*denominator
        }
    }
}

#[pyclass]
#[derive(Clone)]
struct Unum {
    val: f64,
    unit: NumberUnit
}

#[pymethods]
impl Unum {
    #[new]
    fn new(val: f64) -> Self {
        Unum { val, unit: NumberUnit{ u: vec![0; current_unit_count()] } }
    }

    fn __str__(&self) -> PyResult<String> {
        Ok(self.val.to_string() + " [" + &*self.unit.to_string() + "]")
    }

    fn __mul__(&self, other: &PyAny) -> PyResult<Unum> {
        let o = unwrap_unum(other);
        let mut unit_vec = vec![0; current_unit_count()];
        for i in 0..self.unit.u.len() {
            unit_vec[i] = self.unit.u[i]
        }
        for i in 0..o.unit.u.len() {
            unit_vec[i] += o.unit.u[i]
        }
        Ok(Unum {
            val: self.val * o.val,
            unit: NumberUnit { u: unit_vec }
        })
    }

    fn __div__(&self, other: &PyAny) -> PyResult<Unum> {
        let o = unwrap_unum(other);
        let mut unit_vec = vec![0; current_unit_count()];
        for i in 0..self.unit.u.len() {
            unit_vec[i] = self.unit.u[i]
        }
        for i in 0..o.unit.u.len() {
            unit_vec[i] -= o.unit.u[i]
        }
        Ok(Unum {
            val: self.val / o.val,
            unit: NumberUnit { u: unit_vec }
        })
    }

    fn __add__(&self, other: &Unum) -> PyResult<Unum> {
        return if self.unit == other.unit {
            Ok(Unum {
                val: self.val + other.val,
                unit: self.unit.clone()
            })
        } else {
            Err(PyTypeError::new_err("unit mismatch"))
        }
    }

    fn __sub__(&self, other: &Unum) -> PyResult<Unum> {
        return if self.unit == other.unit {
            Ok(Unum {
                val: self.val - other.val,
                unit: self.unit.clone()
            })
        } else {
            Err(PyTypeError::new_err("unit mismatch"))
        }
    }

    #[inline]
    fn __rmul__(&self, other: &PyAny) -> PyResult<Unum> {
        self.__mul__(other)
    }

    #[inline]
    fn __truediv__(&self, other: &PyAny) -> PyResult<Unum> {
        self.__div__(other)
    }

    #[inline]
    fn __rdiv__(&self, other: &PyAny) -> PyResult<Unum> {
        self.__div__(other)
    }

    #[inline]
    fn __rtruediv__(&self, other: &PyAny) -> PyResult<Unum> {
        self.__div__(other)
    }
}

#[pymodule]
fn fast_unit(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<Unum>()?;
    m.add_function(wrap_pyfunction!(add_unit, m)?)?;

    Ok(())
}
