//! SchemaBundle - A bundle wrapper that validates Schema constraints.

use core::marker::PhantomData;

use crate::bundle::Bundle;

use super::check::CompatibleWith;
use super::Schema;

/// A bundle wrapper that validates Schema constraints at compile time.
///
/// `SchemaBundle` wraps a bundle `B` and associates it with a schema `S`.
/// When created via [`SchemaBundle::new`], it verifies at compile time that
/// `B` satisfies `S`'s constraints.
///
/// # Usage
///
/// Use `.into_inner()` to extract the bundle for spawning:
///
/// ```rust,ignore
/// use bevy_ecs::prelude::*;
/// use bevy_ecs::invariant::{Schema, SchemaBundle};
///
/// #[derive(Component)]
/// struct Player;
///
/// #[derive(Component)]
/// struct Health(i32);
///
/// impl Schema for Player {
///     type Require = Health;
///     type RequireOrDefault = ();
///     type Forbid = ();
/// }
///
/// // Create a validated bundle - compile-time check
/// let bundle = SchemaBundle::<Player, _>::new((Player, Health(100)));
///
/// // Extract and spawn
/// commands.spawn(bundle.into_inner());
///
/// // This would NOT compile - missing Health
/// // let bad = SchemaBundle::<Player, _>::new((Player,));
/// ```
pub struct SchemaBundle<S: Schema, B: Bundle> {
    bundle: B,
    _schema: PhantomData<S>,
}

impl<S: Schema, B: Bundle> SchemaBundle<S, B> {
    /// Creates a new `SchemaBundle` with compile-time validation.
    ///
    /// This only compiles if `B` satisfies `S`'s constraints:
    /// - `B` contains all components in `S::Require`
    /// - `B` contains no components in `S::Forbid`
    #[inline]
    pub fn new(bundle: B) -> Self
    where
        B: CompatibleWith<S>,
    {
        Self {
            bundle,
            _schema: PhantomData,
        }
    }

    /// Creates a new `SchemaBundle` without validation.
    ///
    /// Use this to bypass compile-time checks when you know the bundle is valid
    /// but the type system can't prove it.
    #[inline]
    pub fn new_unchecked(bundle: B) -> Self {
        Self {
            bundle,
            _schema: PhantomData,
        }
    }

    /// Unwraps the inner bundle.
    ///
    /// Use this to pass the bundle to `commands.spawn()` or similar APIs.
    #[inline]
    pub fn into_inner(self) -> B {
        self.bundle
    }

    /// Returns a reference to the inner bundle.
    #[inline]
    pub fn inner(&self) -> &B {
        &self.bundle
    }

    /// Returns a mutable reference to the inner bundle.
    #[inline]
    pub fn inner_mut(&mut self) -> &mut B {
        &mut self.bundle
    }
}
