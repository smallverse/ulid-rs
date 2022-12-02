use pyo3::{Py, PyAny, PyObject, Python};
use ulid::{DecodeError, Ulid};

pub trait UlidTrait {
    fn gen_ulid_str() -> String;
}

impl UlidTrait for Ulid {
    fn gen_ulid_str() -> String {
        Ulid::new().to_string()
    }
}