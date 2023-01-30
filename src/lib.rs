mod pep3333;

mod server;
mod worker;
mod application;

use pyo3::prelude::*;

use server::serve;
use worker::Worker;

#[pymodule]
#[pyo3(name = "poney")]
fn import_module(_python: Python, module: &PyModule) -> PyResult<()> {
    module.add_class::<Worker>()?;
    module.add_function(wrap_pyfunction!(serve, module)?)?;
    module.add_function(wrap_pyfunction!(application::load_application, module)?)?;
    Ok(())
}
