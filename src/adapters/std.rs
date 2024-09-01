use std::collections::HashMap as StdHashMap;

use super::prelude::*;

table!(StdRwLock, StdHashMap, Value, <K, H>);

impl_collection! {
    |K, H| StdRwLockStdHashMapTable<K, H>;
    with_capacity |capacity| {
        StdRwLock::new(StdHashMap::with_capacity_and_hasher(
            capacity,
            H::default(),
        ))
    };
    get |self, key|  {
        self.0.read().unwrap().get(key).is_some()
    };
    insert |self, key| {
        self.0.write().unwrap().insert(*key, 0).is_none()
    };
    remove |self, key| {
        self.0.write().unwrap().remove(key).is_some()
    };
    update |self, key| {
        self.0
            .write()
            .unwrap()
            .get_mut(key)
            .map(|v| *v += 1)
            .is_some()
    }
}

table!(ParkingLotRwLock, StdHashMap, Value, <K, H>);

impl_collection! {
    |K, H| ParkingLotRwLockStdHashMapTable<K, H>;
    with_capacity |capacity| {
        ParkingLotRwLock::new(StdHashMap::with_capacity_and_hasher(
            capacity,
            H::default(),
        ))
    };
    get |self, key|  {
        self.0.read().get(key).is_some()
    };
    insert |self, key| {
        self.0.write().insert(*key, 0).is_none()
    };
    remove |self, key| {
        self.0.write().remove(key).is_some()
    };
    update |self, key| {
        let mut map = self.0.write();
        map.get_mut(key).map(|v| *v += 1).is_some()
    }
}
