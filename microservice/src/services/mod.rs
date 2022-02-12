use cached::proc_macro::cached;
use pyo3::prelude::*;
use pyo3::types::{PyFunction, PyTuple};
use pyo3::{Py, PyAny};

pub type CharsToJyutpingResult = Vec<(String, String)>;

// Use String instead of &str in args to fulfill cached's requirement
#[cached(size = 250)]
pub fn chars_to_jyutping(text: String) -> CharsToJyutpingResult {
    Python::with_gil(|py| -> PyResult<CharsToJyutpingResult> {
        let pycantonese = py.import("pycantonese").unwrap();
        let py_chars_to_jyutping: Py<PyFunction> =
            pycantonese.getattr("characters_to_jyutping")?.extract()?;
        let args = (text.to_owned(),);
        let results: CharsToJyutpingResult = py_chars_to_jyutping.call1(py, args)?.extract(py)?;

        Ok(results)
    })
    .unwrap()
}

pub fn jyutping_to_yale(text: String) -> String {
    text
}

pub fn chars_to_yale(text: String) -> CharsToJyutpingResult {
    chars_to_jyutping(text)
    // jyutping_to_yale(jyutping)
}

pub type SegmentResult = Vec<String>;

#[cached(size = 250)]
pub fn segment_chars(text: String) -> SegmentResult {
    Python::with_gil(|py| -> PyResult<SegmentResult> {
        let pycantonese = py.import("pycantonese").unwrap();
        let py_chars_to_jyutping: Py<PyFunction> = pycantonese.getattr("segment")?.extract()?;
        let args = (text.to_owned(),);
        let results: SegmentResult = py_chars_to_jyutping.call1(py, args)?.extract(py)?;

        Ok(results)
    })
    .unwrap()
}
