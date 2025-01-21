use std::fs;
use std::ffi::CString;

use pyo3::prelude::*;

fn string_to_c_str(s: &String) -> CString {
    CString::new(s.as_str()).unwrap()
}

fn main() -> PyResult<()> {
    pyo3::prepare_freethreaded_python();
    
    let test_dir = concat!(env!("CARGO_MANIFEST_DIR"), "/tests");
    let tests = fs::read_dir(test_dir).unwrap();
    for test in tests {
	let filename = test.unwrap().path().into_os_string().into_string().unwrap();
	if !filename.ends_with(".py") {
	    continue;
	}

	println!("[STARTED]: {}", filename.clone());
	let module_name = filename[..(filename.len()-".py".len())].to_string();
	let source = string_to_c_str(&fs::read_to_string(filename.clone())?);

	Python::with_gil(|py| -> PyResult<()> {
	    PyModule::from_code(
		py,
		&source.as_c_str(),
		&string_to_c_str(&filename).as_c_str(), &string_to_c_str(&module_name).as_c_str())?;
	    Ok(())
	})?;
	println!("[PASSED]: {}", filename.clone());
    }

    Ok(())
}
