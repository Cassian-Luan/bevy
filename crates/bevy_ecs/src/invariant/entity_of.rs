//! EntityOf - Entity wrapper with Schema proof.

use core::marker::PhantomData;

use crate::entity::Entity;

use super::Schema;

/// An [`Entity`] that is guaranteed to satisfy a [`Schema`].
///
/// This type serves as a "proof" that:
/// 1. The underlying Entity exists and is valid
/// 2. The Entity has component `S`
/// 3. The Entity's archetype satisfies all invariants of `S`
///
/// # Usage
///
/// `EntityOf<S>` can be used in queries to get type-safe entity handles:
///
/// ```rust,ignore
/// fn player_system(players: Query<EntityOf<Player>>) {
///     for player in &players {
///         // `player` is guaranteed to satisfy Player's Schema
///         let entity: Entity = player.entity();
///     }
/// }
/// ```
///
/// # Type Safety
///
/// Operations on `EntityOf<S>` can be statically checked against `S`'s constraints.
/// For example, inserting a component that is in `S::Forbid` would be a compile error.
#[derive(Clone, Copy)]
pub struct EntityOf<S: Schema> {
    entity: Entity,
    _marker: PhantomData<S>,
}

impl<S: Schema> EntityOf<S> {
    /// Creates a new `EntityOf` from an `Entity`.
    ///
    /// # Safety
    ///
    /// The caller must ensure that:
    /// - The entity exists and is valid
    /// - The entity has component `S`
    /// - The entity's archetype satisfies all invariants of `S`
    #[inline]
    pub(crate) unsafe fn new_unchecked(entity: Entity) -> Self {
        Self {
            entity,
            _marker: PhantomData,
        }
    }

    /// Returns the underlying [`Entity`].
    ///
    /// This is a "downgrade" operation that loses the type-level guarantee.
    #[inline]
    pub fn entity(self) -> Entity {
        self.entity
    }

    /// Returns the underlying [`Entity`] by reference.
    #[inline]
    pub fn as_entity(&self) -> Entity {
        self.entity
    }
}

impl<S: Schema> From<EntityOf<S>> for Entity {
    #[inline]
    fn from(typed: EntityOf<S>) -> Self {
        typed.entity
    }
}

impl<S: Schema> core::fmt::Debug for EntityOf<S> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("EntityOf")
            .field("entity", &self.entity)
            .field("schema", &core::any::type_name::<S>())
            .finish()
    }
}

impl<S: Schema> PartialEq for EntityOf<S> {
    fn eq(&self, other: &Self) -> bool {
        self.entity == other.entity
    }
}

impl<S: Schema> Eq for EntityOf<S> {}

impl<S: Schema> core::hash::Hash for EntityOf<S> {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        self.entity.hash(state);
    }
}
