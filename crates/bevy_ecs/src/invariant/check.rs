//! Compile-time and startup-time verification traits for Schema constraints.
//!
//! # Verification Strategy
//!
//! 1. **Compile-time**: Basic type compatibility via trait bounds
//! 2. **Startup-time**: Full constraint validation during Bundle registration
//!    (via `component_ids` - happens at first spawn, not mid-execution)
//!
//! The startup-time check provides a practical "compile-time-like" guarantee:
//! invalid Schema configurations are caught immediately when the program starts,
//! not during gameplay.

use crate::bundle::Bundle;
use crate::component::Component;

use super::Schema;

/// Marker trait: Bundle `B` contains component `C`.
///
/// This trait enables compile-time verification that a bundle contains
/// a specific component. Manual implementation is required for complex cases.
pub trait Contains<C: Component>: Bundle {}

/// Marker trait: Bundle `B` does NOT contain component `C`.
///
/// Due to Rust's lack of negative trait bounds, this must be
/// implemented explicitly.
pub trait NotContains<C: Component>: Bundle {}

/// Marker trait: Bundle `B` contains all components in `R`.
pub trait Satisfies<R: Bundle>: Bundle {}

/// Marker trait: Bundle `B` has no overlap with `F`.
pub trait DisjointFrom<F: Bundle>: Bundle {}

/// Marker trait: Bundle `B` is compatible with Schema `S`.
///
/// A bundle is compatible if it:
/// 1. Contains all components in `S::Require`
/// 2. Has no overlap with `S::Forbid`
pub trait CompatibleWith<S: Schema>: Bundle {}

// A component contains itself
impl<C: Component> Contains<C> for C {}

// Any bundle satisfies empty requirement
impl<B: Bundle> Satisfies<()> for B {}

// Any bundle is disjoint from empty
impl<B: Bundle> DisjointFrom<()> for B {}

// CompatibleWith blanket impl
impl<B, S> CompatibleWith<S> for B
where
    S: Schema,
    B: Bundle + Satisfies<S::Require> + DisjointFrom<S::Forbid>,
{
}

/// assertion that a bundle is compatible with a schema.
pub const fn assert_compatible<B, S>()
where
    B: CompatibleWith<S>,
    S: Schema,
{
}

/// assertion that a bundle satisfies requirements.
pub const fn assert_satisfies<B, R>()
where
    B: Satisfies<R>,
    R: Bundle,
{
}

/// assertion that a bundle avoids forbidden components.
pub const fn assert_disjoint<B, F>()
where
    B: DisjointFrom<F>,
    F: Bundle,
{
}
