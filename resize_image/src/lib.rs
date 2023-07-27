use pyo3::prelude::*;
use pyo3::types::PyList;

#[pyfunction]
fn process_image(input_image: Vec<Vec<u8>>) -> Vec<Vec<u8>> {
    // Perform any image processing or manipulation here
    // For this example, we'll return the same input_image
    input_image
}

#[pymodule]
fn my_image_resize_crate(py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(process_image, m)?)?;
    Ok(())
}