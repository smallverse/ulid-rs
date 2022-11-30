use pyo3::{Py, PyAny, PyObject, Python, ToPyObject};
use ulid::{DecodeError, Ulid};

pub trait UlidTrait {
    fn to_object(&self, py: Python<'_>) -> PyObject;
    fn gen_ulid_str() -> String;
}

impl UlidTrait for Ulid {
    fn to_object(&self, py: Python<'_>) -> PyObject {
        self.to_object(py)
    }

    fn gen_ulid_str() -> String {
        Ulid::new().to_string()
    }
}