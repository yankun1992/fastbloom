use pyo3::prelude::*;
use pyo3::types::PyBytes;

use fastbloom_rs::{BloomFilter, CountingBloomFilter, Deletable, FilterBuilder, Hashes, Membership};

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

    pub fn build_counting_bloom_filter(&mut self) -> PyResult<PyCountingBloomFilter> {
        let filter = self.filter_builder.build_counting_bloom_filter();
        Ok(PyCountingBloomFilter { counting_bloom_filter: filter })
    }

    pub fn expected_elements(&self) -> u64 {
        self.filter_builder.expected_elements
    }

    pub fn false_positive_probability(&self) -> f64 {
        self.filter_builder.false_positive_probability
    }

    pub fn enable_repeat_insert(&mut self, enable: bool) {
        self.filter_builder.enable_repeat_insert(enable);
    }

    pub fn size(&self) -> u64 {
        self.filter_builder.size
    }

    pub fn hashes(&self) -> u32 {
        self.filter_builder.hashes
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

    pub fn add_int_if_not_contains(&mut self, element: i64) -> bool {
        self.bloomfilter.add_if_not_contains(&i64::to_le_bytes(element))
    }

    pub fn add_int_batch(&mut self, array: Vec<i64>) {
        for x in array {
            self.add_int(x)
        };
    }

    pub fn add_str(&mut self, element: &str) {
        self.bloomfilter.add(element.as_bytes());
    }

    pub fn add_str_if_not_contains(&mut self, element: &str) -> bool {
        self.bloomfilter.add_if_not_contains(element.as_bytes())
    }

    pub fn add_str_batch(&mut self, array: Vec<&str>) {
        for x in array {
            self.bloomfilter.add(x.as_bytes())
        }
    }

    pub fn add_bytes(&mut self, bts: &PyBytes) {
        self.bloomfilter.add(bts.as_bytes());
    }

    pub fn add_bytes_batch(&mut self, elements: Vec<&PyBytes>) {
        for element in elements {
            self.bloomfilter.add(element.as_bytes())
        }
    }

    pub fn add_bytes_if_not_contains(&mut self, bts: &PyBytes) -> bool {
        self.bloomfilter.add_if_not_contains(bts.as_bytes())
    }

    pub fn contains_int(&mut self, element: i64) -> bool {
        self.bloomfilter.contains(&i64::to_le_bytes(element))
    }

    pub fn contains_int_batch(&mut self, elements: Vec<i64>) -> PyResult<Vec<bool>> {
        let mut res = Vec::<bool>::with_capacity(elements.len());
        for ele in elements {
            let value = self.bloomfilter.contains(&i64::to_le_bytes(ele));
            res.push(value);
        }
        Ok(res)
    }

    pub fn contains_str(&mut self, element: &str) -> bool {
        self.bloomfilter.contains(element.as_bytes())
    }

    pub fn contains_str_batch(&mut self, elements: Vec<&str>) -> PyResult<Vec<bool>> {
        let mut res = Vec::<bool>::with_capacity(elements.len());
        for ele in elements {
            let value = self.bloomfilter.contains(ele.as_bytes());
            res.push(value);
        }
        Ok(res)
    }

    pub fn contains_bytes(&self, bts: &PyBytes) -> bool {
        self.bloomfilter.contains(bts.as_bytes())
    }

    pub fn contains_bytes_batch(&self, elements: Vec<&PyBytes>) -> PyResult<Vec<bool>> {
        let mut res = Vec::<bool>::with_capacity(elements.len());
        for ele in elements {
            let value = self.bloomfilter.contains(ele.as_bytes());
            res.push(value);
        }
        Ok(res)
    }

    pub fn contains_hash_indices(&self, indices: Vec<u64>) -> bool {
        self.bloomfilter.contains_hash_indices(&indices)
    }

    pub fn config(&self) -> PyResult<PyFilterBuilder> {
        Ok(PyFilterBuilder { filter_builder: self.bloomfilter.config() })
    }

    pub fn hashes(&self) -> PyResult<u32> {
        Ok(self.bloomfilter.hashes())
    }

    pub fn get_bytes(&self) -> PyResult<&[u8]> {
        Ok(self.bloomfilter.get_u8_array())
    }

    pub fn get_int_array(&self) -> PyResult<Vec<u32>> {
        Ok(Vec::from(self.bloomfilter.get_u32_array()))
    }

    pub fn clear(&mut self) {
        self.bloomfilter.clear()
    }

    pub fn is_empty(&self) -> PyResult<bool> {
        Ok(self.bloomfilter.is_empty())
    }

    pub fn union(&mut self, other: &PyBloomFilter) -> PyResult<bool> {
        Ok(self.bloomfilter.union(&other.bloomfilter))
    }

    pub fn intersect(&mut self, other: &PyBloomFilter) -> PyResult<bool> {
        Ok(self.bloomfilter.intersect(&other.bloomfilter))
    }

    pub fn get_hash_indices_int(&self, element: i64) -> PyResult<Vec<u64>> {
        Ok(self.bloomfilter.get_hash_indices(&i64::to_le_bytes(element)))
    }

    pub fn get_hash_indices_str(&self, element: &str) -> PyResult<Vec<u64>> {
        Ok(self.bloomfilter.get_hash_indices(element.as_bytes()))
    }

    pub fn get_hash_indices(&self, bts: &PyBytes) -> PyResult<Vec<u64>> {
        Ok(self.bloomfilter.get_hash_indices(bts.as_bytes()))
    }


    #[staticmethod]
    pub fn from_bytes(array: &[u8], hashes: u32) -> PyResult<Self> {
        Ok(PyBloomFilter { bloomfilter: BloomFilter::from_u8_array(array, hashes) })
    }

    #[staticmethod]
    pub fn from_int_array(array: Vec<u32>, hashes: u32) -> PyResult<Self> {
        Ok(PyBloomFilter { bloomfilter: BloomFilter::from_u32_array(array.as_slice(), hashes) })
    }
}

