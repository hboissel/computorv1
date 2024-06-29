import numpy as np
from sympy import symbols, Eq, solve

# Define the variable
X = symbols('X')

def generate_random_coefficients():
    # Generate random coefficients a, b, c, d, e, f
    coefficients = np.random.uniform(-200, 200, 6)
    return coefficients

def create_equation_and_solve():
    a, b, c, d, e, f = generate_random_coefficients()

    # print the equation in the correct format: a*X^2 + b*X + c = d*X^2 + e*X + f
    # if negative value, dont print the + sign
    equation = f"{a}*X^2"
    equation += f" + {b}*X" if b >= 0 else f" {b}*X"
    equation += f" + {c}" if c >= 0 else f" {c}"
    equation += f" = {d}*X^2"
    equation += f" + {e}*X" if e >= 0 else f" {e}*X"
    equation += f" + {f}" if f >= 0 else f" {f}"
    print(f"Equation: {equation}")
    
    # Create the equation: a*X^2 + b*X + c = d*X^2 + e*X + f
    lhs = a*X**2 + b*X + c
    rhs = d*X**2 + e*X + f
    
    # Simplify to: (a-d)*X^2 + (b-e)*X + (c-f) = 0
    equation = Eq(lhs, rhs)
    simplified_equation = Eq((a - d)*X**2 + (b - e)*X + (c - f), 0)
    
    # Solve the equation
    solutions = solve(simplified_equation, X)
    
    return lhs, rhs, simplified_equation, solutions

if __name__ == "__main__":
    lhs, rhs, simplified_equation, solutions = create_equation_and_solve()
    print(f"Solutions: {solutions}")
