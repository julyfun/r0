#![allow(unused_imports)]
use std::rc::Rc;

// 非 pub 是怎么拿过来的
// 要么是 main.rs 特权
// 要么是 test.rs 或者 #[test] 特权

#[test]
fn test_r0() {
    use crate::interp::interp_r1;
    use crate::syntax::Expr::*;
    let p3 = Prim2("+".to_string(), Box::new(Int(10)), Box::new(Int(32)));
    let r = interp_r1(p3);
    assert_eq!(r, 42);
}

#[test]
fn test_parse() {
    use crate::parser::parse;
    use crate::syntax::Expr::*;
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
    use crate::interp::interp_r1;
    use crate::parser::parse;

    let s = "(+ 1 (+ 2 3))";
    let expr = parse(s);
    let r = interp_r1(expr); // defined in main
    assert_eq!(r, 6);
}

#[test]
fn test_env2() {
    use crate::syntax::SymTable;
    use crate::{hashmap, string};
    let env = Rc::new(SymTable::new());
    let map = hashmap!(string!("jenny") => 100, string!("x") => 42);
    // 此处推断出 env 的类型
    let env2 = SymTable::<String, i64>::extended(map, &env);
    let v = env2.lookup(&"x".to_string());
    assert_eq!(*v, 42);
}

#[test]
fn test_nest_let() {
    use crate::interp::interp_r1;
    use crate::syntax::Expr::*;
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
    use crate::interp::interp_r1;
    use crate::parser::parse;
    let s = "(let (x 8) (let (y (+ x 2)) (+ x y)))";
    let exp = parse(s);
    assert_eq!(interp_r1(exp), 18);
}
