use std::marker::PhantomData;
use std::string::ToString;

use super::Expr;

use crate::opset::Opset;
use crate::value::Membership;
use crate::impl_fuzzy_expr_ops;


/// Fuzzy expression for runtime choice of path.
///
/// Can be created via `as_left()` and `as_right` methods
/// on `Expr`.
///
/// Example:
/// ```
/// # use fuzzy_systems::{Expr, Hamacher1};
/// # let flag = || { true };
/// # let a = Expr::<Hamacher1>::new(0.4);
/// # let b = Expr::new(0.6);
/// # let c = Expr::new(0.4);
/// let d = if flag() {
///     (a | b).as_left()
/// } else {
///     (a & c).as_right()
/// };
/// // won't compile without ExprEither:
/// //   let d = if flag() {
/// //      (a | b)
/// //   } else {
/// //      (a & c)
/// //   };
/// ```
pub enum ExprEither<S, L, R> {
    Left(L),
    Right(R),
    Never(!, PhantomData<S>)
}

impl<S, L, R> ExprEither<S, L, R> {
    pub fn left(val: L) -> Self {
        ExprEither::Left(val)
    }

    pub fn right(val: R) -> Self {
        ExprEither::Right(val)
    }
}

impl<S, L, R> Clone for ExprEither<S, L, R>
where
    L: Clone,
    R: Clone
{
    fn clone(&self) -> Self {
        match self {
            ExprEither::Left(lhs) => Self::left(lhs.clone()),
            ExprEither::Right(rhs) => Self::right(rhs.clone()),
            ExprEither::Never(..) => panic!("Invalid state!")
        }
    }
}

impl<S, L, R> ToString for ExprEither<S, L, R>
where
    L: ToString,
    R: ToString
{
    fn to_string(&self) -> String {
        match self {
            ExprEither::Left(e) => e.to_string(),
            ExprEither::Right(e) => e.to_string(),
            ExprEither::Never(..) => panic!("Invalid state!")
        }
    }
}

impl<S, L, R> Expr<S> for ExprEither<S, L, R>
where
    S: Opset,
    L: Expr<S>,
    R: Expr<S>
{
    #[inline]
    fn to_value(&self) -> Membership<S> {
        match self {
            ExprEither::Left(e) => e.to_value(),
            ExprEither::Right(e) => e.to_value(),
            ExprEither::Never(..) => panic!("Invalid state!")
        }
    }
}

impl_fuzzy_expr_ops! {
    ExprEither<S, L, R>
}
