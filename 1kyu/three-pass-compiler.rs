use std::slice::Iter;
use std::iter::Peekable;
use std::collections::HashMap;

#[derive(Clone)]
enum Ast {
    BinOp(String, Box<Ast>, Box<Ast>),
    UnOp(String, i64),
}

impl Ast {
    fn imm(x: i64) -> Ast { Ast::UnOp("imm".to_string(), x) }

    fn bin(op: &str, lhs: Ast, rhs: Ast) -> Ast { Ast::BinOp(op.to_string(), Box::new(lhs), Box::new(rhs) ) }

    fn reduce(&self) -> Ast {
        match self {
            Ast::UnOp(..) => self.clone(),
            Ast::BinOp(op, lhs, rhs) => {
                let reduced_lhs = lhs.reduce();
                let reduced_rhs = rhs.reduce();
                match (op.as_str(), &reduced_lhs, &reduced_rhs) {
                    ("+", Ast::UnOp(tx, x), Ast::UnOp(ty, y)) if tx == "imm" && ty == "imm" => Ast::imm(x + y),
                    ("-", Ast::UnOp(tx, x), Ast::UnOp(ty, y)) if tx == "imm" && ty == "imm" => Ast::imm(x - y),
                    ("*", Ast::UnOp(tx, x), Ast::UnOp(ty, y)) if tx == "imm" && ty == "imm" => Ast::imm(x * y),
                    ("/", Ast::UnOp(tx, x), Ast::UnOp(ty, y)) if tx == "imm" && ty == "imm" => Ast::imm(x / y),
                    (op, lhs, rhs) => Ast::bin(op, lhs.clone(), rhs.clone()),
                }
            },
        }
    }

    fn emit(&self, output: &mut Vec<String>) {
        match self {
            Ast::UnOp(op, val) => match op.as_str() {
                "imm" => output.push(format!("IM {}", val)),
                "arg" => output.push(format!("AR {}", val)),
                _     => panic!("unrecognized unary operator"),
            },
            Ast::BinOp(op, lhs, rhs) => {
                lhs.emit(output);
                output.push("SW".to_string());
                output.push("PU".to_string());
                rhs.emit(output);
                output.push("SW".to_string());
                output.push(match op.as_str() {
                    "+" => "AD".to_string(),
                    "-" => "SU".to_string(),
                    "*" => "MU".to_string(),
                    "/" => "DI".to_string(),
                    _   => panic!("unrecognized binary operator"),
                });
                output.push("SW".to_string());
                output.push("PO".to_string());
                output.push("SW".to_string());
            }
        }
    }
}

impl ToString for Ast {
    fn to_string(&self) -> String {
        match self {
            Ast::UnOp(op, x) => format!("({} {})", op, x),
            Ast::BinOp(op, lhs, rhs) => format!("({} {} {})", op, lhs.to_string(), rhs.to_string()),
        }
    }
}

enum Token {
    Identifier(String),
    Literal(i32),
    Symbol(char),
}

struct Compiler {
    args: HashMap<String, i32>,
}

impl Compiler {
    fn new() -> Compiler {
        Compiler { args: HashMap::new() }
    }

