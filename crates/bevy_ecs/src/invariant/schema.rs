//! Schema trait definition for archetype invariants.

use crate::bundle::Bundle;
use alloc::vec::Vec;
use crate::component::{Component, ComponentsRegistrator};

use super::ComponentConstraints;

/// Defines archetype invariants for a component.
///
/// When a component implements `Schema`, it becomes a **Schema Component**.
/// The archetype layer will automatically validate its constraints whenever
/// an archetype containing this component is created or transitioned.
///
/// # Analogy
///
/// Think of this as a database table schema:
/// - `Require`: NOT NULL columns that must exist (caller must provide)
/// - `Forbid`: CHECK constraint that prevents certain combinations
///
/// # Example
///
/// ```rust,ignore
/// use bevy_ecs::prelude::*;
/// use bevy_ecs::invariant::Schema;
///
/// #[derive(Component)]
/// struct Player;
///
/// #[derive(Component)]
/// struct Enemy;
///
/// #[derive(Component)]
/// struct Health(i32);
///
/// impl Schema for Player {
///     type Require = Health;
///     type RequireOrDefault = ();
///     type Forbid = Enemy;
/// }
/// ```
pub trait Schema: Component {
    /// Components that MUST coexist with Self.
    ///
    /// Use `()` if no requirements.
    type Require: Bundle;

    /// Components that must coexist with Self, or will be inserted with defaults.
    ///
    /// Use `()` if no requirements.
    type RequireOrDefault: Bundle;

    /// Components that CANNOT coexist with Self.
    ///
    /// Use `()` if no forbidden components.
    type Forbid: Bundle;

    /// Builds runtime [`SchemaConstraints`] from this Schema's type information.
    ///
    /// This registers the Require/Forbid component types and converts them to
    /// `ComponentId` sets for archetype-level validation.
    fn build_constraints(registrator: &mut ComponentsRegistrator) -> ComponentConstraints {
        let require_ids: Vec<_> =
            <Self::Require as Bundle>::component_ids(registrator).collect();
        let forbid_ids: Vec<_> =
            <Self::Forbid as Bundle>::component_ids(registrator).collect();
        ComponentConstraints::new(require_ids.into(), forbid_ids.into())
    }
}
