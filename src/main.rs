mod computor;

use core::num;
use std::env;
use computor::{Equation, Polynomial};


fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        println!("Usage: ./computorv1 \"[equation]\"");
        std::process::exit(1);
    }
    let equation = parsing(args[1].to_string());
    let equation = match equation {
        Ok(equation) => {
            equation
        }
        Err(err) => {
            println!("{}", err);
            std::process::exit(1);
        }
    };
}

fn parsing(raw_equation: String) -> Result<Equation, String> {
    let equation = raw_equation.replace(" ", "");
    let sides: Vec<&str> = equation.split('=').collect();
    if sides.len() != 2 {
        return Err("Invalid equation".to_string());
    }
    let left = parsing_polynomial(sides[0]);
    if left.is_err() {
        return Err(left.err().unwrap());
    }
    let left = left.unwrap();
    let right = parsing_polynomial(sides[1]);
    if right.is_err() {
        return Err(right.err().unwrap());
    }
    let right = right.unwrap();

    Ok(Equation { left, right })
}

fn parsing_polynomial(raw_polynomial: &str) -> Result<Polynomial, String> {
    let mut a = 0.0;
    let mut b = 0.0;
    let mut c = 0.0;

    let values = parse_value(raw_polynomial);

    Ok(Polynomial::new(a, b, c))
}

/// Function which split the raw polynomial by + and - operators and return a vector of the values
/// Example: "5x^2 + 3x - 2" -> ["5x^2", "3x", "-2"]
fn parse_value(raw_polynomial: &str) -> Vec<&str> {
    let mut values = Vec::new();
    let mut start = 0;
    let mut end = 0;
    while end < raw_polynomial.len() {
        if raw_polynomial.chars().nth(end).unwrap() == '+' || raw_polynomial.chars().nth(end).unwrap() == '-' {
            values.push(&raw_polynomial[start..end]);
            start = end;
        }
        end += 1;
    }
    values.push(&raw_polynomial[start..end]);
    values
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_value() {
        assert_eq!(parse_value("5x^2+3x-2"), vec!["5x^2", "3x", "-2"]);
        assert_eq!(parse_value("5x^2+3x+2"), vec!["5x^2", "3x ", "2"]);
        assert_eq!(parse_value("-5x^2-3x-2"), vec!["-5x^2", "-3x", "-2"]);
        assert_eq!(parse_value("5x^2"), vec!["5x^2"]);
        assert_eq!(parse_value(""), vec![""]);
    }
}

