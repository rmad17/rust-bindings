use pyo3::prelude::*;
use std::fs::File;

#[pyfunction]
fn read_csv(filename: &str) -> PyResult<String> {
    use std::time::Instant;
    let now = Instant::now();
    let file = File::open(filename)?;
    let mut rdr = csv::Reader::from_reader(file);

    for result in rdr.records() {
        let record = result?;
    }
    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);

    Ok("Completed!");
}

#[pymodule]
fn rust(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(read_csv, m)?)?;
    Ok(());
}
