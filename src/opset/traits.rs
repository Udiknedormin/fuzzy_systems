use std::fmt::Debug;

use crate::value::{Membership, Raw};


/// Fuzzy operations set.
pub trait Opset: Sized + Clone + Copy + Ord + Eq + Debug {
    /// Fuzzy negation (`not` operator).
    ///
    /// ```rust
    /// # use fuzzy_systems::{Opset, Hamacher1};
    /// let a = Hamacher1::member(0.8);
    /// let b = !a;
    /// assert!((b.as_raw() - 0.2).abs() < 0.001);
    /// ```
    fn not(val: Membership<Self>) -> Membership<Self>;

    /// Fuzzy alternative (`or` operator).
    ///
    /// ```rust
    /// # use fuzzy_systems::{Opset, Hamacher1};
    /// let a = Hamacher1::member(0.8);
    /// let b = Hamacher1::member(0.3);
    /// let c = a | b;
    /// assert!((c.as_raw() - 0.86) < 0.001);
    /// ```
    fn or(lhs: Membership<Self>, rhs: Membership<Self>) -> Membership<Self>;

    /// Fuzzy conjunction (`and` operator).
    ///
    /// ```rust
    /// # use fuzzy_systems::{Opset, Hamacher1};
    /// let a = Hamacher1::member(0.8);
    /// let b = Hamacher1::member(0.3);
    /// let c = a & b;
    /// assert!((c.as_raw() - 0.24) < 0.001);
    /// ```
    fn and(lhs: Membership<Self>, rhs: Membership<Self>) -> Membership<Self>;

    /// Creater member.
    ///
    /// ```rust
    /// # use fuzzy_systems::{Membership, Opset, Hamacher1};
    /// let a = Hamacher1::member(0.1);
    /// let b = Membership::<Hamacher1>::new(0.1);
    /// assert_eq!(a, b);
    /// ```
    fn member(raw: Raw) -> Membership<Self> {
        Membership::new(raw)
    }
}


/// Fuzzy operations set all operations of which are differentiable.
pub trait OpsetDifferentiable: Opset {}

