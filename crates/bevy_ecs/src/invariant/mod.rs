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

mod validate;

pub use validate::{check_forbid, check_forbid_in_lists, check_require, check_require_in_lists, fnv1a_hash};
