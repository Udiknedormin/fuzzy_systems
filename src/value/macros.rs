#[macro_export]
macro_rules! impl_fuzzy_ops {
    ($name:ident<$p:ident>) => {
        impl<$p: $crate::Opset> std::ops::Not for $name<$p> {
            type Output = Membership<S>;

            fn not(self) -> Membership<S> {
                $p::negation(self.membership())
            }
        }

        impl<$p, B> std::ops::BitOr<B> for $name<$p>
        where $p: $crate::Opset,
              B: $crate::Value<$p> {

            type Output = $crate::Membership<S>;

            fn bitor(self, rhs: B) -> $crate::Membership<S> {
                $p::alternative(self.membership(), rhs.membership())
            }
        }

        impl<$p, B> std::ops::BitAnd<B> for $name<$p>
        where $p: $crate::Opset,
              B: $crate::Value<$p> {

            type Output = $crate::Membership<S>;

            fn bitand(self, rhs: B) -> $crate::Membership<S> {
                $p::conjunction(self.membership(), rhs.membership())
            }
        }
    }
}
