use cpython::{py_fn, PyResult, Python};

// Замените "libmyrustlib.so" на имя вашей библиотеки
const RUST_LIB_PATH: &str = "libmyrustlib.so";

py_module_initializer!(rust_tts_module, initrust_tts_module, PyInit_rust_tts_module, |py, m| {
    m.add(py, "__doc__", "Module docstring")?;
    m.add(py, "speak", py_fn!(py, speak(words: &str)))?;
    Ok(())
});

fn speak(_: Python, words: &str) -> PyResult<String> {
    unsafe {
        let lib = libc::dlopen(RUST_LIB_PATH, libc::RTLD_NOW);
        if lib.is_null() {
            return Err(cpython::PyErr::new::<cpython::exc::ValueError, _>(
                py,
                "Failed to load Rust library",
            ));
        }

        let speak_fn: Symbol<extern "C" fn(*const libc::c_char) -> *const libc::c_char> =
            libc::dlsym(lib, "speak");

        if speak_fn.is_null() {
            return Err(cpython::PyErr::new::<cpython::exc::ValueError, _>(
                py,
                "Failed to find speak function in Rust library",
            ));
        }

        let c_words = CString::new(words)?;

        let result = speak_fn(c_words.as_ptr());

        if result.is_null() {
            return Err(cpython::PyErr::new::<cpython::exc::ValueError, _>(
                py,
                "Error calling speak function",
            ));
        }

        let result_str = CStr::from_ptr(result).to_str()?.to_string();

        Ok(result_str)
    }
}
