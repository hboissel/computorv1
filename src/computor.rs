pub struct Polynomial {
    pub a: f64,
    pub b: f64,
    pub c: f64,
}

pub struct Equation {
    pub left: Polynomial,
    pub right: Polynomial,
}

impl Polynomial {
    pub fn new(a: f64, b: f64, c: f64) -> Polynomial {
        Polynomial { a, b, c }
    }

    pub fn print(&self) {
        println!("{}x^2 + {}x + {}", self.a, self.b, self.c);
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

    pub fn solve(&self) {
        let d = self.discriminant();
        if d > 0.0 {
            let x1 = (-self.b + d.sqrt()) / (2.0 * self.a);
            let x2 = (-self.b - d.sqrt()) / (2.0 * self.a);
            println!("Discriminant is strictly positive, the two solutions are:");
            println!("x1 = {}", x1);
            println!("x2 = {}", x2);
        } else if d == 0.0 {
            let x = -self.b / (2.0 * self.a);
            println!("Discriminant is zero, the solution is:");
            println!("x = {}", x);
        } else {
            let real = -self.b / (2.0 * self.a);
            let imaginary = (-d).sqrt() / (2.0 * self.a);
            println!("Discriminant is strictly negative, the two solutions are:");
            println!("x1 = {} + {}i", real, imaginary);
            println!("x2 = {} - {}i", real, imaginary);
        }
    }
}