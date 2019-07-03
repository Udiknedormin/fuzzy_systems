
/// Fuzzy opset creation macro.
/// Implements not(x), and(a,b), or(a,b) functions
/// in any of the two notations:
///   * function notation:  not(x) { ... }  and(a,b) { ... }
///   * operator notation:  ~x = ...;  a & b = ...;
#[macro_export]
macro_rules! fuzzy_opset {
	( $( $spec:tt )* ) => {
		parse_fuzzy_opset! {
			meta: [],
			spec: $( $spec )*
		}
	}
}

#[doc(hidden)]
#[macro_export]
macro_rules! parse_fuzzy_opset {
    // Empty parse data.
    ( meta: [ ], spec: ) => {};

    // Parse meta-data...
    (
        meta: [ $( #[$metas:meta] )* ],
        spec: #[$n_meta:meta] $( $tts:tt )+
    ) => {
        parse_fuzzy_opset! {
            meta: [ $( #[$metas] )* #[$n_meta] ],
            spec: $( $tts )+
        }
    };

	// Parse name and public visibility...
    (
        meta: [ $( #[$metas:meta] )* ],
        spec: pub $name:ident $( $rest:tt )*
    ) => {
        parse_fuzzy_opset! {
            vis:  [ pub ],
            meta: [ $( #[$metas] )* ],
			name: [ $name ],
			spec: $( $rest )*
        }
	};

	// Parse name and private visibility...
    (
        meta: [ $( #[$metas:meta] )* ],
        spec: $name:ident $( $rest:tt )*
    ) => {
        parse_fuzzy_opset! {
            vis:  [ ],
            meta: [ $( #[$metas] )* ],
			name: [ $name ],
            spec: $($rest)*
        }
	};

    // Parse inner
    (
		vis:  [ $( $vis:ident )* ],
        meta: [ $( #[$metas:meta] )* ],
        name: [ $name:ident ],
        spec: {
            $($inner:tt)*
        }
        $($rest:tt)*
    ) => {
        parse_fuzzy_opset! {
            vis:  [ $($vis)* ],
            meta: [ $( #[$metas] )* ],
			name: [ $name ],
            not: [] () {},
            or:  [] () {},
            and: [] () {},
            inner: {
                $($inner)*
            }
        }

        parse_fuzzy_opset! {
            meta: [ ],
            spec: $( $rest )*
        }
    };

    // Parse `not` short form.
    (
		vis:  [ $( $vis:ident )* ],
        meta: [ $( #[$metas:meta] )* ],
        name: [ $name:ident ],
        not: [] () {},
        or:  [ $(#[$ormet:meta])*  ] ( $($orarg:ident),*  ) { $($or:tt)*  },
        and: [ $(#[$andmet:meta])* ] ( $($andarg:ident),* ) { $($and:tt)* },
        inner: {
            $(#[$met:meta])*
            ~$arg:ident = $impl:expr;
            $($inner:tt)*
        }
    ) => {
        parse_fuzzy_opset! {
            vis:  [ $($vis)* ],
            meta: [ $( #[$metas] )* ],
			name: [ $name ],
            not: [ $(#[$met])* ] ( $arg ) { $impl },
            or:  [ $(#[$ormet])*  ] ( $($orarg),*  ) { $($or)*  },
            and: [ $(#[$andmet])* ] ( $($andarg),* ) { $($and)* },
            inner: {
                $($inner)*
            }
        }
    };

    // Parse `not` long form.
    (
		vis:  [ $( $vis:ident )* ],
        meta: [ $( #[$metas:meta] )* ],
        name: [ $name:ident ],
        not: [] () {},
        or:  [ $(#[$ormet:meta])*  ] ( $($orarg:ident),*  ) { $($or:tt)*  },
        and: [ $(#[$andmet:meta])* ] ( $($andarg:ident),* ) { $($and:tt)* },
        inner: {
            $(#[$met:meta])*
            not($arg:ident) { $($impl:tt)* }
            $($inner:tt)*
        }
    ) => {
        parse_fuzzy_opset! {
            vis:  [ $($vis)* ],
            meta: [ $( #[$metas] )* ],
			name: [ $name ],
            not: [ $(#[$met])* ] ( $arg ) { $($impl)* },
            or:  [ $(#[$ormet])*  ] ( $($orarg),*  ) { $($or)*  },
            and: [ $(#[$andmet])* ] ( $($andarg),* ) { $($and)* },
            inner: {
                $($inner)*
            }
        }
    };

    // Parse `or` short form.
    (
		vis:  [ $( $vis:ident )* ],
        meta: [ $( #[$metas:meta] )* ],
        name: [ $name:ident ],
        not: [ $(#[$notmet:meta])* ] ( $($notarg:ident)*  ) { $($not:tt)* },
        or:  [] () {},
        and: [ $(#[$andmet:meta])* ] ( $($andarg:ident),* ) { $($and:tt)* },
        inner: {
            $(#[$met:meta])*
            $arg1:ident | $arg2:ident = $impl:expr;
            $($inner:tt)*
        }
    ) => {
        parse_fuzzy_opset! {
            vis:  [ $($vis)* ],
            meta: [ $( #[$metas] )* ],
			name: [ $name ],
            not: [ $(#[$notmet])* ] ( $($notarg)*  ) { $($not)*  },
            or:  [ $(#[$met])* ] ( $arg1, $arg2 ) { $impl },
            and: [ $(#[$andmet])* ] ( $($andarg),* ) { $($and)*  },
            inner: {
                $($inner)*
            }
        }
    };

    // Parse `or` long form.
    (
		vis:  [ $( $vis:ident )* ],
        meta: [ $( #[$metas:meta] )* ],
        name: [ $name:ident ],
        not: [ $(#[$notmet:meta])* ] ( $($notarg:ident)*  ) { $($not:tt)* },
        or:  [] () {},
        and: [ $(#[$andmet:meta])* ] ( $($andarg:ident),* ) { $($and:tt)* },
        inner: {
            $(#[$met:meta])*
            or($arg1:ident, $arg2:ident) { $($impl:tt)* }
            $($inner:tt)*
        }
    ) => {
        parse_fuzzy_opset! {
            vis:  [ $($vis)* ],
            meta: [ $( #[$metas] )* ],
			name: [ $name ],
            not: [ $(#[$notmet])* ] ( $($notarg)*  ) { $($not)*  },
            or:  [ $(#[$met])* ] ( $arg1, $arg2 ) { $($impl)* },
            and: [ $(#[$andmet])* ] ( $($andarg),* ) { $($and)*  },
            inner: {
                $($inner)*
            }
        }
    };

    // Parse `and` short form.
    (
		vis:  [ $( $vis:ident )* ],
        meta: [ $( #[$metas:meta] )* ],
        name: [ $name:ident ],
        not: [ $(#[$notmet:meta])* ] ( $($notarg:ident)*  ) { $($not:tt)* },
        or:  [ $(#[$ormet:meta])*  ] ( $($orarg:ident),*  ) { $($or:tt)*  },
        and: [] () {},
        inner: {
            $(#[$met:meta])*
            $arg1:ident & $arg2:ident = $impl:expr;
            $($inner:tt)*
        }
    ) => {
        parse_fuzzy_opset! {
            vis:  [ $($vis)* ],
            meta: [ $( #[$metas] )* ],
			name: [ $name ],
            not: [ $(#[$notmet])* ] ( $($notarg)* ) { $($not)* },
            or:  [ $(#[$ormet])*  ] ( $($orarg),* ) { $($or)*  },
            and: [ $(#[$met])* ] ( $arg1, $arg2 ) { $impl },
            inner: {
                $($inner)*
            }
        }
    };

    // Parse `and` long form.
    (
		vis:  [ $( $vis:ident )* ],
        meta: [ $( #[$metas:meta] )* ],
        name: [ $name:ident ],
        not: [ $(#[$notmet:meta])* ] ( $($notarg:ident)*  ) { $($not:tt)* },
        or:  [ $(#[$ormet:meta])*  ] ( $($orarg:ident),*  ) { $($or:tt)*  },
        and: [] () {},
        inner: {
            $(#[$met:meta])*
            and($arg1:ident, $arg2:ident) { $($impl:tt)* }
            $($inner:tt)*
        }
    ) => {
        parse_fuzzy_opset! {
            vis:  [ $($vis)* ],
            meta: [ $( #[$metas] )* ],
			name: [ $name ],
            not: [ $(#[$notmet])* ] ( $($notarg)* ) { $($not)* },
            or:  [ $(#[$ormet])*  ] ( $($orarg),* ) { $($or)*  },
            and: [ $(#[$met])* ] ( $arg1, $arg2 ) { $($impl)* },
            inner: {
                $($inner)*
            }
        }
    };

    // Implement
    (
		vis:  [ $( $vis:ident )* ],
        meta: [ $( #[$metas:meta] )* ],
        name: [ $name:ident ],
        not: [ $(#[$notmet:meta])* ] ( $($notarg:ident)*  ) { $($not:tt)* },
        or:  [ $(#[$ormet:meta])*  ] ( $($orarg:ident),*  ) { $($or:tt)*  },
        and: [ $(#[$andmet:meta])* ] ( $($andarg:ident),* ) { $($and:tt)* },
        inner: {}
    ) => {
        impl_fuzzy_opset! {
            vis:  [ $($vis)* ],
            meta: [ $( #[$metas] )* ],
			name: [ $name ],
            not: [ $(#[$notmet])* ] ( $($notarg)*  ) { $($not)* },
            or:  [ $(#[$ormet])*  ] ( $($orarg),*  ) { $($or)*  },
            and: [ $(#[$andmet])* ] ( $($andarg),* ) { $($and)* }
        }
    }
}

#[doc(hidden)]
#[macro_export]
macro_rules! impl_fuzzy_opset {
    (
		vis:  [ $( $vis:ident )* ],
        meta: [ $( #[$metas:meta] )* ],
        name: [ $name:ident ],
        not:
            [ $(#[$notmet:meta])* ]
            ( $not1:ident )
            { $($not:tt)* },
        or:
            [ $(#[$ormet:meta])*  ]
            ( $or1:ident, $or2:ident )
            { $($or:tt)*  },
        and:
            [ $(#[$andmet:meta])* ]
            ( $and1:ident, $and2:ident )
            { $($and:tt)* }
    ) => {
        $( #[ $metas ] )*
        #[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Debug)]
        $($vis)* struct $name {}

        impl Opset for $name {
            $( #[$notmet] )*
            fn not($not1: Membership<Self>) -> Membership<Self> {
                let $not1 = $not1.as_raw();
                Membership::unchecked_new($($not)*)
            }

            $( #[$ormet] )*
            fn or($or1: Membership<Self>, $or2: Membership<Self>)
                -> Membership<Self> {

                let $or1 = $or1.as_raw();
                let $or2 = $or2.as_raw();
                Membership::unchecked_new($($or)*)
            }

            $( #[$andmet] )*
            fn and($and1: Membership<Self>, $and2: Membership<Self>)
                -> Membership<Self> {

                let $and1 = $and1.as_raw();
                let $and2 = $and2.as_raw();
                Membership::unchecked_new($($and)*)
            }
        }
    }
}