#[pyclass]
pub struct PyCountingBloomFilter {
    counting_bloom_filter: CountingBloomFilter,
}

#[pymethods]
impl PyCountingBloomFilter {
    pub fn add_int(&mut self, element: i64) {
        self.counting_bloom_filter.add(&i64::to_le_bytes(element));
    }

    pub fn add_int_batch(&mut self, array: Vec<i64>) {
        for x in array {
            self.add_int(x)
        };
    }

    pub fn remove_int(&mut self, element: i64) {
        self.counting_bloom_filter.remove(&i64::to_le_bytes(element));
    }

    pub fn add_str(&mut self, element: &str) {
        self.counting_bloom_filter.add(element.as_bytes());
    }

    pub fn add_str_batch(&mut self, array: Vec<&str>) {
        for x in array {
            self.counting_bloom_filter.add(x.as_bytes())
        }
    }

    pub fn remove_str(&mut self, element: &str) {
        self.counting_bloom_filter.remove(element.as_bytes());
    }

    pub fn add_bytes(&mut self, bts: &PyBytes) {
        self.counting_bloom_filter.add(bts.as_bytes());
    }

    pub fn add_bytes_batch(&mut self, elements: Vec<&PyBytes>) {
        for element in elements {
            self.counting_bloom_filter.add(element.as_bytes())
        }
    }

    pub fn remove_bytes(&mut self, bts: &PyBytes) {
        self.counting_bloom_filter.remove(bts.as_bytes());
    }

    pub fn contains_int(&mut self, element: i64) -> bool {
        self.counting_bloom_filter.contains(&i64::to_le_bytes(element))
    }

    pub fn contains_int_batch(&mut self, elements: Vec<i64>) -> PyResult<Vec<bool>> {
        let mut res = Vec::<bool>::with_capacity(elements.len());
        for ele in elements {
            res.push(self.counting_bloom_filter.contains(&i64::to_le_bytes(ele)));
        }
        Ok(res)
    }

