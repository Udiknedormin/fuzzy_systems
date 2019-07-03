use super::Membership;

use crate::opset::Opset;
use crate::expr::ExprValue;


/// Any fuzzy value of a certain operation set.
pub trait Value<S: Opset>: PartialEq + PartialOrd {
    /// Get fuzzy membership value.
    fn membership(&self) -> Membership<S>;

    /// Get fuzzy membership expression.
    fn to_expr(&self) -> ExprValue<S> {
        ExprValue::from_membership(self.membership())
    }
}

/// Implement shortcuts for fuzzy operations (not, and, or).
///
/// ```
/// # use std::marker::PhantomData;
/// # use fuzzy_systems::{
/// #  impl_fuzzy_ops, Membership, Opset, Value, Yager1
/// # };
/// #[derive(PartialOrd, PartialEq)]
/// struct FuzzyF32<S: Opset> {
///     value: f32,
///     phantom: PhantomData<S>
/// }
///
/// impl<S: Opset> FuzzyF32<S> {
///     fn new(value: f32) -> Self {
///         Self {
///             value,
///             phantom: PhantomData
///         }
///     }
/// }
///
/// impl<S: Opset> Value<S> for FuzzyF32<S> {
///     fn membership(&self) -> Membership<S> {
///         Membership::new(self.value as f64)
///     }
/// }
///
/// impl_fuzzy_ops!(FuzzyF32<S> with S);
/// let a = FuzzyF32::<Yager1>::new(0.1);
/// let b = FuzzyF32::<Yager1>::new(0.5);
/// let c = a | b;
/// assert!((c.as_raw() - 0.6).abs() < 0.001);
/// ```
#[macro_export]
macro_rules! impl_fuzzy_ops {
    ($name:ident<$($params:ident),*> with $p:ident) => {
        impl<$p: $crate::Opset> std::ops::Not for $name<$($params),*> {
            type Output = Membership<S>;

            fn not(self) -> Membership<S> {
                $p::not(self.membership())
            }
        }

        impl<$p, B> std::ops::BitOr<B> for $name<$($params),*>
        where $p: $crate::Opset,
              B: $crate::Value<$p> {

            type Output = $crate::Membership<S>;

            fn bitor(self, rhs: B) -> $crate::Membership<S> {
                $p::or(self.membership(), rhs.membership())
            }
        }

        impl<$p, B> std::ops::BitAnd<B> for $name<$($params),*>
        where $p: $crate::Opset,
              B: $crate::Value<$p> {

            type Output = $crate::Membership<S>;

            fn bitand(self, rhs: B) -> $crate::Membership<S> {
                $p::and(self.membership(), rhs.membership())
            }
        }
    }
}
