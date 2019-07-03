use std::fmt::{Debug, Display, Formatter, Result as FmtResult};
use std::marker::PhantomData;

use super::Expr;

use crate::opset::Opset;
use crate::value::Membership;
use crate::impl_fuzzy_expr_ops;


/// Fuzzy "and" expression.
pub struct ExprNot<S, V> {
    val: V,
    phantom: PhantomData<S>
}

impl<S, V> ExprNot<S, V> {
    pub fn new(val: V) -> Self {
        Self {
            val,
            phantom: PhantomData
        }
    }
}

impl<S, V> Clone for ExprNot<S, V>
where
    V: Clone
{
    fn clone(&self) -> Self {
        Self::new(self.val.clone())
    }
}

impl<S, V> Copy for ExprNot<S, V>
where
    V: Copy
{}

impl<S, V: Debug> Debug for ExprNot<S, V> {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "ExprNot({:?})", self.val)
    }
}

impl<S, V: Display> Display for ExprNot<S, V> {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "!{}", self.val)
    }
}

impl<S, V> Expr<S> for ExprNot<S, V>
where
    S: Opset,
    V: Expr<S>
{
    #[inline]
    fn to_value(&self) -> Membership<S> {
        !self.val.to_value()
    }
}

impl_fuzzy_expr_ops! {
    ExprNot<S, V>
}
