use crate::syntax::Expr::{self, *};

/// 生成的 Token
/// String -> Token Tree -> AST
#[derive(Debug, Eq, PartialEq)]
pub enum Sexpr {
    Atom(String),
    List(Vec<Sexpr>),
}
pub use Sexpr::{Atom, List};

pub fn scan(expr: &str) -> Sexpr {
    let mut stack = vec![];
    let mut list = vec![];
    let mut sym = String::new();
    for c in expr.chars() {
        // (+ (+ 29 (- 8)) 10)
        match c {
            '(' => {
                stack.push(list); // moved
                list = vec![];
            }
            '0'..='9' => sym.push(c),
            'a'..='z' | 'A'..='Z' => sym.push(c),
            '+' | '-' | '*' | '/' => sym.push(c),
            ' ' => {
                if !sym.is_empty() {
                    list.push(Atom(sym));
                    sym = String::new();
                }
                // 如果不是空的算匹配成功哈
            }
            ')' => {
                if !sym.is_empty() {
                    list.push(Atom(sym));
                    sym = String::new();
                }
                let mut pre_list = stack.pop().unwrap();
                pre_list.push(List(list));
                list = pre_list;
            }
            _ => (), // 这里应该 painc 么
        }
    }
    if !sym.is_empty() {
        Atom(sym)
    } else {
        list.pop().unwrap()
    }
}

pub fn parse_sexpr(sexpr: &Sexpr) -> Expr {
    match sexpr {
        Atom(s) => {
            let val: i64 = s.parse().expect("Not an integer");
            Int(val)
        }
        List(v) => match v.as_slice() {
            [Atom(op)] if op.as_str() == "read" => Prim0(op.clone()),
            [Atom(op), e] if op.as_str() == "-" => Prim1(op.clone(), Box::new(parse_sexpr(e))),
            [Atom(op), e1, e2] if op.as_str() == "+" => Prim2(
                op.clone(),
                Box::new(parse_sexpr(e1)),
                Box::new(parse_sexpr(e2)),
            ),
            _ => panic!("Invalid form!"),
        },
    }
}

pub fn parse(expr: &str) -> Expr {
    let sexpr = scan(expr);
    let expr = parse_sexpr(&sexpr);
    expr
}
