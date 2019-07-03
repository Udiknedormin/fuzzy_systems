
use std::fmt::{Debug, Display, Formatter, Result as fmtResult};
use std::ops::Deref;  // for Value impl
use std::default::Default;
use std::convert::From;
use std::marker::PhantomData;

use super::Value;

use crate::impl_fuzzy_ops;
use crate::opset::Opset;


/// Raw representation of a fuzzy membership value.
pub type Raw = f64;

/// Atomic fuzzy membership degree.
#[derive(Clone, Copy, PartialEq, PartialOrd)]
pub struct Membership<S: Opset>(Raw, PhantomData<S>);

impl<S: Opset> Default for Membership<S>
where Raw: Default {
    fn default() -> Self {
        Membership::new(Raw::default())
    }
}

impl<S: Opset> From<Raw> for Membership<S> {
    fn from(val: Raw) -> Self {
        Membership::new(val)
    }
}

impl<S: Opset> Membership<S> {
    /// Checks whether the precondition of the construction is fulfilled.
    fn precondition(val: Raw) -> bool {
        val >= Self::MIN_VALUE && val <= Self::MAX_VALUE
    }

    /// Minimum valid value.
    const MIN_VALUE: Raw = 0.0;

    /// Maximum valid value.
    const MAX_VALUE: Raw = 1.0;

    /// Creates a membership value based on raw numerical value.
    /// Panics if the value is not between 0.0 and 1.0 inclusive.
    pub fn new(val: Raw) -> Self {
        assert!(Self::precondition(val));
        Self::unchecked_new(val)
    }

    /// Creates a membership value based on raw numerical value.
    /// Does not check anything. Use only for cases where precondition
    /// is proved to be true.
    pub fn unchecked_new(val: Raw) -> Self {
        Membership(val, PhantomData)
    }

    /// Creates a membership value based on raw numerical value.
    pub fn try_new(val: Raw) -> Option<Self> {
        if Self::precondition(val) {
            Some(Self::unchecked_new(val))
        }
        else {
            None
        }
    }

    /// Creates a membership value based on raw numerical value.
    /// Panics if the value is not between 0.0 and 1.0 inclusive.
    pub fn with_fit(val: Raw) -> Self {
        assert!(!val.is_nan());
        let val = val.max(Self::MIN_VALUE).min(Self::MAX_VALUE);
        Self::unchecked_new(val)
    }

    /// Raw numerical value of the membership value.
    pub fn as_raw(self) -> Raw {
        self.0
    }
}

impl<S: Opset> Deref for Membership<S> {
    type Target = Raw;

    fn deref(&self) -> &Raw {
        &self.0
    }
}

impl<S: Opset> Value<S> for Membership<S> {
    fn membership(&self) -> Membership<S> {
        *self
    }
}

impl_fuzzy_ops!(Membership<S> with S);


#[test]
fn membership_ops() {
    use crate::opset::YagerInf;

    let x = Membership::<YagerInf>::new(0.5);
    let y = Membership::new(0.3);
    assert_eq!(   (!y).as_raw(), 0.7);
    assert_eq!((x & y).as_raw(), 0.3);
    assert_eq!((x | y).as_raw(), 0.5);
}


impl<S: Opset> Debug for Membership<S> {
    fn fmt(&self, fmt: &mut Formatter) -> fmtResult {
        fmt.debug_tuple("Membership")
            .field(&self.0)
            .finish()
    }
}

impl<S: Opset> Display for Membership<S> {
    fn fmt(&self, fmt: &mut Formatter) -> fmtResult {
        write!(fmt, "{}", self.0)
    }
}
