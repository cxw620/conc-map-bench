use crossbeam_skiplist::SkipMap as CrossbeamSkipMap;

use super::prelude::*;

table!(CrossbeamSkipMap, ParkingLotMutex<Value>, <K>);

impl_collection! {
    |K| CrossbeamSkipMapTable<K>;
    with_capacity |_capacity| {
        CrossbeamSkipMap::new()
    };
    get |self, key|  {
        self.0.get(key).is_some()
    };
    insert |self, key| {
        let map = &mut self.0;
        let prev = map.get(key).is_none();
        map.insert(*key, ParkingLotMutex::new(0));
        prev
    };
    remove |self, key| {
        self.0.remove(key).is_some()
    };
    update |self, key| {
        self.0.get(key).map(|e| {
            *e.value().lock() += 1;
        }).is_some()
    }
}
