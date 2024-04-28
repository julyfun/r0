use std::collections::HashMap;
use std::hash::Hash;
use std::rc::Rc;

/// AST 结点
#[derive(Debug, PartialEq, Eq)]
pub enum Expr {
    Int(i64),
    /// 已有变量名，待查询
    Var(String),
    /// 这为啥三个 Box
    /// 这儿第一个不是 Box<Var> 捏，Var 的外在类型是 Expr
    /// 那第一个为啥不是 String 捏
    /// (let (x (+ 1 2)) (+ x 200)) -> 203
    ///      <1>  <2>       <3>
    Let(Box<Expr>, Box<Expr>, Box<Expr>),
    Prim0(String),
    Prim1(String, Box<Expr>),
    Prim2(String, Box<Expr>, Box<Expr>),
}

/// SymTable 就是变量集合（现阶段从 String => i64）
#[derive(Debug)]
pub struct SymTable<T, H>
where
    T: Eq + Hash,
    H: Eq + Hash,
{
    pub map: HashMap<T, H>,
    env: Option<Rc<SymTable<T, H>>>,
}

impl<T, H> SymTable<T, H>
where
    T: Eq + Hash,
    H: Eq + Hash,
{
    pub fn new() -> Self {
        SymTable {
            map: HashMap::new(),
            env: None,
        }
    }

    /// 寻找变量名对应的引用
    pub fn lookup(&self, x: &T) -> &H {
        if let Some(h) = self.map.get(x) {
            h
        } else if let Some(env) = &self.env {
            env.lookup(x) // 父环境继续查找
        } else {
            panic!("undefined variable")
        }
    }

    /// 新建变量
    pub fn bind(&mut self, var: T, val: H) -> Option<H> {
        // insert 进去的东西被上面写法拿出以后解引用，所有权怎么说？
        self.map.insert(var, val)
    }

    /// 将 table 作为 map 的父环境集合
    /// this is a static method
    /// table 传引用进来，不占用所有权，内部使用拷贝
    pub fn extended(map: HashMap<T, H>, table: &Rc<SymTable<T, H>>) -> Self {
        SymTable {
            map,
            env: Some(Rc::clone(table)),
        }
    }
}
