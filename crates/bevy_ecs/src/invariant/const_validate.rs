//! Compile-time component constraint validation.
//!
//! This module provides const functions that enable **compile-time** verification
//! of component constraints (require/forbid).
//!
//! # How It Works
//!
//! Each component gets a unique compile-time identifier (`SCHEMA_UID`) computed from
//! `module_path!()` and the type name via a const FNV-1a hash. Schema components
//! declare their require/forbid constraints as arrays of these UIDs. The Bundle
//! trait carries a `SCHEMA_VALIDATED` associated const that performs constraint
//! checking via const evaluation — a violation triggers a const panic, which is
//! a **compile error**.
//!
//! # Limitations
//!
//! - **Flat tuples only**: Full compile-time validation works for flat tuples of
//!   components (e.g., `(Player, Health, Transform)`). Nested bundles validate
//!   their own internal constraints but cannot propagate UIDs upward for
//!   cross-bundle checking. The existing runtime validation in `archetype.rs`
//!   serves as a fallback for nested cases.
//! - **Hash collisions**: UIDs are based on `module_path!() + type_name` hashing.
//!   Collisions are astronomically unlikely but theoretically possible.

/// Computes a 128-bit FNV-1a hash of the given byte slice.
///
/// This is used to generate compile-time unique identifiers for components
/// from their `module_path!()` and type name.
///
/// The hash is evaluated entirely at compile time when used in const context.
pub const fn const_fnv1a_hash(bytes: &[u8]) -> u128 {
    // FNV-1a 128-bit offset basis and prime
    let mut hash: u128 = 0x6c62_272e_07bb_0142_eb62_642a_7631_64c6;
    let prime: u128 = 0x0000_0000_0100_0000_0000_0000_0000_01b3;
    let mut i = 0;
    while i < bytes.len() {
        hash ^= bytes[i] as u128;
        hash = hash.wrapping_mul(prime);
        i += 1;
    }
    hash
}

/// Asserts at compile time that all required UIDs are present in the bundle's UID list.
///
/// # Panics (compile error)
///
/// Panics if any UID in `required` is not found in `bundle_uids`.
pub const fn const_check_require(required: &[u128], bundle_uids: &[u128]) {
    let mut i = 0;
    while i < required.len() {
        let req = required[i];
        let mut found = false;
        let mut j = 0;
        while j < bundle_uids.len() {
            if bundle_uids[j] == req {
                found = true;
            }
            j += 1;
        }
        assert!(
            found,
            "Schema constraint violation: a required component is missing from the bundle"
        );
        i += 1;
    }
}

/// Asserts at compile time that no forbidden UIDs are present in the bundle's UID list.
///
/// # Panics (compile error)
///
/// Panics if any UID in `forbidden` is found in `bundle_uids`.
pub const fn const_check_forbid(forbidden: &[u128], bundle_uids: &[u128]) {
    let mut i = 0;
    while i < forbidden.len() {
        let forbid = forbidden[i];
        let mut j = 0;
        while j < bundle_uids.len() {
            assert!(
                bundle_uids[j] != forbid,
                "Schema constraint violation: a forbidden component is present in the bundle"
            );
            j += 1;
        }
        i += 1;
    }
}
