use crate::ffi::from_python::utils::call_arrow_c_stream;
use crate::record_batch_reader::PyRecordBatchReader;
use pyo3::prelude::*;
use pyo3::{PyAny, PyResult};

impl<'a> FromPyObject<'a> for PyRecordBatchReader {
    fn extract_bound(ob: &Bound<'a, PyAny>) -> PyResult<Self> {
        let capsule = call_arrow_c_stream(ob)?;
        Python::with_gil(|py| {
            Self::from_arrow_pycapsule(&py.get_type_bound::<PyRecordBatchReader>(), &capsule)
        })
    }
}
