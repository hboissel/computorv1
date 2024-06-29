#[derive(Debug)]
pub struct Polynomial {
    pub a: f64,
    pub b: f64,
    pub c: f64,
}

#[derive(Debug)]
pub struct Equation {
    pub left: Polynomial,
    pub right: Polynomial,
}

impl Polynomial {
    pub fn new(a: f64, b: f64, c: f64) -> Polynomial {
        Polynomial { a, b, c }
    }

    pub fn is_zero(&self) -> bool {
        self.a == 0.0 && self.b == 0.0 && self.c == 0.0
    }

    pub fn print(&self) {
        println!("{}x^2 + {}x + {} = 0", self.a, self.b, self.c);
    }

    pub fn degree(&self) -> i32 {
        if self.a != 0.0 {
            2
        } else if self.b != 0.0 {
            1
        } else {
            0
        }
    }

    pub fn minus(&mut self, other: &Polynomial) {
        self.a -= other.a;
        self.b -= other.b;
        self.c -= other.c;
    }

    pub fn discriminant(&self) -> f64 {
        (self.b * self.b) - 4.0 * self.a * self.c
    }

    pub fn solve(&self) -> Option<Vec<f64>> {
        match self.degree() {
            2 => self.solve_2e(),
            1 => self.solve_1e(),
            0 => self.solve_0e(),
            _ => {
                println!("The polynomial degree is stricly greater than 2, I can't solve.");
                None
            }
        }
    }

    pub fn solve_0e(&self) -> Option<Vec<f64>> {
        if self.c == 0.0 {
            println!("The solution is:");
            println!("x = 0");
            Some(vec![0.0])
        } else {
            println!("There is no solution.");
            None
        }
    }

    pub fn solve_1e(&self) -> Option<Vec<f64>> {
        let x = -self.c / self.b;
        println!("The solution is:");
        println!("x = {}", x);
        Some(vec![x])
    }

    pub fn solve_2e(&self) -> Option<Vec<f64>> {
        let d = self.discriminant();
        if d > 0.0 {
            let x1 = (-self.b + d.sqrt()) / (2.0 * self.a);
            let x2 = (-self.b - d.sqrt()) / (2.0 * self.a);
            println!("Discriminant is strictly positive, the two solutions are:");
            println!("x1 = {}", x1);
            println!("x2 = {}", x2);
            Some(vec![x1, x2])
        } else if d == 0.0 {
            let x = -self.b / (2.0 * self.a);
            println!("Discriminant is zero, the solution is:");
            println!("x = {}", x);
            Some(vec![x])
        } else {
            let real = -self.b / (2.0 * self.a);
            let imaginary = (-d).sqrt() / (2.0 * self.a);
            println!("Discriminant is strictly negative, the two solutions are:");
            println!("x1 = {} + {}i", real, imaginary);
            println!("x2 = {} - {}i", real, imaginary);
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_polynomial_new() {
        let poly = Polynomial::new(1.0, -3.0, 2.0);
        assert_eq!(poly.a, 1.0);
        assert_eq!(poly.b, -3.0);
        assert_eq!(poly.c, 2.0);
    }

    #[test]
    fn test_is_zero_true() {
        let poly = Polynomial::new(0.0, 0.0, 0.0);
        assert!(poly.is_zero());
    }

    #[test]
    fn test_is_zero_false() {
        let poly = Polynomial::new(1.0, 0.0, 0.0);
        assert!(!poly.is_zero());
    }

    #[test]
    fn test_degree_2() {
        let poly = Polynomial::new(1.0, 0.0, 0.0);
        assert_eq!(poly.degree(), 2);
    }

    #[test]
    fn test_degree_1() {
        let poly = Polynomial::new(0.0, 1.0, 0.0);
        assert_eq!(poly.degree(), 1);
    }

    #[test]
    fn test_degree_0() {
        let poly = Polynomial::new(0.0, 0.0, 1.0);
        assert_eq!(poly.degree(), 0);
    }

    #[test]
    fn test_minus() {
        let mut poly1 = Polynomial::new(3.0, -2.0, 1.0);
        let poly2 = Polynomial::new(1.0, -1.0, 1.0);
        poly1.minus(&poly2);
        assert_eq!(poly1.a, 2.0);
        assert_eq!(poly1.b, -1.0);
        assert_eq!(poly1.c, 0.0);
    }

    #[test]
    fn test_discriminant_positive() {
        let poly = Polynomial::new(1.0, -3.0, 2.0);
        assert!(poly.discriminant() > 0.0);
    }

    #[test]
    fn test_discriminant_zero() {
        let poly = Polynomial::new(1.0, -2.0, 1.0);
        assert_eq!(poly.discriminant(), 0.0);
    }

    #[test]
    fn test_discriminant_negative() {
        let poly = Polynomial::new(1.0, 2.0, 3.0);
        assert!(poly.discriminant() < 0.0);
    }

    #[test]
    fn test_solve_2e_positive_discriminant() {
        let poly = Polynomial::new(1.0, -3.0, 2.0);
        let solutions = poly.solve_2e().unwrap();
        assert_eq!(solutions.len(), 2);
        assert!((solutions[0] - 2.0).abs() < f64::EPSILON);
        assert!((solutions[1] - 1.0).abs() < f64::EPSILON);
    }

    #[test]
    fn test_solve_2e_zero_discriminant() {
        let poly = Polynomial::new(1.0, -2.0, 1.0);
        let solutions = poly.solve_2e().unwrap();
        assert_eq!(solutions.len(), 1);
        assert!((solutions[0] - 1.0).abs() < f64::EPSILON);
    }

    #[test]
    fn test_solve_2e_negative_discriminant() {
        let poly = Polynomial::new(1.0, 2.0, 3.0);
        assert!(poly.solve_2e().is_none());
    }

    #[test]
    fn test_solve_1e() {
        let poly = Polynomial::new(0.0, 2.0, -4.0);
        let solutions = poly.solve_1e().unwrap();
        assert_eq!(solutions.len(), 1);
        assert!((solutions[0] - 2.0).abs() < f64::EPSILON);
    }

    #[test]
    fn test_solve_0e_no_solution() {
        let poly = Polynomial::new(0.0, 0.0, 1.0);
        assert!(poly.solve_0e().is_none());
    }

    #[test]
    fn test_solve_0e_solution() {
        let poly = Polynomial::new(0.0, 0.0, 0.0);
        let solutions = poly.solve_0e().unwrap();
        assert_eq!(solutions.len(), 1);
        assert_eq!(solutions[0], 0.0);
    }
}
