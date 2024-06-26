use std::rc::Rc;

use crate::hashmap;
use crate::helper::readint;
use crate::syntax::{
    Expr::{self, *},
    SymTable,
};

/// AST -> value
pub fn interp_exp(expr: Expr, env: Rc<SymTable<String, i64>>) -> i64 {
    match expr {
        Int(val) => val,
        // match 过程中 read() 被 parse 的顺序是...
        // 后方保证是从左往右
        Prim0(op) if op.as_str() == "read" => readint(),
        Prim1(op, box e) if op.as_str() == "-" => -interp_exp(e, env),
        Prim2(op, box e1, box e2) if op.as_str() == "+" => {
            // 前面这次不夺取 Rc 所有权
            interp_exp(e1, Rc::clone(&env)) + interp_exp(e2, env)
        }
        Var(name) => *env.lookup(&name), // 自动解 Rc
        Let(name, box e1, box e2) => {
            // let x (+ 2 3) (+ x 5)
            // 在父环境下解释 e1
            let name_val = interp_exp(e1, Rc::clone(&env));
            // Rc::clone 不会复制内容
            let sub_map = hashmap!(name => name_val);
            let sub_env = Rc::new(SymTable::<String, i64>::extended(sub_map, &env));
            let right_val = interp_exp(e2, sub_env);
            right_val
        }
        _ => panic!("invalid ast form"),
    }
}

pub fn interp_r1(expr: Expr) -> i64 {
    interp_exp(expr, Rc::new(SymTable::<String, i64>::new()))
}
