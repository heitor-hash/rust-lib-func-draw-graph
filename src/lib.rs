use pyo3::prelude::*;
mod mod1;

#[pyfunction]
pub fn draw_plot_f2deg(
    a: f64,
    b: f64,
    c: f64,
    step: f64,
    x_min: i32,
    x_max: i32,
    y_min: i32,
    y_max: i32,
    res_mult: f64,
) -> PyResult<()> {
    mod1::f2_deg(a, b, c, step, x_min, x_max, y_min, y_max, res_mult);
    Ok(())
}

#[pyfunction]
pub fn draw_plot_f3deg(
    a: f64,
    b: f64,
    c: f64,
    d: f64,
    step: f64,
    x_min: i32,
    x_max: i32,
    y_min: i32,
    y_max: i32,
    res_mult: f64,
) -> PyResult<()> {
    mod1::f3_deg(a, b, c, d, step, x_min, x_max, y_min, y_max, res_mult);
    Ok(())
}

#[pymodule]
fn pyo3_plotter(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(draw_plot_f2deg, m)?)?;
    m.add_function(wrap_pyfunction!(draw_plot_f3deg, m)?)?;
    Ok(())
}
