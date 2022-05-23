use std::any::{Any, TypeId};

use pyo3::prelude::*;
use pyo3::PyTypeInfo;
use pyo3::types::{PyBytes, PyInt, PyLong, PyString};

use fastbloom_rs::{BloomFilter, FilterBuilder};

#[pyclass]
pub struct PyFilterBuilder {
    filter_builder: FilterBuilder,
}

#[pymethods]
impl PyFilterBuilder {
    #[new]
    pub fn __init__(expected_elements: u64, false_positive_probability: f64) -> PyResult<Self> {
        Ok(
            PyFilterBuilder {
                filter_builder: FilterBuilder::new(expected_elements, false_positive_probability)
            }
        )
    }

    pub fn build_bloom_filter(&mut self) -> PyResult<PyBloomFilter> {
        let filter = self.filter_builder.build_bloom_filter();
        Ok(PyBloomFilter { bloomfilter: filter })
    }
}


#[pyclass]
pub struct PyBloomFilter {
    bloomfilter: BloomFilter,
}

#[pymethods]
impl PyBloomFilter {
    pub fn add_int(&mut self, element: i64) {
        self.bloomfilter.add(&i64::to_le_bytes(element));
    }

    pub fn add_str(&mut self, element: &str) {
        self.bloomfilter.add(element.as_bytes());
    }

    pub fn add_bytes(&mut self, bts: &PyBytes) {
        self.bloomfilter.add(bts.as_bytes());
    }

    pub fn contains_int(&mut self, element: i64) -> PyResult<bool> {
        Ok(self.bloomfilter.contains(&i64::to_le_bytes(element)))
    }

    pub fn contains_str(&mut self, element: &str) -> PyResult<bool> {
        Ok(self.bloomfilter.contains(element.as_bytes()))
    }

    pub fn contains_bytes(&self, bts: &PyBytes) -> PyResult<bool> {
        Ok(self.bloomfilter.contains(bts.as_bytes()))
    }
}