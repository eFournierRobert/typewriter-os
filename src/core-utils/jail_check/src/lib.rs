//! Path validation for enforcing the user's jail boundary.
//!
//! All core utilities should use `is_target_valid` before operating on a path to ensure it resolves
//! within `/home/writer`, preventing operations outside the permitted directory tree.

use std::{env, path::Path};

/// The home directory that all operations are restricted to.
const HOME_PATH: &str = "/home/writer";

/// Checks whether `p` resolves to a path inside [`HOME_PATH`].
///
/// Joins `p` with the current working directory and canonicalizes it, then verifies the
/// resolved path starts with `/home/writer`. Returns `false` if canonicalization fails
/// (e.g., the path doesn't exist or is outside the jail).
pub fn is_target_valid(p: &Path) -> bool {
    let full_path = env::current_dir().unwrap().join(p);
    if let Ok(resolved_path) = full_path.canonicalize() {
        return resolved_path.starts_with(HOME_PATH);
    }

    false
}
