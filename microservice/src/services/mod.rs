use pyo3::prelude::*;
// use pyo3::types::IntoPyDict;

pub fn chars_to_jyutping(text: String) -> String {
    Python::with_gil(|py| -> PyResult<()> {
        let pycantonese = py.import("pycantonese").unwrap();
        Ok(())
    });
    text
}

pub fn jyutping_to_yale(text: String) -> String {
    text
}

pub fn chars_to_yale(text: String) -> String {
    let jyutping = chars_to_jyutping(text);
    jyutping_to_yale(jyutping)
}
