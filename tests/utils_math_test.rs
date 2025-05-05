use bash_and_blade::utils::math;

#[test]
fn test_integration_add() {
    assert_eq!(math::add(10, 5), 15);
    assert_eq!(math::add(-10, 5), -5);
}

#[test]
fn test_integration_subtract() {
    assert_eq!(math::subtract(10, 5), 5);
    assert_eq!(math::subtract(5, 10), -5);
}

#[test]
fn test_integration_multiply() {
    assert_eq!(math::multiply(10, 5), 50);
    assert_eq!(math::multiply(10, 0), 0);
}

#[test]
fn test_integration_divide() {
    assert_eq!(math::divide(10, 5), Ok(2));
    assert_eq!(
        math::divide(10, 0),
        Err("Cannot divide by zero".to_string())
    );
}
