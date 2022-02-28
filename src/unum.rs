use pyo3::exceptions::PyTypeError;
use pyo3::prelude::*;

use crate::NumberUnit;
use crate::unit_reg::current_unit_count;


#[pyclass]
#[derive(Clone)]
pub(crate) struct Unum {
    pub(crate) val: f64,
    pub(crate) unit: NumberUnit
}

#[pymethods]
impl Unum {
    #[new]
    fn new(val: f64) -> Self {
        Unum { val, unit: NumberUnit{ u: vec![0; current_unit_count()] } }
    }

    fn as_number(&self, u: &Unum) -> PyResult<f64> {
        return if self.unit == u.unit {
            Ok(self.val / u.val)
        } else {
            Err(PyTypeError::new_err("Unit Mismatch"))
        }
    }

    fn __pos__(&self) -> PyResult<Unum> {
        Ok(self.clone())
    }

    fn __str__(&self) -> PyResult<String> {
        Ok(self.val.to_string() + " [" + &*self.unit.to_string() + "]")
    }

    #[inline]
    fn __repr__(&self) -> PyResult<String> {
        self.__str__()
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
            Err(PyTypeError::new_err("Unit Mismatch"))
        }
    }

    fn __sub__(&self, other: &Unum) -> PyResult<Unum> {
        return if self.unit == other.unit {
            Ok(Unum {
                val: self.val - other.val,
                unit: self.unit.clone()
            })
        } else {
            Err(PyTypeError::new_err("Unit Mismatch"))
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
