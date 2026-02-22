//! Archetype Invariants - Type-safe ECS constraint system.
//!
//! This module provides the ability to model relational database constraints on top of ECS.
//!
//! # Core Concepts
//!
//! - [`Schema`]: Defines component invariants (analogous to database table schema)
//! - [`EntityOf`]: Entity wrapper with type proof (proves it satisfies a Schema)
//!
//! # Design Document
//!
//! See `DESIGN.md` in this directory for detailed design rationale and open challenges.
//!
//! # Example
//!
//! ```rust,ignore
//! use bevy_ecs::prelude::*;
//! use bevy_ecs::invariant::{Schema, EntityOf};
//!
//! #[derive(Component)]
//! struct Player;
//!
//! #[derive(Component)]
//! struct Enemy;
//!
//! #[derive(Component)]
//! struct Health(i32);
//!
//! impl Schema for Player {
//!     type Require = Health;
//!     type Forbid = Enemy;
//! }
//!
//! fn player_system(players: Query<EntityOf<Player>>) {
//!     for player in &players {
//!         // `player` is guaranteed to have Health and not have Enemy
//!         let entity: Entity = player.entity();
//!     }
//! }
//! ```
//!
//! # Roadmap
//!
//! 1. **Static Verification** (current): Compile-time checks via `Schema` trait
//! 2. **Runtime Verification**: Validate invariants when archetypes are created
//! 3. **Scheduler Optimization**: Use invariants to improve query parallelism

mod check;
mod const_validate;
mod constraints;
mod entity_of;
mod schema;
mod schema_bundle;

pub use check::{
    assert_compatible, assert_disjoint, assert_satisfies, CompatibleWith, Contains, DisjointFrom,
    NotContains, Satisfies,
};
pub use const_validate::{const_check_forbid, const_check_require, const_fnv1a_hash};
pub use constraints::ComponentConstraints;
pub use entity_of::EntityOf;
pub use schema::Schema;
pub use schema_bundle::SchemaBundle;
