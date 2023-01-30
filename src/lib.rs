mod pep3333;

mod server;
mod worker;

use pyo3::prelude::*;

use server::serve;
use worker::Worker;

#[pymodule]
#[pyo3(name = "poney")]
fn import_module(_python: Python, module: &PyModule) -> PyResult<()> {
    module.add_class::<Worker>()?;
    module.add_function(wrap_pyfunction!(serve, module)?)?;
    Ok(())
}
