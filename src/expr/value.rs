use std::fmt::{Debug, Display, Formatter, Result as FmtResult};
use std::marker::PhantomData;

use super::*;

use crate::opset::Opset;
use crate::value::{Membership, Raw};
use crate::impl_fuzzy_expr_ops;


/// Fuzzy value expression.
#[derive(Clone, Copy)]
pub struct ExprValue<S: Opset> {
    membership: Membership<S>,
    phantom: PhantomData<S>
}

impl<S: Opset> ExprValue<S> {
    /// Creates from raw value. Panics if it's not a valid membership.
    pub fn new(raw: Raw) -> Self {
        Self {
            membership: Membership::new(raw),
            phantom: PhantomData
        }
    }

    /// Creates from raw value.
    pub fn try_new(raw: Raw) -> Option<Self> {
        Membership::try_new(raw).map(|raw| {
            Self {
                membership: raw,
                phantom: PhantomData
            }
        })
    }

    /// Creates from valid membership.
    pub fn from_membership(membership: Membership<S>) -> Self {
        Self {
            membership,
            phantom: PhantomData
        }
    }

    /// Changes into tagged value.
    pub fn with_tag<T>(self, tag: T) -> ExprTagged<S, T> {
        ExprTagged::from_unnamed(self, tag)
    }
}

impl<S: Opset> Debug for ExprValue<S> {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "ExprValue({:?})", self.membership.to_string())
    }
}

impl<S: Opset> Display for ExprValue<S> {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "{}", self.membership.to_string())
    }
}

impl<S: Opset> Expr<S> for ExprValue<S> {
    #[inline]
    fn to_value(&self) -> Membership<S> {
        self.membership
    }
}

impl_fuzzy_expr_ops! {
    ExprValue<S>
}