    fn tokenize<'a>(&self, program : &'a str) -> Vec<Token> {
        let mut tokens : Vec<Token> = vec![];

        let mut iter = program.chars().peekable();
        loop {
            match iter.peek() {
                Some(&c) => match c {
                    'a'..='z'|'A'..='Z' => {
                        let mut tmp = String::new();
                        while iter.peek().is_some() && iter.peek().unwrap().is_alphabetic() {
                            tmp.push(iter.next().unwrap());
                        }
                        tokens.push(Token::Identifier(tmp));
                    },
                    '0'..='9' => {
                        let mut tmp = String::new();
                        while iter.peek().is_some() && iter.peek().unwrap().is_numeric() {
                            tmp.push(iter.next().unwrap());
                        }
                        tokens.push(Token::Literal(tmp.parse().unwrap()));
                    },
                    ' ' => { iter.next(); },
                    _ => {
                        tokens.push(Token::Symbol(iter.next().unwrap()));
                    },
                },
                None => break
            }
        }

        tokens
    }

    fn compile(&mut self, program : &str) -> Vec<String> {
        let ast = self.pass1(program);
        let ast = self.pass2(&ast);
        self.pass3(&ast)
    }

    fn parse_function(&mut self, iter: &mut Peekable<Iter<Token>>) -> Ast {
        self.args.clear();
        let mut arg_counter = 0;
        Compiler::expect_symbol(iter, '[');
        while let Some(Token::Identifier(name)) = iter.peek() {
            iter.next();
            self.args.insert(name.clone(), arg_counter);
            arg_counter += 1;
        }
        Compiler::expect_symbol(iter, ']');
        self.parse_expression(iter)
    }

    fn parse_expression(&self, iter: &mut Peekable<Iter<Token>>) -> Ast {
        let mut lhs = self.parse_term(iter);
        while let Some(Token::Symbol(c)) = iter.peek() {
            if *c == '+' || *c == '-' {
                iter.next();
                let rhs = self.parse_term(iter);
                lhs = Ast::BinOp(c.to_string(), Box::new(lhs), Box::new(rhs));
            } else {
                break;
            }
        }
        lhs
    }

    fn parse_term(&self, iter: &mut Peekable<Iter<Token>>) -> Ast {
        let mut lhs = self.parse_factor(iter);
        while let Some(Token::Symbol(c)) = iter.peek() {
            if *c == '*' || *c == '/' {
                iter.next();
                let rhs = self.parse_factor(iter);
                lhs = Ast::BinOp(c.to_string(), Box::new(lhs), Box::new(rhs));
            } else {
                break;
            }
        }
        lhs
    }

    fn parse_factor(&self, iter: &mut Peekable<Iter<Token>>) -> Ast {
        match iter.next() {
            Some(Token::Literal(x)) => Ast::UnOp("imm".to_string(), *x as i64),
            Some(Token::Identifier(name)) => match self.args.get(name.as_str()) {
                Some(index) => Ast::UnOp("arg".to_string(), *index as i64),
                None => panic!("undeclared identifier"),
            },
            Some(Token::Symbol('(')) => {
                let content = self.parse_expression(iter);
                Compiler::expect_symbol(iter, ')');
                content
            },
            _ => panic!("expect a number, variable or parenthesised expression")
        }
    }

    fn expect_symbol(iter: &mut Peekable<Iter<Token>>, s: char) {
        if let Some(..) = iter.peek() {
            if let Token::Symbol(s) = iter.next().unwrap() {
                ()
            } else {
                panic!("unexpected token");
            }
        } else {
            panic!("unexpected EOF");
        }
    }

    fn pass1(&mut self, program : &str) -> Ast {
        let tokens = self.tokenize(program);
        let mut iter = tokens.iter().peekable();
        self.parse_function(&mut iter)
    }

    fn pass2(&mut self, ast : &Ast) -> Ast {
        ast.reduce()
    }

    fn pass3(&mut self, ast : &Ast) -> Vec<String> {
        let mut result = Vec::new();
        ast.emit(&mut result);
        result
    }
}

fn simulate(assembly : Vec<&str>, argv : Vec<i32>) -> i32 {
    let mut r = (0, 0);
    let mut stack : Vec<i32> = vec![];

    for ins in assembly {
        let mut ws = ins.split_whitespace();
        match ws.next() {
            Some("IM") => r.0 = i32::from_str_radix(ws.next().unwrap(), 10).unwrap(),
            Some("AR") => r.0 = argv[i32::from_str_radix(ws.next().unwrap(), 10).unwrap() as usize],
            Some("SW") => r = (r.1,r.0),
            Some("PU") => stack.push(r.0),
            Some("PO") => r.0 = stack.pop().unwrap(),
            Some("AD") => r.0 += r.1,
            Some("SU") => r.0 -= r.1,
            Some("MU") => r.0 *= r.1,
            Some("DI") => r.0 /= r.1,
            _ => panic!("Invalid instruction encountered"),
        }
    }
    r.0
}

fn main() {
    let mut compiler = Compiler::new();
    let code = "[a b] a * a + b * b";
    let tokens = compiler.tokenize(code);
    tokens.iter().for_each(|x| match x {
        Token::Literal(x) => print!("Literal({}), ", x),
        Token::Identifier(x) => print!("Identifier({}), ", x),
        Token::Symbol(x) => print!("Symbol({}), ", x),
    });
    let ast = compiler.pass1(code);
    println!("{}", ast.to_string());
}
