// https://www.codewars.com/kata/symbolic-differentiation-of-prefix-expressions/train/rust
// I struggled on this kata for such a long time because of a small pitfall.

use std::ops;
use std::fmt::Display;
use std::str::FromStr;

#[derive(Copy, Clone)]
enum UnaryOp {
    Sine,
    Cosine,
    Tangent,
    Exponent,
    Logarithm,
}

impl UnaryOp {
    fn eval(&self, x: f64) -> f64 {
        match self {
            UnaryOp::Sine => x.sin(),
            UnaryOp::Cosine => x.cos(),
            UnaryOp::Tangent => x.tan(),
            UnaryOp::Exponent => x.exp(),
            UnaryOp::Logarithm => x.ln(),
        }
    }
}

impl Display for UnaryOp {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", match self {
            UnaryOp::Sine => "sin",
            UnaryOp::Cosine => "cos",
            UnaryOp::Tangent => "tan",
            UnaryOp::Exponent => "exp",
            UnaryOp::Logarithm => "ln",
        })
    }
}

#[derive(Copy, Clone)]
enum BinaryOp {
    Add,
    Sub,
    Mul,
    Div,
    Pow,
}

impl BinaryOp {
    fn eval(&self, x: f64, y: f64) -> f64 {
        match self {
            BinaryOp::Add => x + y,
            BinaryOp::Sub => x - y,
            BinaryOp::Mul => x * y,
            BinaryOp::Div => x / y,
            BinaryOp::Pow => x.powf(y),
        }
    }
}

impl Display for BinaryOp {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", match self {
            BinaryOp::Add => '+',
            BinaryOp::Sub => '-',
            BinaryOp::Mul => '*',
            BinaryOp::Div => '/',
            BinaryOp::Pow => '^',
        })
    }
}

#[derive(Clone)]
enum Expr {
    Number(f64),
    Variable(char),
    Unary(UnaryOp, Box<Expr>),
    Binary(BinaryOp, Box<Expr>, Box<Expr>),
}

impl ops::Add<Expr> for Expr {
    type Output = Expr;

    fn add(self, rhs: Expr) -> Self::Output {
        Expr::Binary(BinaryOp::Add, Box::new(self), Box::new(rhs))
    }
}

impl ops::Sub<Expr> for Expr {
    type Output = Expr;

    fn sub(self, rhs: Expr) -> Self::Output {
        Expr::Binary(BinaryOp::Sub, Box::new(self), Box::new(rhs))
    }
}

impl ops::Mul<Expr> for Expr {
    type Output = Expr;

    fn mul(self, rhs: Expr) -> Self::Output {
        Expr::Binary(BinaryOp::Mul, Box::new(self), Box::new(rhs))
    }
}

impl ops::Div<Expr> for Expr {
    type Output = Expr;

    fn div(self, rhs: Expr) -> Self::Output {
        Expr::Binary(BinaryOp::Div, Box::new(self), Box::new(rhs))
    }
}

impl ops::BitXor<Expr> for Expr {
    type Output = Expr;

    fn bitxor(self, rhs: Expr) -> Self::Output {
        Expr::Binary(BinaryOp::Pow, Box::new(self), Box::new(rhs))
    }
}

impl Expr {
    fn zero() -> Expr { Expr::Number(0f64) }

    fn one() -> Expr { Expr::Number(1f64) }

    fn var(c: char) -> Expr { Expr::Variable(c) }

    fn is_constant(&self) -> bool {
        match self {
            Expr::Number(_) => true,
            Expr::Variable(_) => false,
            Expr::Unary(_, arg) => arg.is_constant(),
            Expr::Binary(_, lhs, rhs) => lhs.is_constant() && rhs.is_constant(),
        }
    }

    fn get_copy(&self) -> Expr {
        self.clone()
    }

    fn unary_with(&self, op: UnaryOp) -> Expr {
        Expr::Unary(op, Box::new(self.clone()))
    }

    fn sin(&self) -> Expr {
        self.unary_with(UnaryOp::Sine)
    }

    fn cos(&self) -> Expr {
        self.unary_with(UnaryOp::Cosine)
    }

    fn log(&self) -> Expr {
        self.unary_with(UnaryOp::Logarithm)
    }

    fn exp(&self) -> Expr {
        self.unary_with(UnaryOp::Exponent)
    }

    fn inverse(&self) -> Expr {
        Expr::one() / self.clone()
    }

    fn square(&self) -> Expr {
        Expr::Binary(BinaryOp::Pow, Box::new(self.clone()), Box::new(Expr::Number(2f64)))
    }

    fn neg(&self) -> Expr { Expr::Number(-1f64) * self.clone() }

