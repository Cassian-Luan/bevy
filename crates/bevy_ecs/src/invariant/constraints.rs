//! Unified constraint system for components.
//!
//! [`ComponentConstraints`] merges all component-level constraints into one structure:
//! - `require`: must be explicitly provided (panic if missing)
//! - `require_or_default`: auto-inserted with defaults if missing (absorbs `#[require]`)
//! - `forbid`: cannot coexist
//! - `required_by`: reverse index (who depends on me)

use alloc::boxed::Box;

use bevy_platform::hash::FixedHasher;
use indexmap::IndexSet;

use crate::component::{ComponentId, Components, RequiredComponents};

/// Constraints for a component, stored in [`ComponentInfo`](crate::component::ComponentInfo).
///
/// Every component has a [`ComponentConstraints`] (defaulting to empty).
/// Components implementing [`Schema`](super::Schema) populate the `require` and `forbid` fields.
/// The `require_or_default` and `required_by` fields absorb the existing `#[require]` mechanism.
#[derive(Debug, Clone, Default)]
pub struct ComponentConstraints {
    /// Components that MUST be explicitly provided when this component is present.
    /// Missing any of these in an archetype will panic.
    pub(crate) require: Box<[ComponentId]>,

    /// Components that must exist but will be auto-inserted with defaults if missing.
    /// This is the existing `#[require]` / `RequiredComponents` mechanism.
    pub(crate) require_or_default: RequiredComponents,

    /// Components that CANNOT coexist with this component.
    pub(crate) forbid: Box<[ComponentId]>,

    /// Reverse index: components that depend on this component.
    /// Invariant: components in this set always appear after the components that they require.
    pub(crate) required_by: IndexSet<ComponentId, FixedHasher>,
}

impl ComponentConstraints {
    /// Creates a new `SchemaConstraints` with only `require` and `forbid` sets.
    /// Used by [`Schema::build_constraints`](super::Schema::build_constraints).
    pub fn new(require: Box<[ComponentId]>, forbid: Box<[ComponentId]>) -> Self {
        Self {
            require,
            forbid,
            ..Default::default()
        }
    }

    /// Returns the set of strictly required component IDs.
    #[inline]
    pub fn require(&self) -> &[ComponentId] {
        &self.require
    }

    /// Returns the required-or-default components (the `#[require]` mechanism).
    #[inline]
    pub fn require_or_default(&self) -> &RequiredComponents {
        &self.require_or_default
    }

    /// Returns a mutable reference to the required-or-default components.
    #[inline]
    pub(crate) fn require_or_default_mut(&mut self) -> &mut RequiredComponents {
        &mut self.require_or_default
    }

    /// Returns the set of forbidden component IDs.
    #[inline]
    pub fn forbid(&self) -> &[ComponentId] {
        &self.forbid
    }

    /// Returns the set of components that depend on this one.
    #[inline]
    pub fn required_by(&self) -> &IndexSet<ComponentId, FixedHasher> {
        &self.required_by
    }

    /// Returns a mutable reference to the `required_by` set.
    #[inline]
    pub(crate) fn required_by_mut(&mut self) -> &mut IndexSet<ComponentId, FixedHasher> {
        &mut self.required_by
    }

    /// Validates that a set of component IDs satisfies the `require` and `forbid` constraints.
    ///
    /// # Panics
    ///
    /// Panics if any required component is missing or any forbidden component is present.
    ///
    /// # Safety
    ///
    /// All IDs in `component_ids`, `self.require`, and `self.forbid` must be valid in `components`.
    pub unsafe fn validate(&self, component_ids: &[ComponentId], owner: ComponentId, components: &Components) {
        // SAFETY: The caller ensures all IDs are valid.
        let name = |id: ComponentId| unsafe { components.get_info_unchecked(id) }.name();

        for &req_id in &self.require {
            if !component_ids.contains(&req_id) {
                panic!(
                    "Schema invariant violation: `{}` requires component `{}`, but it is missing from the archetype {:?}",
                    name(owner),
                    name(req_id),
                    component_ids,
                );
            }
        }
        for &forbid_id in &self.forbid {
            if component_ids.contains(&forbid_id) {
                panic!(
                    "Schema invariant violation: `{}` forbids component `{}`, but it is present in the archetype {:?}",
                    name(owner),
                    name(forbid_id),
                    component_ids,
                );
            }
        }
    }
}
