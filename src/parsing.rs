use crate::computor::{Equation, Polynomial};

pub fn parsing(raw_equation: String) -> Result<Equation, String> {
    let equation = raw_equation.replace(' ', "");
    let sides: Vec<&str> = equation.split('=').collect();
    if sides.len() != 2 {
        return Err("Invalid equation".to_string());
    }
    let left = parsing_polynomial(sides[0])?;
    let right = parsing_polynomial(sides[1])?;

    Ok(Equation { left, right })
}

fn parsing_polynomial(raw_polynomial: &str) -> Result<Polynomial, String> {
    let values = parse_values(raw_polynomial)?;
    let coefficients = parse_coefficient(&values)?;
    let (a, b, c) = add_coefficients(&coefficients)?;

    Ok(Polynomial::new(a, b, c))
}

fn add_coefficients(coefficients: &Vec<(u8, f64)>) -> Result<(f64, f64, f64), String> {
    let mut a = 0.0;
    let mut b = 0.0;
    let mut c = 0.0;

    for (power, coef) in coefficients {
        match power {
            0 => c += coef,
            1 => b += coef,
            2 => a += coef,
            _ => return Err(format!("Invalid power: {}", power)),
        }
    }
    Ok((a, b, c))
}

fn parse_coefficient(values: &Vec<&str>) -> Result<Vec<(u8, f64)>, String> {
    let mut coefficients: Vec<(u8, f64)> = Vec::new();
    for value in values {
        let mut parts: Vec<&str> = value.split('*').collect();
        let mut x_parts: Vec<&str> = Vec::new();
        let mut i = 0;
        while i < parts.len() {
            if parts[i].contains('X') {
                x_parts.push(parts.remove(i));
            } else {
                i += 1;
            }
        }
        let vec_f64 = vec_str_to_vec_f64(parts)?;
        let coef = vec_f64.iter().fold(1.0, |acc, x| acc * x);
        let power = parse_x_part(&x_parts)?;

        coefficients.push((power, coef));
    }
    Ok(coefficients)
}

fn parse_x_part(x_part: &Vec<&str>) -> Result<u8, String> {
    let mut power = 0;

    for part in x_part {
        if *part == "X" {
            power += 1;
            continue;
        }
        if &part[0..1] != "X" {
            return Err(format!("Invalid part: {}", part));
        }
        if &part[1..2] != "^" {
            return Err(format!("Invalid part: {}", part));
        }
        let power_str = &part[2..];
        match power_str.parse::<u8>() {
            Ok(value) => power += value,
            Err(_) => return Err(format!("Invalid power: {}", part)),
        }
    }

    Ok(power)
}

fn vec_str_to_vec_f64(vec: Vec<&str>) -> Result<Vec<f64>, String> {
    let mut res = Vec::new();
    for value in vec {
        // check err while parsing
        match value.parse::<f64>() {
            Ok(value) => res.push(value),
            Err(_) => return Err(format!("Invalid value: {}", value)),
        }
    }
    Ok(res)
}

/// Function which split the raw polynomial by + and - operators and return a vector of the values
/// Example: "5x^2 + 3x - 2" -> ["5x^2", "3x", "-2"]
fn parse_values(raw_polynomial: &str) -> Result<Vec<&str>, String> {
    let mut values = Vec::new();
    let mut start = 0;
    let mut end = 0;
    while end < raw_polynomial.len() {
        if raw_polynomial.chars().nth(end).unwrap() == '+'
            || raw_polynomial.chars().nth(end).unwrap() == '-'
        {
            if start == end {
                end += 1;
                continue;
            }
            values.push(&raw_polynomial[start..end]);
            start = end;
        }
        end += 1;
    }
    values.push(&raw_polynomial[start..end]);
    check_value_vec(&values)?;
    Ok(values)
}

fn check_value_vec(values: &Vec<&str>) -> Result<(), String> {
    for value in values {
        if (*value).is_empty() {
            return Err("Empty value".to_string());
        }
        if *value == "+" || *value == "-" {
            return Err(format!("Invalid value: {}", value));
        }
    }
    Ok(())
}