    fn differentiate(&self) -> Expr {
        match self {
            Expr::Number(_) => Expr::zero(),
            Expr::Variable(_) => Expr::one(),
            Expr::Unary(op, arg) => match op {
                UnaryOp::Sine => arg.differentiate() * arg.cos(),
                UnaryOp::Cosine => arg.differentiate() * arg.sin().neg(),
                UnaryOp::Tangent => arg.differentiate() * (Expr::one() + self.square()),
                UnaryOp::Exponent => arg.differentiate() * self.get_copy(),
                UnaryOp::Logarithm => arg.differentiate() * arg.inverse(),
            },
            Expr::Binary(op, lhs, rhs) => match op {
                BinaryOp::Add => lhs.differentiate() + rhs.differentiate(),
                BinaryOp::Sub => lhs.differentiate() - rhs.differentiate(),
                BinaryOp::Mul => lhs.differentiate() * rhs.get_copy() + lhs.get_copy() * rhs.differentiate(),
                BinaryOp::Div => if rhs.is_constant() { // Denominator is constant
                    lhs.differentiate() / rhs.get_copy()
                } else { // General rule for division
                    (lhs.differentiate() * rhs.get_copy() - lhs.get_copy() * rhs.differentiate()) / rhs.square()
                },
                BinaryOp::Pow => match (lhs.is_constant(), rhs.is_constant()) { // the precedence here is the key to pass, notice parts embraced in brackets
                    (true, true) => Expr::zero(), // {a ^ b}' = 0
                    (true, false) => rhs.differentiate() * ((lhs.get_copy() ^ rhs.get_copy()) * lhs.log()), // {a ^ f(x)}' = f'(x) * [a ^ f(x) * ln a]
                    (false, true) => lhs.differentiate() * (rhs.get_copy() * (lhs.get_copy() ^ (rhs.get_copy() - Expr::one()))), // {f(x) ^ a}' = f'(x) * [a * f(x) ^ (a - 1)]
                    (false, false) => (rhs.get_copy() * lhs.log()).exp().differentiate(), // {f(x) ^ g(x)}' = {e ^ (g(x) * ln(f(x)))}'
                },
            },
        }
    }

    fn simplify(&self) -> Expr {
        match self {
            Expr::Unary(op, arg) => Expr::Unary(*op, Box::new(arg.simplify())),
            Expr::Binary(op, lhs, rhs) => match (op, lhs.simplify(), rhs.simplify()) {
                (op, Expr::Number(x), Expr::Number(y)) => Expr::Number(op.eval(x, y)),
                (BinaryOp::Add, Expr::Number(c), fx) => if c == 0f64 { fx } else { Expr::Number(c) + fx },
                (BinaryOp::Add, fx, Expr::Number(c)) => if c == 0f64 { fx } else { fx + Expr::Number(c) },
                (BinaryOp::Sub, Expr::Variable(a), Expr::Variable(b)) => if a == b { Expr::zero() } else { Expr::var(a) - Expr::var(b) }
                (BinaryOp::Sub, fx, Expr::Number(c)) => if c == 0f64 { fx } else { fx - Expr::Number(c) },
                (BinaryOp::Mul, fx, Expr::Number(c)) => if c == 0f64 { Expr::zero() } else if c == 1f64 { fx } else { fx * Expr::Number(c) },
                (BinaryOp::Mul, Expr::Number(c), fx) => if c == 0f64 { Expr::zero() } else if c == 1f64 { fx } else { Expr::Number(c) * fx }
                (BinaryOp::Div, Expr::Number(num), den) => if num == 0f64 { Expr::zero() } else { Expr::Number(num) / den },
                (BinaryOp::Div, num, Expr::Number(den)) => if den == 0f64 { panic!("divide by zero") } else if den == 1f64 { num } else { num / Expr::Number(den) },
                (BinaryOp::Pow, base, Expr::Number(exp)) => if exp == 0f64 { Expr::one() } else if exp == 1f64 { base } else { base ^ Expr::Number(exp) },
                (BinaryOp::Pow, Expr::Number(base), exp) => if base == 0f64 { Expr::zero() } else if base == 1f64 { Expr::one() } else { Expr::Number(base) ^ exp },
                (op, lhs, rhs) => Expr::Binary(*op, Box::new(lhs), Box::new(rhs)),
            },
            _ => self.clone(),
        }
    }
}

impl ToString for Expr {
    fn to_string(&self) -> String {
        match self {
            Expr::Number(value) => value.to_string(),
            Expr::Variable(name) => name.to_string(),
            Expr::Unary(op, arg) => format!("({} {})", op, arg.to_string()),
            Expr::Binary(op, lhs, rhs) => format!("({} {} {})", op, lhs.to_string(), rhs.to_string()),
        }
    }
}

