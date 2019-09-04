use std::ops;
use std::fmt::Display;
use std::str::FromStr;
use std::borrow::Borrow;

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
    fn unary_with(&self, op: UnaryOp) -> Expr {
        Expr::Unary(op, Box::new(self.clone()))
    }

    fn sin(&self) -> Expr {
        self.unary_with(UnaryOp::Sine)
    }

    fn cos(&self) -> Expr {
        self.unary_with(UnaryOp::Cosine)
    }

    fn tan(&self) -> Expr {
        self.unary_with(UnaryOp::Tangent)
    }

    fn exp(&self) -> Expr {
        self.unary_with(UnaryOp::Exponent)
    }

    fn log(&self) -> Expr {
        self.unary_with(UnaryOp::Logarithm)
    }

    fn inverse(&self) -> Expr {
        Expr::Number(1f64) / self.clone()
    }

    fn square(&self) -> Expr {
        self.clone() * self.clone()
    }

    fn neg(&self) -> Expr { Expr::Number(-1f64) * self.clone() }

    fn differentiate(&self) -> Expr {
        match self {
            Expr::Number(value) => Expr::Number(0f64),
            Expr::Variable(name) => Expr::Number(1f64),
            Expr::Unary(op, arg) => match op {
                UnaryOp::Sine => arg.differentiate() * arg.cos(),
                UnaryOp::Cosine => arg.differentiate() * arg.sin().neg(),
                UnaryOp::Tangent => arg.differentiate() * arg.cos().square().inverse(),
                UnaryOp::Exponent => arg.clone(),
                UnaryOp::Logarithm => arg.differentiate() * arg.inverse(),
            },
            Expr::Binary(op, lhs, rhs) => match op {
                BinaryOp::Add => lhs.differentiate() + rhs.differentiate(),
                BinaryOp::Sub => lhs.differentiate() - rhs.differentiate(),
                BinaryOp::Mul => lhs.differentiate() * rhs.borrow().clone() + lhs.borrow().clone() * rhs.differentiate(),
                BinaryOp::Div => (lhs.differentiate() * rhs.borrow().clone() - lhs.borrow().clone() * rhs.differentiate()) / rhs.square(),
                BinaryOp::Pow => (lhs ^ rhs) * (lhs.differentiate() * (rhs / lhs) + rhs.differentiate() * lhs.log()),
            },
        }
    }

    fn simplify(&self) -> Expr {
        match self {
            &Expr::Number(value) => Expr::Number(value),
            &Expr::Variable(name) => Expr::Variable(name),
            Expr::Unary(op, arg) => {
                let simplified_arg = arg.simplify();
                if let Expr::Number(x) = simplified_arg {
                    Expr::Number(op.eval(x))
                } else {
                    Expr::Unary(*op, Box::new(simplified_arg))
                }
            },
            Expr::Binary(op, lhs, rhs) => {
                match (op, lhs.simplify(), rhs.simplify()) {
                    (BinaryOp::Add, Expr::Number(0f64), r) => r, // 0 + x = x
                    (BinaryOp::Add, l, Expr::Number(0f64)) | (BinaryOp::Sub, l, Expr::Number(0f64)) => l, // x + 0 = x - 0 = x
                    (BinaryOp::Mul, Expr::Number(0f64), _) | (BinaryOp::Mul, _, Expr::Number(0f64)) => Expr::Number(0f64), // 0 * x = x * 0 = 0
                    (BinaryOp::Mul, Expr::Number(1f64), r) => r, // 1 * x = x
                    (BinaryOp::Mul, l, Expr::Number(1f64)) | (BinaryOp::Div, l, Expr::Number(1f64)) => l, // x * 1 = x / 1 = x
                    (o, Expr::Number(x), Expr::Number(y)) => Expr::Number(o.eval(x, y)),
                    (o, l, r) => Expr::Binary(*o, Box::new(l), Box::new(r)),
                }
            }
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
    message: &'static str,
}

impl ExprParserError {
    fn new(at: usize, message: &'static str) -> ExprParserError {
        ExprParserError { at, message }
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
        ExprParserError::new(self.at, message)
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
                if let Some(op) = self.parse_unary() {
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
                }
            } else if c.is_ascii_digit() {
                let mut buf = String::new();
                while let Some(c) = self.peek() {
                    if c.is_ascii_digit() {
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
        Ok(expr) => expr.differentiate().to_string(),
        Err(err) => format!("{}: {}", err.at, err.message),
    }
}

fn main() {
    println!("{}", diff("(* 3 (* 2 x))"));
}
