mod pep3333;

mod application;
//mod server;
mod http;
mod worker;

use pyo3::prelude::*;

//use server::serve;
use http::run_server;
use tokio::runtime::Runtime;
use worker::Worker;

#[pyfunction]
pub fn serve() -> PyResult<()> {
    let rt = Runtime::new()?;

    rt.block_on(async { run_server().await });

    Ok(())
}

#[pymodule]
#[pyo3(name = "poney")]
fn import_module(_python: Python, module: &PyModule) -> PyResult<()> {
    module.add_class::<Worker>()?;
    module.add_function(wrap_pyfunction!(serve, module)?)?;
    module.add_function(wrap_pyfunction!(application::load_application, module)?)?;
    Ok(())
}
