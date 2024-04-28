use std::rc::Rc;

use crate::hashmap;
use crate::helper::gensym;
use crate::syntax::{
    Expr::{self, *},
    SymTable,
};

pub fn uniquify(expr: Expr) -> Expr {
    let env = Rc::new(SymTable::new());
    uniqify_expr(expr, env)
}

// AST Node -> AST Node 但变量名更换
fn uniqify_expr(expr: Expr, uniq_name_env: Rc<SymTable<String, String>>) -> Expr {
    match expr {
        // old var
        Var(x) => Var(uniq_name_env.lookup(&x).to_string()),
        Int(n) => Int(n),
        //
        Let(box Var(name), box name_ast, box res_ast) => {
            let new_name = gensym();
            // new_name.clone() 防止夺取所有权
            let new_map = hashmap!(name => new_name.clone());
            let sub_uniq_name_env: SymTable<String, String> =
                SymTable::extended(new_map, &uniq_name_env);
            Let(
                Box::new(Var(new_name)),
                Box::new(uniqify_expr(name_ast, uniq_name_env)),
                Box::new(uniqify_expr(res_ast, Rc::new(sub_uniq_name_env))),
            )
        }
        Prim0(op) => Prim0(op),
        Prim1(op, box e1) => Prim1(op, Box::new(uniqify_expr(e1, uniq_name_env))),
        Prim2(op, box e1, box e2) => Prim2(
            op,
            Box::new(uniqify_expr(e1, uniq_name_env.clone())),
            Box::new(uniqify_expr(e2, uniq_name_env)),
        ),
        _ => panic!("invalid expr {expr:?}"),
    }
}

#[test]
fn test_uniquify() {
    use crate::interp::interp_r1;
    use crate::parser::parse;
    use crate::syntax::Expr::*;
    let e = "(let (x (let (x 4)
                    (+ x 1)))
            (+ x 2))";
    let exp = parse(e);
    let exp = uniquify(exp);
    if let Let(
        box Var(x1),
        box Let(box Var(x2), _i1, box Prim2(_add, box Var(x2_), box _i2)),
        box Prim2(_add1, box Var(x1_), box _i3),
    ) = &exp
    {
        assert_ne!(x1, "x");
        assert_eq!(x1, x1_);
        assert_eq!(x2, x2_);
        assert_ne!(x1, x2);
    } else {
        panic!("uniquify fails!");
    }
    let res = interp_r1(exp);
    assert_eq!(res, 7);
}
