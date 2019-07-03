use super::*;

use crate::value::Membership;


#[test]
fn and_less_eq_or() {
    macro_rules! test {
        ($name:ident) => {
            let x = Membership::<YagerInf>::new(0.5);
            let y = Membership::new(0.3);
            assert!((x & y).as_raw() <= (x | y).as_raw());
        };
        ($name:ident, $($names:ident),*) => {
            test!($name);
            test!($($names)*);
        }
    }

    test!(Yager1, YagerInf);
    let x = Membership::<YagerInf>::new(0.5);
    let y = Membership::new(0.3);
    let x_and_y = (x & y).as_raw();
    let x_or_y  = (x | y).as_raw();
    assert!(0.0 <= x_and_y                 );
    assert!(       x_and_y <= x_or_y       );
    assert!(                  x_or_y <= 1.0);
}
