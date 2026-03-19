pub fn eval(expression: &str) -> anyhow::Result<f64> {
    Ok(meval::eval_str(expression)?)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn approx_eq(a: f64, b: f64) -> bool {
        (a - b).abs() < 1e-9
    }

    #[test]
    fn test_basic_arithmetic() {
        assert_eq!(eval("2 + 3").unwrap(), 5.0);
        assert_eq!(eval("10 - 4").unwrap(), 6.0);
        assert_eq!(eval("3 * 7").unwrap(), 21.0);
        assert_eq!(eval("15 / 4").unwrap(), 3.75);
    }

    #[test]
    fn test_operator_precedence() {
        assert_eq!(eval("2 + 3 * 4").unwrap(), 14.0);
        assert_eq!(eval("(2 + 3) * 4").unwrap(), 20.0);
    }

    #[test]
    fn test_exponentiation() {
        assert_eq!(eval("2^10").unwrap(), 1024.0);
        assert_eq!(eval("9^0.5").unwrap(), 3.0);
    }

    #[test]
    fn test_trigonometry() {
        assert!(approx_eq(eval("sin(0)").unwrap(), 0.0));
        assert!(approx_eq(eval("cos(0)").unwrap(), 1.0));
        assert!(approx_eq(eval("sin(pi/2)").unwrap(), 1.0));
        assert!(approx_eq(eval("tan(pi/4)").unwrap(), 1.0));
    }

    #[test]
    fn test_sqrt_and_ln() {
        assert_eq!(eval("sqrt(144)").unwrap(), 12.0);
        assert!(approx_eq(eval("ln(e)").unwrap(), 1.0));
    }

    #[test]
    fn test_rounding_and_abs() {
        assert_eq!(eval("floor(3.9)").unwrap(), 3.0);
        assert_eq!(eval("ceil(3.1)").unwrap(), 4.0);
        assert_eq!(eval("abs(-42)").unwrap(), 42.0);
    }

    #[test]
    fn test_constants() {
        assert!(approx_eq(eval("pi").unwrap(), std::f64::consts::PI));
        assert!(approx_eq(eval("e").unwrap(), std::f64::consts::E));
    }

    #[test]
    fn test_invalid_expression() {
        assert!(eval("2 +* 3").is_err());
        assert!(eval("not_a_function(1)").is_err());
    }
}
