#![feature(box_patterns)]

mod helper;
mod syntax;
mod test;

pub use crate::helper::readint;
pub use crate::syntax::Expr::{self, *};

fn interp_exp(expr: Expr) -> i64 {
    match expr {
        Int(val) => val,
        Prim0(op) if op.as_str() == "read" => readint(),
        Prim1(op, box e) if op.as_str() == "-" => -interp_exp(e),
        Prim2(op, box e1, box e2) if op.as_str() == "+" => interp_exp(e1) + interp_exp(e2),
        _ => panic!("Invalid form!"),
    }
}

fn main() {
    println!("Hello, world!");
}
