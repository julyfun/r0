use std::io::{self, Write};

pub fn readint() -> i64 {
    print!("input an integer: ");
    io::stdout().flush().expect("flush failed");
    let mut v = String::new();
    io::stdin().read_line(&mut v).expect("read failed");
    v.trim().parse().expect("not an integer!")
}

pub fn is_digit(x: &str) -> bool {
    x.chars().all(|x| x.is_digit(10))
}

/// private
trait HasDigit {
    fn has_digit(&self) -> bool;
}

impl HasDigit for str {
    fn has_digit(&self) -> bool {
        self.chars().any(|x| x.is_digit(10))
    }
}

pub fn is_valid_var_name(s: &str) -> bool {
    // only contains ...
    s.chars()
        .all(|x| x.is_ascii_digit() || x.is_ascii_alphabetic() || x == '_')
        && !s.chars().next().unwrap().is_ascii_digit()
}

#[test]
fn test1() {
    assert!(is_valid_var_name("xyz"));
    assert!(is_valid_var_name("x1"));
    assert!(is_valid_var_name("_x_X_12"));
    assert!(!is_valid_var_name("1x"));
    assert!(!is_valid_var_name("123"));
}

#[macro_export]
macro_rules! string {
    ($x: expr) => {
        String::from($x)
    };
}

#[macro_export]
macro_rules! hashmap {
    ($($key: expr => $val: expr), *) => {
        {
            let mut map = std::collections::HashMap::new();
            $(map.insert($key, $val);)*
            map
        }
    };
}
