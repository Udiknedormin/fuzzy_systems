//! Fuzzy operations sets-related traits, structs and macros.
//!
//! # Traits
//!
//! Systems traits are used to mark ceratain classes of fuzzy systems based
//! on their characteristics.

#[macro_use]
mod macros;
mod traits;
mod list;
#[cfg(test)]
mod test;

pub use self::macros::*;
pub use self::traits::*;
pub use self::list::*;
