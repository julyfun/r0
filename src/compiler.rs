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
        Let(var, box var_ast, box res_ast) => {
            let new_name = gensym();
            // new_name.clone() 防止夺取所有权
            let new_map = hashmap!(var => new_name.clone());
            let sub_uniq_name_env: SymTable<String, String> =
                SymTable::extended(new_map, &uniq_name_env);
            Let(
                new_name,
                Box::new(uniqify_expr(var_ast, uniq_name_env)),
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
        // _ => panic!("invalid expr {expr:?}"),
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
        x1,
        box Let(x2, _i1, box Prim2(_add, box Var(x2_), box _i2)),
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

/// anf_exp：任何 prim1 or 2 的参数是原子表达式（ Int or Var )
/// anf_exp
/// 然而不是改子树就行的，可能需要改根的类型
fn anf_exp(expr: Expr) -> Expr {
    match expr {
        // - L
        // if L is not atm
        //     return Let x L.anf (-x)
        Prim1(op, box e) => {
            if !is_atm(&e) {
                let x = gensym();
                Let(
                    x.clone(),
                    Box::new(anf_exp(e)),
                    Box::new(Prim1(op, Box::new(Var(x)))),
                )
            } else {
                // e is atm
                Prim1(op, Box::new(e))
            }
        }
        // 转化时，类型可能变化
        Prim2(op, box e1, box e2) => {
            // (+ L R)
            // if L is not atm
            //     => Let (x L.anf (+ x R).anf) // 记得保证所有子节点为 anf
            // what if e1 is prim0?
            if !is_atm(&e1) {
                let x = gensym();
                Let(
                    x.clone(),
                    Box::new(anf_exp(e1)),
                    Box::new(anf_exp(Prim2(op, Box::new(Var(x)), Box::new(e2)))),
                )
            } else if !is_atm(&e2) {
                // e1 is atm
                let y = gensym();
                Let(
                    y.clone(),
                    Box::new(anf_exp(e2)),
                    Box::new(Prim2(op, Box::new(e1), Box::new(Var(y)))),
                )
            } else {
                Prim2(op, Box::new(e1), Box::new(e2))
            }
        }
        Let(var, box var_ast, box res_ast) => {
            Let(var, Box::new(anf_exp(var_ast)), Box::new(anf_exp(res_ast)))
        }
        // Prim0, Int, Var
        other => other,
    }
}

fn is_atm(expr: &Expr) -> bool {
    match expr {
        Int(_) | Var(_) => true,
        _ => false,
    }
}

#[test]
fn test_anf_exp() {
    use crate::parser::parse;
    use crate::syntax::Expr::*;
    let e = "(+ (- 2) (+ 30 10))";
    // let t1 (- 2) (let t2 (+ 30 10) (+ t1 t2)))
    let exp = parse(e); // suppose work right
    let exp = anf_exp(exp);
    println!("{exp:?}");
    if let Let(
        t1,
        box Prim1(op, box that_2),
        box Let(
            t2,
            box Prim2(op2, box that30, box that10),
            box Prim2(op3, box Var(var_t1), box Var(var_t2)),
        ),
    ) = exp
    {
        assert_eq!(op, "-");
        assert!(matches!(that_2, Int(2)));
        assert_eq!(op2, "+");
        assert!(matches!(that30, Int(30)));
        assert!(matches!(that10, Int(10)));
        assert_eq!(op3, "+");
        assert_eq!(t1, var_t1);
        assert_eq!(t2, var_t2);
    } else {
        panic!("anf_exp fails!");
    }
}

// Atomic: Int or Var
// fn anf_atm(expr: Expr) -> Expr {
//     match expr {
//         other => other,
//     }
// }
