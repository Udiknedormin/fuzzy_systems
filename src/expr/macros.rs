/// Macro for easy and generic fuzzy math.
///
/// Works with trait objects:
///
/// ```
/// # use fuzzy_systems::{Opset, Expr, Hamacher1, fuzzy_math};
/// fn do_or<S, L, R>(lhs: L, rhs: R) -> impl Expr<S>
/// where
///     S: Opset,
///     L: Expr<S>,
///     R: Expr<S>
/// {
///     fuzzy_math!(lhs | rhs)
///     // expands to: lhs.or(rhs)
///     // without macro, the following fails: lhs | rhs
/// }
/// ```
///
/// Enables operator syntax in generic scenarios.
#[macro_export]
macro_rules! fuzzy_math {
    // ident
    ($a:ident) => {
        $a
    };


    // `not`
    (!$a:tt $($rest:tt)*) => {
        fuzzy_math! {
            (fuzzy_math!($a.not()))
            $($rest)*
        }
    };
    

    // `or`
    ($a:tt | !$b:tt & $c:tt $($rest:tt)*) => {
        fuzzy_math! {
            $a | (fuzzy_math!($b).not().and(fuzzy_math!($c)))
            $($rest)*
        }
    };

    ($a:tt | $b:tt & !$c:tt $($rest:tt)*) => {
        fuzzy_math! {
            $a | (fuzzy_math!($b).and(fuzzy_math!($c).not()))
            $($rest)*
        }
    };

    ($a:tt | $b:tt & $c:tt $($rest:tt)*) => {
        fuzzy_math! {
            $a | (fuzzy_math!($b).and(fuzzy_math!($c)))
            $($rest)*
        }
    };


    ($a:tt | !$b:tt $($rest:tt)*) => {
        fuzzy_math! {
            (fuzzy_math!($a).or(fuzzy_math!($b).not()))
            $($rest)*
        }
    };

    ($a:tt | $b:tt $($rest:tt)*) => {
        fuzzy_math! {
            (fuzzy_math!($a).or(fuzzy_math!($b)))
            $($rest)*
        }
    };


    // `and`
    ($a:tt & !$b:tt $($rest:tt)*) => {
        fuzzy_math! {
            (fuzzy_math!($a).and(fuzzy_math!($b).not()))
            $($rest)*
        }
    };

    ($a:tt & $b:tt $($rest:tt)*) => {
        fuzzy_math! {
            (fuzzy_math!($a).and(fuzzy_math!($b)))
            $($rest)*
        }
    };

    // parens
    (($($a:tt)*)) => {
        fuzzy_math!($($a)*)
    };

    // recurse
    ($($a:tt)*) => {
        $($a)*
    }
}

#[doc(hidden)]
#[macro_export]
macro_rules! impl_fuzzy_expr_ops {
    ($typ:ident<$S:ident $(, $params:ident)*>) => {
        impl<$S $(, $params)*> std::ops::Not for $typ<$S $(, $params)*>
        where
            $S: Opset,
            $($params: Expr<$S>,)*
            Self: Sized
        {
            type Output = $crate::expr::ExprNot<S, Self>;
            fn not(self) -> Self::Output {
                Expr::not(self)
            }
        }

        impl<$S $(, $params)*, __R> std::ops::BitAnd<__R> for $typ<$S $(, $params)*>
        where
            $S: Opset,
            $($params: Expr<$S>,)*
            __R: Expr<S>,
            Self: Sized
        {
            type Output = $crate::expr::ExprAnd<S, Self, __R>;
            fn bitand(self, rhs: __R) -> Self::Output {
                Expr::and(self, rhs)
            }
        }

        impl<$S $(, $params)*, __R> std::ops::BitOr<__R> for $typ<$S $(, $params)*>
        where
            $S: Opset,
            $($params: Expr<$S>,)*
            __R: Expr<S>,
            Self: Sized
        {
            type Output = $crate::expr::ExprOr<S, Self, __R>;
            fn bitor(self, rhs: __R) -> Self::Output {
                Expr::or(self, rhs)
            }
        }
    };
}
