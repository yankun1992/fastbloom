use pyo3::prelude::*;

use crate::pybloom::{PyBloomFilter, PyFilterBuilder};

pub mod pybloom;

#[pymodule]
fn fastbloom_rs(_py: Python<'_>, m: &PyModule) -> PyResult<()> {
    m.add_class::<PyBloomFilter>().unwrap();
    m.add_class::<PyFilterBuilder>().unwrap();
    Ok(())
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
