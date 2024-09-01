use dashmap::DashMap;
use dashmap5::DashMap as DashMap5;

use super::prelude::*;

table!(DashMap, Value, <K, H>);

impl_collection! {
    |K, H| DashMapTable<K, H>;
    with_capacity |capacity| {
        DashMap::with_capacity_and_hasher(capacity, H::default())
    };
    get |self, key|  {
        self.0.get(key).is_some()
    };
    insert |self, key| {
        self.0.insert(*key, 0).is_none()
    };
    remove |self, key| {
        self.0.remove(key).is_some()
    };
    update |self, key| {
        self.0.get_mut(key).map(|mut v| *v += 1).is_some()
    }
}

table!(DashMap5, Value, <K, H>);

impl_collection! {
    |K, H| DashMap5Table<K, H>;
    with_capacity |capacity| {
        DashMap5::with_capacity_and_hasher(capacity, H::default())
    };
    get |self, key|  {
        self.0.get(key).is_some()
    };
    insert |self, key| {
        self.0.insert(*key, 0).is_none()
    };
    remove |self, key| {
        self.0.remove(key).is_some()
    };
    update |self, key| {
        self.0.get_mut(key).map(|mut v| *v += 1).is_some()
    }
}