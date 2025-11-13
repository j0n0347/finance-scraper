use anyhow::Result;
use pyo3::prelude::*;
use pyo3::PyResult;
use std::env;
use std::path::Path;

pub fn run((year, quarter): (i32, i32)) -> Result<()> {
    let project_root = Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("Failed to get project root directory");


    let script_path = project_root
        .join("python")
        .join("src")
        .join("scripts");

    println!("Script path: {}", script_path.display());
    

    pyo3::prepare_freethreaded_python();

    Python::with_gil(|py| -> PyResult<()> {
        let sys = py.import_bound("sys")?;

        let path = sys.getattr("path")?;

        path.call_method1("append", (script_path,))?;

        let module = PyModule::import_bound(py, "main")?;
        let args = (year, quarter);
        let result = module.getattr("scrape")?.call1(args)?;

        println!("scraping finished! {}", result);

        Ok(())
    })?;

    Ok(())
}
