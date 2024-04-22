use crate::*;

#[test]
fn test_r0() {
    let p3 = Prim2("+".to_string(), Box::new(Int(10)), Box::new(Int(32)));
    let r = interp_exp(p3);
    assert_eq!(r, 42);
}