    pub fn contains_str(&mut self, element: &str) -> bool {
        self.counting_bloom_filter.contains(element.as_bytes())
    }

    pub fn contains_str_batch(&mut self, elements: Vec<&str>) -> PyResult<Vec<bool>> {
        let mut res = Vec::<bool>::with_capacity(elements.len());
        for ele in elements {
            res.push(self.counting_bloom_filter.contains(ele.as_bytes()));
        }
        Ok(res)
    }

    pub fn contains_bytes(&self, bts: &PyBytes) -> bool {
        self.counting_bloom_filter.contains(bts.as_bytes())
    }

    pub fn contains_bytes_batch(&self, elements: Vec<&PyBytes>) -> PyResult<Vec<bool>> {
        let mut res = Vec::<bool>::with_capacity(elements.len());
        for ele in elements {
            res.push(self.counting_bloom_filter.contains(ele.as_bytes()));
        }
        Ok(res)
    }

    pub fn contains_hash_indices(&self, indices: Vec<u64>) -> bool {
        self.counting_bloom_filter.contains_hash_indices(&indices)
    }

    pub fn config(&self) -> PyResult<PyFilterBuilder> {
        Ok(PyFilterBuilder { filter_builder: self.counting_bloom_filter.config() })
    }

    pub fn hashes(&self) -> PyResult<u32> {
        Ok(self.counting_bloom_filter.hashes())
    }

    pub fn get_bytes(&self) -> PyResult<&[u8]> {
        Ok(self.counting_bloom_filter.get_u8_array())
    }

    pub fn get_int_array(&self) -> PyResult<Vec<u32>> {
        Ok(Vec::from(self.counting_bloom_filter.get_u32_array()))
    }

    pub fn clear(&mut self) {
        self.counting_bloom_filter.clear()
    }

    pub fn get_hash_indices_int(&self, element: i64) -> PyResult<Vec<u64>> {
        Ok(self.counting_bloom_filter.get_hash_indices(&i64::to_le_bytes(element)))
    }

    pub fn get_hash_indices_str(&self, element: &str) -> PyResult<Vec<u64>> {
        Ok(self.counting_bloom_filter.get_hash_indices(element.as_bytes()))
    }

    pub fn get_hash_indices(&self, bts: &PyBytes) -> PyResult<Vec<u64>> {
        Ok(self.counting_bloom_filter.get_hash_indices(bts.as_bytes()))
    }

    pub fn estimate_count_int(&self, element: i64) -> PyResult<u32> {
        Ok(self.counting_bloom_filter.estimate_count(&i64::to_le_bytes(element)) as u32)
    }

    pub fn estimate_count_str(&self, element: &str) -> PyResult<u32> {
        Ok(self.counting_bloom_filter.estimate_count(element.as_bytes()) as u32)
    }

    pub fn estimate_count(&self, element: &PyBytes) -> PyResult<u32> {
        Ok(self.counting_bloom_filter.estimate_count(element.as_bytes()) as u32)
    }

    pub fn counter_at(&self, index: i64) -> PyResult<u64> {
        Ok(self.counting_bloom_filter.counter_at(index as u64) as u64)
    }

    #[staticmethod]
    pub fn from_bytes(array: &[u8], hashes: u32, enable_repeat_insert: bool) -> PyResult<Self> {
        Ok(PyCountingBloomFilter {
            counting_bloom_filter: CountingBloomFilter::from_u8_array(array, hashes, enable_repeat_insert)
        })
    }

    #[staticmethod]
    pub fn from_int_array(array: Vec<u32>, hashes: u32, enable_repeat_insert: bool) -> PyResult<Self> {
        Ok(PyCountingBloomFilter {
            counting_bloom_filter:
            CountingBloomFilter::from_u32_array(array.as_slice(), hashes, enable_repeat_insert)
        })
    }
}


