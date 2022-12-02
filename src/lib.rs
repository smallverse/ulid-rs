use pyo3::prelude::*;
use ulid::Ulid;

use crate::ulid_trait::UlidTrait;

mod ulid_trait;


#[pyfunction]
pub fn gen_ulid_str() -> String {
    return Ulid::gen_ulid_str();
}


#[pymodule]
fn ulid_rs(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(gen_ulid_str, m)?)?;

    Ok(())
}

fn test_ulid() {

    // Generate a string for a ulid
    let s = gen_ulid_str();
    println!("{}", s);


}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        test_ulid()
        // assert_eq!(result, 4);
    }
}
