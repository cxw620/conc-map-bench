use chashmap::CHashMap;

use super::prelude::*;

table!(CHashMap, Value, <K>);

impl_collection! {
    |K| CHashMapTable<K>;
    with_capacity |capacity| {
        CHashMap::with_capacity(capacity)
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
