//! An option type that is always `Some` or `None`, depending on a static type parameter.
//!
//! # Example
//!
//! ```rust
//!
//! use static_option::*;
//!
//! enum Inner<Selector: StaticOptionSelector> {
//!     A(StaticOption<Selector, i32>),
//!     B(StaticOption<Selector, u32>),
//! }
//!
//! enum MaybeInitialised {
//!     Initialised(Inner<StaticOptionSome>),
//!     Uninitialised(Inner<StaticOptionNone>),
//! }
//!
//! ```

use core::fmt::Debug;

/// The static option type.
/// `Selector` is either `StaticOptionSome` or `StaticOptionNone`.
/// `Type` is the inner type of the option, like `Option<Type>` in the standard library.
pub type StaticOption<Selector, Type> = <Selector as StaticOptionSelector>::Selected<Type>;

/// A static option selector for `Some` or `None`.
/// It is sealed and only implemented by `StaticOptionSome` and `StaticOptionNone`.
pub trait StaticOptionSelector: sealed::Sealed {
    type Selected<T: Debug + Clone>: Debug + Clone;
}

/// Type indicating that a static option is `Some`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct StaticOptionSome;

/// Type indicating that a static option is `None`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct StaticOptionNone;

impl StaticOptionSelector for StaticOptionSome {
    type Selected<T: Debug + Clone> = T;
}

impl StaticOptionSelector for StaticOptionNone {
    type Selected<T: Debug + Clone> = StaticOptionNone;
}

impl sealed::Sealed for StaticOptionSome {}

impl sealed::Sealed for StaticOptionNone {}

mod sealed {
    pub trait Sealed {}
}
