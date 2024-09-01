use super::prelude::*;

type Evmap<K, V, H> = (
    Arc<ParkingLotMutex<evmap::ReadHandle<K, V, (), H>>>,
    Arc<ParkingLotMutex<evmap::WriteHandle<K, V, (), H>>>,
);

table!(Evmap, Value, <K, H>, NOARC);

impl_collection! {
    |K, H| EvmapTable<K, H>;
    with_capacity |capacity| {
        let (rd, wr) = evmap::Options::default()
            .with_hasher(H::default())
            .with_capacity(capacity)
            .construct();

        (Arc::new(ParkingLotMutex::new(rd)), Arc::new(ParkingLotMutex::new(wr)))
    };
    get |self, key|  {
        self.0.0.lock().get_one(key).is_some()
    };
    insert |self, key| {
        let prev = self.0.0.lock().get_one(key).is_none();
        self.0.1.lock().insert(*key, 0).refresh();
        prev
    };
    remove |self, key| {
        let prev = self.0.0.lock().get_one(key).is_some();
        self.0.1.lock().empty(*key).refresh();
        prev
    };
    update |self, key| {
        let val = match self.0.0.lock().get_one(key) {
            Some(val) => *val + 1,
            None => return false,
        };

        let prev = self.0.0.lock().get_one(key).is_some();
        self.0.1.lock().update(*key, val).refresh();
        prev
    }
}
