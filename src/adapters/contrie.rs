use contrie::ConMap as Contrie;

use super::prelude::*;

table!(Contrie, ParkingLotMutex<Value>, <K, H>);

impl_collection! {
    |K, H| ContrieTable<K, H>;
    with_capacity |_capacity| {
        Contrie::with_hasher(H::default())
    };
    get |self, key|  {
        self.0.get(key).is_some()
    };
    insert |self, key| {
        self.0.insert(*key, ParkingLotMutex::new(0)).is_none()
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
