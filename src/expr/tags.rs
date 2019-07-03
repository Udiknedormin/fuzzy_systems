//! Some example zero-cost tags.

/// Create a new statically-typed zero-cost tag.
macro_rules! new_tag {
    ($($v:vis $name:ident => $str:expr,)*) => {
        new_tag!($($v $name => $str),*);
    };

    ($($v:vis $name:ident => $str:expr),*) => {
        $(
            #[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
            $v struct $name;
            
            impl std::fmt::Display for $name {
                fn fmt(
                    &self,
                    fmt: &mut std::fmt::Formatter
                ) -> std::fmt::Result {
                    write!(fmt, "{}", $str)
                }
            }
        )*
    }
}

new_tag! {
    pub TagA => "a",
    pub TagB => "b",
    pub TagC => "c",
    pub TagD => "d",
    pub TagE => "e",
    pub TagF => "f",
    pub TagG => "g",
    pub TagH => "h",
    pub TagI => "i",
    pub TagJ => "j",
    pub TagK => "k",
    pub TagL => "l",
    pub TagM => "m",
    pub TagN => "n",
    pub TagO => "o",
    pub TagP => "p",
    pub TagQ => "q",
    pub TagR => "r",
    pub TagS => "s",
    pub TagT => "t",
    pub TagU => "u",
    pub TagV => "v",
    pub TagW => "w",
    pub TagX => "x",
    pub TagY => "y",
    pub TagZ => "z",
}
