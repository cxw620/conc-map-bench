use scc::{hash_map::HashMap as SccMap, HashIndex as SccIndex, LinkedList};

use super::prelude::*;

table!(SccMap, Value, <K, H>);

impl_collection! {
    |K, H| SccMapTable<K, H>;
    with_capacity |capacity| {
        SccMap::with_capacity_and_hasher(capacity, H::default())
    };
    get |self, key|  {
        self.0.read(key, |_, v| *v).is_some()
    };
    insert |self, key| {
        self.0.insert(*key, 0).is_ok()
    };
    remove |self, key| {
        self.0.remove(key).is_some()
    };
    update |self, key| {
        match self.0.entry(*key) {
            scc::hash_map::Entry::Occupied(mut v) => {
                *v.get_mut() += 1;
                true
            }
            scc::hash_map::Entry::Vacant(_) => false,
        }
    }
}

table!(SccIndex, Value, <K, H>);

impl_collection! {
    |K, H| SccIndexTable<K, H>;
    with_capacity |capacity| {
        SccIndex::with_capacity_and_hasher(capacity, H::default())
    };
    get |self, key|  {
        self.0.peek_with(key, |_, v| *v).is_some()
    };
    insert |self, key| {
        self.0.insert(*key, 0).is_ok()
    };
    remove |self, key| {
        self.0.remove(key)
    };
    update |self, key| {
        match self.0.entry(*key) {
            scc::hash_index::Entry::Occupied(mut v) => {
                unsafe {*v.get_mut() += 1};
                true
            }
            scc::hash_index::Entry::Vacant(_) => false,
        }
    }
}
