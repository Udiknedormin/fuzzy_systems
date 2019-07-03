//! # fuzzy_systems
//!
//! Low-level fuzzy systems toolbox with statically-typed operation systems,
//! values, expressions and memberships.
//!
//! `fuzzy_systems` is especially useful when multiple fuzzy operation
//! systems are used in different parts of the program or a thin layer
//! of zero-cost abstraction is needed for raw performance (could be used
//! for HPC).
//!
//!
//! ## Example
//! ```rust
//! use fuzzy_systems::{Hamacher1, Opset, Membership, Value, Expr};
//!
//! // Take crisp Rust structure:
//! #[derive(PartialOrd, PartialEq)]
//! enum ThreatLevel {
//!     VeryLow,
//!     Low,
//!     Medium,
//!     High,
//!     Critical
//! }
//!
//! // Implement fuzzyfication:
//! impl<S: Opset> Value<S> for ThreatLevel {
//!     fn membership(&self) -> Membership<S> {
//!         use ThreatLevel::*;
//!         let raw = match self {
//!             VeryLow => 0.0,
//!             Low => 0.2,
//!             Medium => 0.5,
//!             High => 0.7,
//!             Critical => 1.0
//!         };
//!         Membership::new(raw)
//!     }
//! }
//!
//! let crisp_a = ThreatLevel::Medium;
//! let crisp_b = ThreatLevel::High;
//!
//! // Do fuzzy operations:
//! let a = crisp_a.membership();
//! let b = crisp_b.membership();
//! let ar = a.as_raw();
//! let br = b.as_raw();
//!
//! let c: Membership<Hamacher1> = (a | b);
//! let cr = c.as_raw();
//!
//! assert!(cr > ar);
//! assert!(cr > br);
//! // ThreatLevel "magically" rises when threats are cumulated!
//!
//! // Can build fuzzy expressions too:
//! let d = {
//!     let a = crisp_a.to_expr().with_tag("medium");
//!     let b = crisp_b.to_expr().with_tag("high");
//!     a | b
//! };
//! assert_eq!(d.to_string(), "(medium | high)");
//! assert_eq!(d.to_value(), c);
//! ```
//!
//!
//! ## What makes it different
//! There are a few other fuzzy systems libraries out there,
//! particularly
//! [rsfuzzy](https://github.com/auseckas/rsfuzzy),
//! [fuzzy_logic](https://github.com/KineticCookie/fuzzy_logic)
//! and
//! [fuzzy-reasoning-mamdani](https://github.com/thokjo12/fuzzy-reasoning-mamdani),
//! each with their own respective advantages.
//! 
//! ### Compared to all
//!  * all others are rule engines, `fuzzy_systems` is more
//!    low-level
//!  * `rsfuzzy` and `fuzzy_logic` use string-identified variables,
//!    `fuzzy-reasoning-mamdani` uses strongly-typed domains with
//!    string-identified states,
//!    `fuzzy_systems` has no concept of a variable (although a value can
//!    be tagged with string name)
//!  * all others can fail in runtime due to non-existing
//!    variables or states, `fuzzy_systems` does its transformations directly
//!    on Rust struct so it can't fail
//!  * all others operate directly on membership values,
//!    `fuzzy_systems` can assign membership to any Rust value (via
//!    `Value<S>` trait)
//!  * `rsfuzzy` and `fuzzy-reasoning-mamdani` have fixed fuzzy operation
//!     system, `fuzzy_logic` and `fuzzy_systems` enable user to pick one as
//!     well as create a custom one
//! 
//! ### Compared to rsfuzzy
//!  * `rsfuzzy` does string rule parsing, `fuzzy_systems.rs` does not
//!  * `rsfuzzy` has fixed fuzzy operation system, `fuzzy_systems` enables user
//!    to pick one of many as well as create a custom one
//! 
//! ### Compared to fuzzy_logic
//!  * `fuzzy_logic` defines sets and sets operations, `fuzzy_systems`
//!    does not
//!  * `fuzzy_logic` binds operation sets to a rule, `fuzzy_systems`
//!    binds it directly to expression
//!  * `fuzzy_logic` doesn't provide easy-to-use fuzzy expressions creation
//!    methods, `fuzzy_systems` does (via methods, binary operators and
//!    `fuzzy_math!` macro)
//!  * `fuzzy_logic` heap-allocates (in `Box`, `HashMap` etc),
//!    `fuzzy_systems` does not
//!  * `fuzzy_logic` caches results, `fuzzy_systems` does not
//!  * `fuzzy_logic` uses `f32`, `fuzzy_systems` uses `f64`
//! 
//! ### Compared to fuzzy-reasoning-mamdani
//!  * both are strongly-typed
//!  * `fuzzy-reasoning-mamdani` uses `(bool, f64)` for fuzzy values,
//!    `fuzzy_systems` uses `f64`

#![cfg_attr(feature="clippy", feature(plugin))]
#![cfg_attr(feature="clippy", plugin(clippy))]

#![feature(fn_traits)] 
#![feature(never_type)]
#![feature(trait_alias)]


pub mod expr;
pub mod opset;
mod value;

pub use self::expr::*;
pub use self::expr::tags;

pub use self::opset::*;

pub use self::value::*;
