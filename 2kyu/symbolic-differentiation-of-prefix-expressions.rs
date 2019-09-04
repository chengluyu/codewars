use std::ops;
use std::fmt::Display;

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

impl Expr {
    fn unary_with(self, op: UnaryOp) -> Expr {
        Expr::Unary(op, Box::new(self))
    }

    fn sin(self) -> Expr {
        self.unary_with(UnaryOp::Sine)
    }

    fn cos(self) -> Expr {
        self.unary_with(UnaryOp::Cosine)
    }

    fn tan(self) -> Expr {
        self.unary_with(UnaryOp::Tangent)
    }

    fn exp(self) -> Expr {
        self.unary_with(UnaryOp::Exponent)
    }

    fn log(self) -> Expr {
        self.unary_with(UnaryOp::Logarithm)
    }

    fn neg(self) -> Expr { Expr::Number(-1f64) * self }

    fn differentiate(&self) -> Expr {
        match self {
            Expr::Number(value) => Expr::Number(0f64),
            Expr::Variable(name) => Expr::Number(1f64),
            Expr::Unary(op, arg) => match op {
                UnaryOp::Sine => arg.differentiate() * arg.cos(),
                UnaryOp::Cosine => arg.differentiate() * arg.sin().neg(),
                UnaryOp::Tangent => unimplemented!(),
                UnaryOp::Exponent => unimplemented!(),
                UnaryOp::Logarithm => unimplemented!(),
            },
            Expr::Binary(op, lhs, rhs) => match op {
                BinaryOp::Add => lhs.differentiate() + rhs.differentiate(),
                BinaryOp::Sub => lhs.differentiate() + rhs.differentiate(),
                BinaryOp::Mul => unimplemented!(),
                BinaryOp::Div => unimplemented!(),
                BinaryOp::Pow => unimplemented!(),
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
