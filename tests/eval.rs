use std::process::Command;

fn alu(expr: &str) -> String {
    let output = Command::new(env!("CARGO_BIN_EXE_alu"))
        .args(["eval", expr])
        .output()
        .expect("failed to run alu");
    String::from_utf8_lossy(&output.stdout).trim().to_string()
}

fn alu_result(expr: &str) -> f64 {
    let out = alu(expr);
    out.strip_prefix("Result: ")
        .expect("unexpected output format")
        .parse()
        .expect("could not parse result as f64")
}

fn approx_eq(a: f64, b: f64) -> bool {
    (a - b).abs() < 1e-9
}

#[test]
fn test_addition() {
    assert_eq!(alu_result("2 + 3"), 5.0);
}

#[test]
fn test_subtraction() {
    assert_eq!(alu_result("10 - 4"), 6.0);
}

#[test]
fn test_multiplication() {
    assert_eq!(alu_result("3 * 7"), 21.0);
}

#[test]
fn test_division() {
    assert_eq!(alu_result("15 / 4"), 3.75);
}

#[test]
fn test_operator_precedence() {
    assert_eq!(alu_result("2 + 3 * 4"), 14.0);
    assert_eq!(alu_result("(2 + 3) * 4"), 20.0);
}

#[test]
fn test_exponentiation() {
    assert_eq!(alu_result("2^10"), 1024.0);
    assert_eq!(alu_result("9^0.5"), 3.0);
}

#[test]
fn test_sqrt() {
    assert_eq!(alu_result("sqrt(144)"), 12.0);
}

#[test]
fn test_trig() {
    assert!(approx_eq(alu_result("sin(0)"), 0.0));
    assert!(approx_eq(alu_result("sin(pi/2)"), 1.0));
    assert!(approx_eq(alu_result("cos(0)"), 1.0));
    assert!(approx_eq(alu_result("cos(pi)"), -1.0));
    assert!(approx_eq(alu_result("tan(pi/4)"), 1.0));
}

#[test]
fn test_ln() {
    assert!(approx_eq(alu_result("ln(e)"), 1.0));
}

#[test]
fn test_floor_ceil_abs() {
    assert_eq!(alu_result("floor(3.9)"), 3.0);
    assert_eq!(alu_result("ceil(3.1)"), 4.0);
    assert_eq!(alu_result("abs(-42)"), 42.0);
}

#[test]
fn test_constants() {
    assert!(approx_eq(alu_result("pi"), std::f64::consts::PI));
    assert!(approx_eq(alu_result("e"), std::f64::consts::E));
}

#[test]
fn test_complex_expression() {
    assert!(approx_eq(
        alu_result("(45.5 * 1024) / 7.2"),
        6471.111111111111
    ));
}

#[test]
fn test_invalid_expression_exits_nonzero() {
    let output = Command::new(env!("CARGO_BIN_EXE_alu"))
        .args(["eval", "2 +* 3"])
        .output()
        .expect("failed to run alu");
    assert!(!output.status.success());
}
