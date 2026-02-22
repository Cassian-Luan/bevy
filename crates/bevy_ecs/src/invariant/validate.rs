//! Compile-time component constraint validation.
//!
//! Each component gets a unique compile-time identifier ([`Component::UID`]) computed from
//! `module_path!()` and the type name via a const FNV-1a hash. Components with
//! constraints declare require/forbid rules as UID arrays. The [`Bundle`] trait
//! carries a [`Bundle::VALIDATED`] associated const that performs constraint
//! checking via const evaluation — a violation triggers a const panic, which is
//! a **compile error**.
//!
//! For `#[derive(Bundle)]` structs, [`Bundle::ALL_UIDS`] is flattened at macro
//! expansion time (concrete types allow const array concatenation). For generic
//! tuple impls, `let` bindings scan each element's `ALL_UIDS` without needing
//! `generic_const_exprs`.

/// Computes a 128-bit FNV-1a hash of the given byte slice.
pub const fn fnv1a_hash(bytes: &[u8]) -> u128 {
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
/// # Panics
/// If any UID in `required` is not found in `bundle_uids`.
pub const fn check_require(required: &[u128], bundle_uids: &[u128]) {
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
            "Constraint violation: a required component is missing from the bundle"
        );
        i += 1;
    }
}

/// Asserts at compile time that no forbidden UIDs are present in the bundle's UID list.
///
/// # Panics
/// If any UID in `forbidden` is found in `bundle_uids`.
pub const fn check_forbid(forbidden: &[u128], bundle_uids: &[u128]) {
    let mut i = 0;
    while i < forbidden.len() {
        let forbid = forbidden[i];
        let mut j = 0;
        while j < bundle_uids.len() {
            assert!(
                bundle_uids[j] != forbid,
                "Constraint violation: a forbidden component is present in the bundle"
            );
            j += 1;
        }
        i += 1;
    }
}

/// Asserts at compile time that all required UIDs exist in at least one of the
/// provided UID lists.
///
/// Used by generic tuple `Bundle` impls where we cannot create a single flat
/// array (would need `generic_const_exprs`), but CAN scan multiple `ALL_UIDS`
/// slices via `let` bindings.
///
/// # Panics
/// If any UID in `required` is not found in any of the `uid_lists`.
pub const fn check_require_in_lists(required: &[u128], uid_lists: &[&[u128]]) {
    let mut i = 0;
    while i < required.len() {
        let req = required[i];
        let mut found = false;
        let mut li = 0;
        while li < uid_lists.len() {
            let list = uid_lists[li];
            let mut j = 0;
            while j < list.len() {
                if list[j] == req {
                    found = true;
                }
                j += 1;
            }
            li += 1;
        }
        assert!(
            found,
            "Constraint violation: a required component is missing from the bundle"
        );
        i += 1;
    }
}

/// Asserts at compile time that no forbidden UIDs exist in any of the provided
/// UID lists.
///
/// # Panics
/// If any UID in `forbidden` is found in any of the `uid_lists`.
pub const fn check_forbid_in_lists(forbidden: &[u128], uid_lists: &[&[u128]]) {
    let mut i = 0;
    while i < forbidden.len() {
        let forbid = forbidden[i];
        let mut li = 0;
        while li < uid_lists.len() {
            let list = uid_lists[li];
            let mut j = 0;
            while j < list.len() {
                assert!(
                    list[j] != forbid,
                    "Constraint violation: a forbidden component is present in the bundle"
                );
                j += 1;
            }
            li += 1;
        }
        i += 1;
    }
}
