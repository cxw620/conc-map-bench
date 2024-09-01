use hashlink::LinkedHashMap as HashLinkMap;

use super::prelude::*;

table!(StdRwLock, HashLinkMap, Value, <K, H>);

impl_collection! {
    |K, H| StdRwLockHashLinkMapTable<K, H>;
    with_capacity |capacity| {
        StdRwLock::new(
            HashLinkMap::with_capacity_and_hasher(capacity, H::default()),
        )
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

table!(ParkingLotRwLock, HashLinkMap, Value, <K, H>);

impl_collection! {
    |K, H| ParkingLotRwLockHashLinkMapTable<K, H>;
    with_capacity |capacity| {
        ParkingLotRwLock::new(
            HashLinkMap::with_capacity_and_hasher(capacity, H::default()),
        )
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
