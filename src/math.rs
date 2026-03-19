pub fn eval(expression: &str) -> anyhow::Result<f64> {
    Ok(meval::eval_str(expression)?)
}