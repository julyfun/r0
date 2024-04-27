#![allow(unused_imports)]
use crate::parser::*;
use crate::*;
use std::rc::Rc;

// 非 pub 是怎么拿过来的
// 要么是 main.rs 特权
// 要么是 test.rs 或者 #[test] 特权

#[test]
fn test_r0() {
    let p3 = Prim2("+".to_string(), Box::new(Int(10)), Box::new(Int(32)));
    let r = interp_r1(p3);
    assert_eq!(r, 42);
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

#[test]
fn test_parse() {
    let s = "(+ 1 2)";
    let expr = Prim2("+".to_string(), Box::new(Int(1)), Box::new(Int(2)));
    assert_eq!(parse(s), expr);
    let s = "(- 10)";
    let expr = Prim1("-".to_string(), Box::new(Int(10)));
    assert_eq!(parse(s), expr);
    let s = "(read)";
    let expr = Prim0("read".to_string());
    assert_eq!(parse(s), expr);
}

#[test]
fn test_interp() {
    let s = "(+ 1 (+ 2 3))";
    let expr = parse(s);
    let r = interp_r1(expr); // defined in main
    assert_eq!(r, 6);
}

#[test]
fn test_env2() {
    use crate::syntax::SymTable;
    let env = Rc::new(SymTable::new());
    let map = hashmap!(string!("jenny") => 100, string!("x") => 42);
    // 此处推断出 env 的类型
    let env2 = SymTable::<String, i64>::extend(map, &env);
    let v = env2.lookup(&"x".to_string());
    assert_eq!(*v, 42);
}

#[test]
fn test_nest_let() {
    let exp = Let(
        "x".to_string(),
        Box::new(Int(8)),
        Box::new(Let(
            "y".to_string(),
            Box::new(Int(34)),
            Box::new(Prim2(
                "+".to_string(),
                Box::new(Var("x".to_string())),
                Box::new(Var("y".to_string())),
            )),
        )),
    );
    assert_eq!(interp_r1(exp), 42);
}

#[test]
fn test_parse_r1() {
    let s = "(let (x 8) (let (y (+ x 2)) (+ x y)))";
    let exp = parse(s);
    assert_eq!(interp_r1(exp), 18);
}
