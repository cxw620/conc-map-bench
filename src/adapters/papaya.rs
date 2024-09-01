use papaya::HashMap as Papaya;

use super::prelude::*;

const BATCH_SIZE: usize = 2000;

table!(Papaya, Value, <K, H>);

impl_collection! {
    |K, H| PapayaTable<K, H>;
    with_capacity |capacity| {
        papaya::HashMap::builder()
            .capacity(capacity)
            .hasher(H::default())
            .collector(
                papaya::Collector::new()
                    .epoch_frequency(None)
                    .batch_size(BATCH_SIZE),
            )
            .build()
    };
    get |self, key|  {
        self.0.pin().get(key).is_some()
    };
    insert |self, key| {
        self.0.pin().insert(*key, 0).is_none()
    };
    remove |self, key| {
        self.0.pin().remove(key).is_some()
    };
    update |self, key| {
        self.0.pin().update(*key, |v| v + 1).is_some()
    }
}
