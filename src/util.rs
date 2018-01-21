//! Utility functions.

use std::collections::HashMap;
use std::hash::Hash;

/// Applies a function to an entry.
pub fn entry_fn<F, K, V>(map: &mut HashMap<K, V>, key: K, f: F)
where
    F: FnOnce(&K, Option<V>) -> Option<V>,
    K: Eq + Hash,
{
    let new = f(&key, map.remove(&key));
    if let Some(new) = new {
        map.insert(key, new);
    }
}
