mod pep3333;

mod application;
mod worker;

use pyo3::prelude::*;

use tokio::runtime::Runtime;
use worker::Worker;


#[pymodule]
#[pyo3(name = "poney")]
fn import_module(_python: Python, module: &PyModule) -> PyResult<()> {
    module.add_class::<Worker>()?;
    Ok(())
}
