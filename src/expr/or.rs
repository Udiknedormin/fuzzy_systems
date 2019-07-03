use std::fmt::{Debug, Display, Formatter, Result as FmtResult};
use std::marker::PhantomData;

use super::Expr;

use crate::opset::Opset;
use crate::value::Membership;
use crate::impl_fuzzy_expr_ops;


/// Fuzzy "or" expression.
pub struct ExprOr<S, L, R> {
    lhs: L,
    rhs: R,
    phantom: PhantomData<S>
}

impl<S, L, R> ExprOr<S, L, R> {
    pub fn new(lhs: L, rhs: R) -> Self {
        Self {
            lhs,
            rhs,
            phantom: PhantomData
        }
    }
}

impl<S, L, R> Clone for ExprOr<S, L, R>
where
    L: Clone,
    R: Clone
{
    fn clone(&self) -> Self {
        Self::new(self.lhs.clone(), self.rhs.clone())
    }
}

impl<S, L, R> Copy for ExprOr<S, L, R>
where
    L: Copy,
    R: Copy
{}

impl<S, L: Debug, R: Debug> Debug for ExprOr<S, L, R> {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "ExprOr({:?}, {:?})", self.lhs, self.rhs)
    }
}

impl<S, L: Display, R: Display> Display for ExprOr<S, L, R> {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "({} | {})", self.lhs, self.rhs)
    }
}

impl<S, L, R> Expr<S> for ExprOr<S, L, R>
where
    S: Opset,
    L: Expr<S>,
    R: Expr<S>
{
    #[inline]
    fn to_value(&self) -> Membership<S> {
        let lhs = self.lhs.to_value();
        let rhs = self.rhs.to_value();
        lhs | rhs
    }
}

impl_fuzzy_expr_ops! {
    ExprOr<S, L, R>
}
