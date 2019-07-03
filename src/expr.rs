//! Fuzzy expressions that can be evaluated on-demand.
//!
//! Expressions are based on a statically-typed `Expr` trait implementors.
//! No `Box` allocations are used, `Expr` is a truly zero-cost abstraction.
//!
//!
//! ## Operators
//!
//! Each implementor, aside from methods provided by the trait, has handy
//! generic binary operators provided. The trait itself tries its best to be
//! transparent.
//!
//! Example:
//!
//! ```
//! # use fuzzy_systems::{Expr, Hamacher1};
//! let a = Expr::<Hamacher1>::new(0.1);
//! let b = Expr::new(0.6);
//! let c = Expr::new(0.4);
//! let d = (a | b) & !c;
//! // same as:  a.or(b).and(c.not())
//! ```
//!
//! Available operators:
//!
//! | operation   | method   | operator |
//! |-------------|----------|----------|
//! | negation    | a.not()  | !a       |
//! | alternation | a.or(b)  | a | b    |
//! | conjunction | a.and(b) | a & b    |
//!
//! Note: when passed generically, implentors of `Expr` cannot use
//! ergonomic operator syntax but can still call methods explicitly.
//! 
//! Operator syntax can be used generically via `fuzzy_math!` macro:
//!
//! ```
//! # use fuzzy_systems::{Opset, Expr, Hamacher1, fuzzy_math};
//! fn do_or<S, A, B, C>(a: A, b: B, c: C) -> impl Expr<S>
//! where
//!     S: Opset,
//!     A: Expr<S>,
//!     B: Expr<S>,
//!     C: Expr<S>
//! {
//!     fuzzy_math!(a | b & c )  // yields: (a | (b & c))
//! }
//! ```
//!
//!
//! ## Stringification
//!
//! Value expressions can be stringified with either their numerical value
//! or chosen name:
//!
//! ```
//! # use fuzzy_systems::{Expr, Hamacher1};
//! // with values:
//! # {
//! let a = Expr::<Hamacher1>::new(0.1);
//! let b = Expr::new(0.6);
//! let c = Expr::new(0.4);
//! let d = (a | b) & !c;
//! assert_eq!(d.to_string(), "((0.1 | 0.6) & !0.4)")
//! # }
//!
//! // with names:
//! # {
//! let a = Expr::<Hamacher1>::new(0.1).with_tag("a");
//! let b = Expr::new(0.6).with_tag("b");
//! let c = Expr::new(0.4).with_tag("c");
//! let d = (a | b) & !c;
//! assert_eq!(d.to_string(), "((a | b) & !c)")
//! # }
//!
//! // with zero-cost tags:
//! # {
//! use fuzzy_systems::tags::{TagA, TagB, TagC};
//! let a = Expr::<Hamacher1>::new(0.1).with_tag(TagA);
//! let b = Expr::new(0.6).with_tag(TagB);
//! let c = Expr::new(0.4).with_tag(TagC);
//! let d = (a | b) & !c;
//! assert_eq!(d.to_string(), "((a | b) & !c)")
//! # }
//! ```
//!
//! Strings are built on-demand and no additional data is stored to build
//! them, so this feature brings no overhead when not used.
//!
//! Note: when passed generically, implementors of `Expr` cannot be given
//! names as only `ExprValue` can.
//!
//!
//! ## Expr vs other traits
//! All standard implementors of `Expr` implement `Clone` (so `Expr<S> +
//! Clone` is fine), `Debug` and `Display`.
//! 
//! `Expr` itself doesn't inherit either of the abovementioned traits to
//! stay minimalistic and object-safe. Thanks to that, `Box<Expr<S>>`,
//! providing `to_value()`, is possible. `Box<Expr<S> + Display>` (or
//! `Debug`) and alike can provide other methods to the box.
//!
//!
//! ## Performance notes
//! `Expr` implementors' `to_value()` calls are inlined to boost
//! performance.

mod value;

mod tagged;
pub mod tags;

mod not;
mod or;
mod and;

mod either;

#[macro_use]
mod macros;

#[cfg(test)]
mod test;

pub use self::value::ExprValue;
pub use self::tagged::ExprTagged;
pub use self::either::ExprEither;

pub use self::macros::*;

use self::not::*;
use self::or::*;
use self::and::*;

use crate::opset::Opset;
use crate::value::{Membership, Raw};


// Can't inherit from '!', '&', '|' directly, as they are to be generic,
// but can implement them as aliases in any of the implementing structs.

/// Evaluable fuzzy expression.
pub trait Expr<S: Opset> {
    /// Expression's value.
    fn to_value(&self) -> Membership<S>;

    /// "Not" expression.
    fn not(self) -> ExprNot<S, Self>
    where
        Self: Sized
    {
        ExprNot::new(self)
    }

    /// "And" expression.
    fn and<R>(self, rhs: R) -> ExprAnd<S, Self, R>
    where
        Self: Sized,
        R: Expr<S>
    {
        ExprAnd::new(self, rhs)
    }

    /// "Or" expression.
    fn or<R>(self, rhs: R) -> ExprOr<S, Self, R>
    where
        Self: Sized,
        R: Expr<S>
    {
        ExprOr::new(self, rhs)
    }

    fn as_left<R>(self) -> ExprEither<S, Self, R>
    where
        Self: Sized,
        R: Expr<S>
    {
        ExprEither::left(self)
    }

    fn as_right<L>(self) -> ExprEither<S, L, Self>
    where
        Self: Sized,
        L: Expr<S>
    {
        ExprEither::right(self)
    }
}

impl<S: Opset> Expr<S> {
    pub fn new(raw: Raw) -> ExprValue<S> {
        ExprValue::new(raw)
    }

    pub fn try_new(raw: Raw) -> Option<ExprValue<S>> {
        ExprValue::try_new(raw)
    }

    pub fn from_membership(membership: Membership<S>) -> ExprValue<S> {
        ExprValue::from_membership(membership)
    }
}
