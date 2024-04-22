use std::io::{self, Write};

pub fn readint() -> i64 {
    print!("input an integer: ");
    io::stdout().flush().expect("flush failed");
    let mut v = String::new();
    io::stdin().read_line(&mut v).expect("read failed");
    v.trim().parse().expect("not an integer!")
}
