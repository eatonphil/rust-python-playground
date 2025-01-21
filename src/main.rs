use std::collections::HashMap;
use std::ffi::CString;
use std::fs;

use pyo3::prelude::*;
use pyo3::IntoPyObjectExt;

fn string_to_c_str(s: &String) -> CString {
    CString::new(s.as_str()).unwrap()
}

#[pyfunction]
fn sql(_sql_string: String) -> Vec<HashMap<String, String>> {
    vec![
        HashMap::from([
            ("name".to_string(), "Terry".to_string()),
            ("id".to_string(), "1".to_string()),
        ]),
        HashMap::from([
            ("name".to_string(), "Bina".to_string()),
            ("id".to_string(), "2".to_string()),
        ]),
    ]
}

#[pyfunction]
fn sql_dynamic(_sql_string: String) -> PyResult<PyObject> {
    Python::with_gil(|py| -> PyResult<PyObject> {
        vec![
            HashMap::from([
                ("name".to_string(), "Terry".into_py_any(py)?),
                ("id".to_string(), 1_i32.into_py_any(py)?),
            ]),
            HashMap::from([
                ("name".to_string(), "Bina".into_py_any(py)?),
                ("id".to_string(), 2_i32.into_py_any(py)?),
            ]),
        ]
        .into_py_any(py)
    })
}

#[pymodule]
fn dbcluster(dbcluster_module: &Bound<'_, PyModule>) -> PyResult<()> {
    dbcluster_module.add_function(wrap_pyfunction!(sql, dbcluster_module)?)?;
    dbcluster_module.add_function(wrap_pyfunction!(sql_dynamic, dbcluster_module)?)
}

fn main() -> PyResult<()> {
    pyo3::append_to_inittab!(dbcluster);
    pyo3::prepare_freethreaded_python();

    let test_dir = concat!(env!("CARGO_MANIFEST_DIR"), "/tests");
    let tests = fs::read_dir(test_dir).unwrap();
    for test in tests {
        let filename = test.unwrap().path().into_os_string().into_string().unwrap();
        if !filename.ends_with(".py") {
            continue;
        }

        println!("[STARTED]: {}", filename.clone());
        let module_name = filename[..(filename.len() - ".py".len())].to_string();
        let source = string_to_c_str(&fs::read_to_string(filename.clone())?);

        Python::with_gil(|py| -> PyResult<()> {
            PyModule::from_code(
                py,
                &source.as_c_str(),
                &string_to_c_str(&filename).as_c_str(),
                &string_to_c_str(&module_name).as_c_str(),
            )?;
            Ok(())
        })?;
        println!("[PASSED]: {}", filename.clone());
    }

    Ok(())
}
