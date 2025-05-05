// src/utils/math.rs

pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

pub fn subtract(a: i32, b: i32) -> i32 {
    a - b
}

pub fn multiply(a: i32, b: i32) -> i32 {
    a * b
}

pub fn divide(a: i32, b: i32) -> Result<i32, String> {
    if b == 0 {
        Err("Cannot divide by zero".to_string())
    } else {
        Ok(a / b)
    }
}

// == Tests ==
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        assert_eq!(add(2, 3), 5);
        assert_eq!(add(-1, -1), -2);
        assert_eq!(add(0, 0), 0);
    }

    #[test]
    fn test_subtract() {
        assert_eq!(subtract(5, 3), 2);
        assert_eq!(subtract(0, 5), -5);
        assert_eq!(subtract(-3, -2), -1);
    }

    #[test]
    fn test_multiply() {
        assert_eq!(multiply(2, 3), 6);
        assert_eq!(multiply(-1, 5), -5);
        assert_eq!(multiply(0, 10), 0);
    }

    #[test]
    fn test_divide() {
        assert_eq!(divide(6, 3), Ok(2));
        assert_eq!(divide(5, 0), Err("Cannot divide by zero".to_string()));
        assert_eq!(divide(-6, -3), Ok(2));
    }
}
