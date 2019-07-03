use std::marker::PhantomData;

use super::Opset;

use crate::value::{Membership, Value};


/// Fuzzy operations set marker, used for raw float calls.
pub struct OpsetMarker<S: Opset> {
    phantom: PhantomData<S>
}

impl<S: Opset> OpsetMarker<S> {
    /// Create new marker.
    pub fn new() -> Self {
        Self {
            phantom: PhantomData
        }
    }

    /// Fuzzy negation (`not` operator).
    ///
    /// ```rust
    /// # use fuzzy_systems::{Opset, Yager1};
    /// let marker = Yager1::marker();
    /// marker.not(0.25);
    /// ```
    pub fn not<M>(&self, val: M) -> Membership<S>
    where
        M: Value<S>
    {
        S::not(val.membership())
    }

    /// Fuzzy alternative (`or` operator).
    pub fn or<L, R>(&self, lhs: L, rhs: R) -> Membership<S>
    where
        L: Value<S>,
        R: Value<S>
    {
        S::or(lhs.membership(), rhs.membership())
    }

    /// Fuzzy conjunction (`and` operator).
    pub fn and<L, R>(&self, lhs: L, rhs: R) -> Membership<S>
    where
        L: Value<S>,
        R: Value<S>
    {
        S::and(lhs.membership(), rhs.membership())
    }

}
