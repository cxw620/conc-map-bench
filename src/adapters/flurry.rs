use flurry::HashMap as Flurry;
use seize::Collector;

use super::prelude::*;

const BATCH_SIZE: usize = 2000;

table!(Flurry, Value, <K, H>);

impl_collection! {
    |K, H| FlurryTable<K, H>;
    with_capacity |capacity| {
        Flurry::with_capacity_and_hasher(capacity, H::default()).with_collector(
            Collector::new()
                .epoch_frequency(None)
                .batch_size(BATCH_SIZE),
        )
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
        self.0
            .pin()
            .compute_if_present(key, |_, v| Some(v + 1))
            .is_some()
    }
}
