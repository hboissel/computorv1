mod computor;
mod parsing;

use parsing::parsing;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        println!("Usage: ./computorv1 \"[equation]\"");
        std::process::exit(1);
    }
    let _ = compute(args[1].to_string());
}

fn compute(str_equation: String) -> Result<Option<Vec<f64>>, ()> {
    let mut equation = match parsing(str_equation) {
        Ok(equation) => equation,
        Err(err) => {
            println!("{}", err);
            return Err(());
        }
    };
    if equation.left.degree() == equation.right.degree() {
        equation.left.minus(&equation.right);
        if equation.left.is_zero() {
            println!("All real numbers are solution");
            return Ok(None);
        }
    } else {
        equation.left.minus(&equation.right);
    }
    print!("Reduced form: ");
    equation.left.print();
    println!("Degree: {}", equation.left.degree());
    Ok(equation.left.solve())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compute_all_real_numbers() {
        let equation = "0 * X^0 = 0 * X^0".to_string();
        let result = compute(equation);
        assert_eq!(result, Ok(None));
    }

    #[test]
    fn test_compute_no_solution() {
        let equation = "0 * X^0 = 1 * X^0".to_string();
        let result = compute(equation);
        assert!(result.unwrap().is_none());
    }

    #[test]
    fn test_compute_linear_solution() {
        let equation = "1 * X^1 = 0 * X^0".to_string();
        let result = compute(equation);
        assert_eq!(result, Ok(Some(vec![0.0])));
    }

    #[test]
    fn test_compute_quadratic_positive_discriminant() {
        let equation = "1 * X^2 + 0 * X^1 - 1 * X^0 = 0 * X^0".to_string();
        let result = compute(equation);
        assert!(result.is_ok());
        if let Ok(Some(roots)) = result {
            assert_eq!(roots.len(), 2);
            assert!(roots.contains(&1.0));
            assert!(roots.contains(&-1.0));
        } else {
            panic!("Expected Ok(Some(Vec)) but got {:?}", result);
        }
    }

    #[test]
    fn test_compute_quadratic_positive_discriminant2() {
        let equation = "1*X^2 + 5*X + 6 = 0".to_string();
        let result = compute(equation);
        assert!(result.is_ok());
        if let Ok(Some(roots)) = result {
            assert_eq!(roots.len(), 2);
            assert!(roots.contains(&-2.0));
            assert!(roots.contains(&-3.0));
        } else {
            panic!("Expected Ok(Some(Vec)) but got {:?}", result);
        }
    }

    #[test]
    fn test_compute_quadratic_zero_discriminant() {
        let equation = "1 * X^2 - 2 * X^1 + 1 * X^0 = 0 * X^0".to_string();
        let result = compute(equation);
        assert_eq!(result, Ok(Some(vec![1.0])));
    }

    #[test]
    fn test_compute_quadratic_negative_discriminant() {
        let equation = "1 * X^2 + 2 * X^1 + 5 * X^0 = 0 * X^0".to_string();
        let result = compute(equation);
        assert!(result.unwrap().is_none());
    }

    #[test]
    fn test_compute_invalid_input() {
        let equation = "This is not a valid equation".to_string();
        let result = compute(equation);
        assert!(result.is_err());
    }

    #[test]
    fn test_compute_higher_degree() {
        let equation = "1 * X^3 - 3 * X^2 + 3 * X^1 - 1 * X^0 = 0 * X^0".to_string();
        let result = compute(equation);
        assert!(result.is_err());
    }
}
