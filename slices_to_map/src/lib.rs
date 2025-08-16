use std::hash::Hash;
use std::collections::HashMap;

pub fn slices_to_map<'a, T: Hash + Eq, U>(keys: &'a [T], values: &'a [U]) -> HashMap<&'a T, &'a U> {
    keys.iter().zip(values).collect()
}