impl FromStr for Expr {
    type Err = ExprParserError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parser = ExprParser::new(s);
        parser.parse_expression()
    }
}

struct ExprParserError {
    at: usize,
    meet: Option<char>,
    message: &'static str,
}

impl ExprParserError {
    fn new(at: usize, meet: Option<char>, message: &'static str) -> ExprParserError {
        ExprParserError { at, meet, message }
    }
}

struct ExprParser {
    chars: Vec<char>,
    at: usize,
}

impl ExprParser {
    fn new(s: &str) -> ExprParser {
        ExprParser { chars: s.chars().collect(), at: 0 }
    }

    fn error(&self, message: &'static str) -> ExprParserError {
        let meet = if self.at < self.chars.len() { Some(self.chars[self.at]) } else { None };
        ExprParserError::new(self.at, meet, message)
    }

    fn peek(&self) -> Option<char> {
        if self.at < self.chars.len() {
            Some(self.chars[self.at])
        } else {
            None
        }
    }

    fn next(&mut self) -> Option<char> {
        if self.at < self.chars.len() {
            let save = self.chars[self.at];
            self.at += 1;
            Some(save)
        } else {
            None
        }
    }

    fn skip_whitespace(&mut self) {
        while let Some(c) = self.peek() {
            if c.is_whitespace() { self.next(); } else { break; }
        }
    }

    fn parse_expression(&mut self) -> Result<Expr, ExprParserError> {
        self.skip_whitespace();
        if let Some(c) = self.peek() {
            if c == '(' {
                self.next();
                self.skip_whitespace();
                let result = if let Some(op) = self.parse_unary() {
                    match self.parse_expression() {
                        Ok(arg) => Ok(Expr::Unary(op, Box::new(arg))),
                        e => e,
                    }
                } else if let Some(op) = self.parse_operator() {
                    self.next();
                    match self.parse_expression() {
                        Ok(lhs) => match self.parse_expression() {
                            Ok(rhs) => Ok(Expr::Binary(op, Box::new(lhs), Box::new(rhs))),
                            e => e,
                        },
                        e => e,
                    }
                } else {
                    Err(self.error("expect binary or unary expression"))
                };
                self.skip_whitespace();
                if let Some(')') = self.next() {
                    result
                } else {
                    Err(self.error("bracket doesn't match"))
                }
            } else if c.is_ascii_digit() || c == '.' || c == '-' {
                let mut buf = String::new();
                while let Some(c) = self.peek() {
                    if c.is_ascii_digit() || c == '.' || c == '-' {
                        buf.push(c);
                        self.next();
                    } else {
                        break;
                    }
                }
                Ok(Expr::Number(buf.parse::<f64>().unwrap()))
            } else if c.is_alphabetic() {
                self.next();
                Ok(Expr::Variable(c))
            } else {
                Err(self.error("unrecognized character"))
            }
        } else {
            Err(self.error("expect an expression instead of EOF"))
        }
    }

    fn parse_operator(&mut self) -> Option<BinaryOp> {
        match self.peek() {
            Some('+') => Some(BinaryOp::Add),
            Some('-') => Some(BinaryOp::Sub),
            Some('*') => Some(BinaryOp::Mul),
            Some('/') => Some(BinaryOp::Div),
            Some('^') => Some(BinaryOp::Pow),
            _ => None,
        }
    }

    fn parse_identifier(&mut self) -> Option<String> {
        let mut buf = String::new();
        while let Some(c) = self.peek() {
            if c.is_alphabetic() {
                buf.push(c);
                self.next();
            } else {
                break;
            }
        }
        if buf.is_empty() { None } else { Some(buf) }
    }

    fn parse_unary(&mut self) -> Option<UnaryOp> {
        if let Some(name) = self.parse_identifier() {
            match name.as_str() {
                "sin" => Some(UnaryOp::Sine),
                "cos" => Some(UnaryOp::Cosine),
                "tan" => Some(UnaryOp::Tangent),
                "exp" => Some(UnaryOp::Exponent),
                "ln" => Some(UnaryOp::Logarithm),
                _ => None,
            }
        } else {
            None
        }
    }
}

fn diff(expr: &str) -> String {
    match Expr::from_str(expr) {
        Ok(expr) => expr.simplify().differentiate().simplify().to_string(),
        Err(err) => if let Some(c) = err.meet {
            format!("{} of {}: {}", err.at, c, err.message)
        } else {
            format!("{}: {}", err.at, err.message)
        },
    }
}
