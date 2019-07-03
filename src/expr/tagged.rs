use std::fmt::{Debug, Display, Formatter, Result as FmtResult};
use std::marker::PhantomData;
use std::ops::{Not, BitAnd, BitOr};

use super::*;

use crate::opset::Opset;
use crate::value::Membership;


/// Fuzzy value expression with custom tag.
pub struct ExprTagged<S: Opset, T> {
    membership: Membership<S>,
    tag: T,
    phantom: PhantomData<S>
}

impl<S: Opset, T> ExprTagged<S, T> {
    pub fn from_unnamed(val: ExprValue<S>, tag: T) -> Self {
        Self {
            membership: val.to_value(),
            tag,
            phantom: PhantomData
        }
    }

    pub fn from_membership(membership: Membership<S>, tag: T) -> Self {
        Self {
            membership,
            tag,
            phantom: PhantomData
        }
    }
}

impl<S: Opset, T> Clone for ExprTagged<S, T>
where
    T: Clone
{
    fn clone(&self) -> Self {
        Self::from_membership(self.membership, self.tag.clone())
    }
}

impl<S: Opset, T> Copy for ExprTagged<S, T>
where
    T: Copy
{}

impl<S: Opset, T: Debug> Debug for ExprTagged<S, T> {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "ExprTagged({:?}, {:?})", self.membership, self.tag)
    }
}

impl<S: Opset, T: Display> Display for ExprTagged<S, T> {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "{}", self.tag)
    }
}

impl<S: Opset, T> Expr<S> for ExprTagged<S, T> {
    #[inline]
    fn to_value(&self) -> Membership<S> {
        self.membership
    }
}

impl<S, T> Not for ExprTagged<S, T>
where
    S: Opset,
    Self: Sized
{
    type Output = ExprNot<S, Self>;
    fn not(self) -> Self::Output {
        Expr::not(self)
    }
}

impl<S, T, R> BitAnd<R> for ExprTagged<S, T>
where
    S: Opset,
    R: Expr<S>,
    Self: Sized
{
    type Output = ExprAnd<S, Self, R>;
    fn bitand(self, rhs: R) -> Self::Output {
        Expr::and(self, rhs)
    }
}

impl<S, T, R> BitOr<R> for ExprTagged<S, T>
where
    S: Opset,
    R: Expr<S>,
    Self: Sized
{
    type Output = ExprOr<S, Self, R>;
    fn bitor(self, rhs: R) -> Self::Output {
        Expr::or(self, rhs)
    }
}
