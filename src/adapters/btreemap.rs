use std::collections::BTreeMap;

use super::prelude::*;

table!(StdRwLock, BTreeMap, Value, <K>);

impl_collection! {
    |K| StdRwLockBTreeMapTable<K>;
    with_capacity |_capacity| {
        StdRwLock::new(BTreeMap::new())
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
        self.0.write().unwrap().get_mut(key).map(|v| *v += 1).is_some()
    }
}

table!(ParkingLotRwLock, BTreeMap, Value, <K>);

impl_collection! {
    |K| ParkingLotRwLockBTreeMapTable<K>;
    with_capacity |_capacity| {
        ParkingLotRwLock::new(BTreeMap::new())
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
        self.0.write().get_mut(key).map(|v| *v += 1).is_some()
    }
}
