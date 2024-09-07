use anyhow::Result;
use pyo3::prelude::*;
use pyo3::PyResult;

pub fn test_run((year, quarter): (i32, i32)) -> Result<()> {
    pyo3::prepare_freethreaded_python();

    Python::with_gil(|py| -> PyResult<()> {
        let sys = py.import_bound("sys")?;

        let path = sys.getattr("path")?;

        path.call_method1(
            "append",
            ("/home/jon/Programming/Rust/learning/scraping/finance_scraper/python/src/scripts",),
        )?;

        let module = PyModule::import_bound(py, "main")?;
        let args = (year, quarter);
        let result = module.getattr("scrape")?.call1(args)?;

        println!("scraping finished! {}", result);

        Ok(())
    })
    .unwrap();

    Ok(())
}
