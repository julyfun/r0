use crate::{
    helper::{is_digit, is_valid_var_name},
    syntax::Expr::{self, *},
};

/// 生成的 Token Tree
/// Token 就是拆分好的 String
/// String -> Token Tree -> AST
/// shouldn't be public
#[derive(Debug, Eq, PartialEq)]
enum Sexpr {
    // pub only for test?
    Atom(String),
    /// 第一个元素是父节点，后面元素依次为子节点
    List(Vec<Sexpr>),
}
use Sexpr::{Atom, List}; // "define" Atom and List for below

/// str -> token tree
fn scan(expr: &str) -> Sexpr {
    let mut stack = vec![];
    let mut list = vec![]; // 当前子树
    let mut sym = String::new(); // 当前 token symbol (string)
    for c in expr.chars() {
        // (+ (+ 29 (- 8)) 10)
        match c {
            '(' => {
                stack.push(list); // moved
                list = vec![]; // 开启全新的 list
            }
            '0'..='9' => sym.push(c),
            'a'..='z' | 'A'..='Z' => sym.push(c),
            '_' => sym.push(c),
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
                pre_list.push(List(list)); // 该子树成为其父亲的一个子结点
                list = pre_list;
            }
            _ => (), // 这里应该 panic 么
        }
    }
    if !sym.is_empty() {
        Atom(sym)
    } else {
        list.pop().unwrap()
    }
}

#[test]
fn test_scan() {
    let s = "(1 2 (+ 1 2))";
    let expr = scan(s);
    let t = List(vec![
        Atom("1".to_string()),
        Atom("2".to_string()),
        List(vec![
            Atom("+".to_string()),
            Atom("1".to_string()),
            Atom("2".to_string()),
        ]),
    ]);
    assert_eq!(expr, t);
}

/// token (symbol string) tree -> abstract syntax tree
/// ast 可存储数值，变量名字符串，二进制化的关键字
fn parse_sexpr(sexpr: &Sexpr) -> Expr {
    match sexpr {
        Atom(s) if is_digit(s) => {
            let val: i64 = s
                .parse()
                .unwrap_or_else(|_| panic!("invalid number: {}", s));
            Int(val)
        }
        // 目前唯一生成 Var 类型（是已有变量名，而不是新建变量）
        Atom(s) if is_valid_var_name(s) => Var(s.to_string()),
        List(vec) => match vec.as_slice() {
            [Atom(op)] if op.as_str() == "read" => Prim0(op.clone()),
            [Atom(op), e] if op.as_str() == "-" => Prim1(op.clone(), Box::new(parse_sexpr(e))),
            // vec 类型为 Vec<Sexpr>
            [Atom(op), e1, e2] if op.as_str() == "+" => Prim2(
                op.clone(),
                Box::new(parse_sexpr(e1)),
                Box::new(parse_sexpr(e2)),
            ),
            // let expression
            // token   bind      exp
            // let    (x (...))  (...)
            // 并不是所有的 List 都是一个 i64
            [Atom(token), List(bind), exp] if token.as_str() == "let" => {
                match bind.as_slice() {
                    [Atom(name), var_exp] if is_valid_var_name(name.as_str()) => Let(
                        Box::new(Var(name.clone())),
                        // 这里结果是啥：一个二进制化的树
                        Box::new(parse_sexpr(var_exp)),
                        Box::new(parse_sexpr(exp)),
                    ),
                    [Atom(name), _] if !is_valid_var_name(name.as_str()) => {
                        panic!("invalid var name: {name:?}")
                    }
                    _ => {
                        panic!("invalid bind form: {bind:?}")
                    }
                }
            }
            _ => panic!("invalid form: {vec:?}"),
        },
        _ => panic!("invalid sexpr: {sexpr:?}"),
    }
}

/// str -> AST
pub fn parse(expr: &str) -> Expr {
    let sexpr = scan(expr);
    let expr = parse_sexpr(&sexpr);
    expr
}

#[test]
fn test_valid_name() {
    let s = "(let (_x 2) 3)";
    let _ = parse(s);
}

#[test]
#[should_panic(expected = "invalid var name: \"1x\"")]
fn test_invalid_name() {
    let s = "(let (1x 2) 3)";
    let _ = parse(s);
}
