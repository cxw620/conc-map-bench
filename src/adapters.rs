pub use self::{
    btreemap::ParkingLotRwLockBTreeMapTable, btreemap::StdRwLockBTreeMapTable,
    chashmap::CHashMapTable, contrie::ContrieTable, crossbeam_skiplist::CrossbeamSkipMapTable,
    dashmap::DashMap5Table, dashmap::DashMapTable, evmap::EvmapTable, flurry::FlurryTable,
    hashlink::ParkingLotRwLockHashLinkMapTable, hashlink::StdRwLockHashLinkMapTable,
    papaya::PapayaTable, scc::SccIndexTable, scc::SccMapTable,
    std::ParkingLotRwLockStdHashMapTable, std::StdRwLockStdHashMapTable,
};

mod btreemap;
mod chashmap;
mod contrie;
mod crossbeam_skiplist;
mod dashmap;
mod evmap;
mod flurry;
mod hashlink;
mod papaya;
mod scc;
mod std;

mod prelude {
    pub(crate) use std::sync::Arc;

    pub(crate) use crate::{impl_collection, table};

    pub(crate) type Value = u32;
    pub(crate) type StdRwLock<T> = ::std::sync::RwLock<T>;
    pub(crate) type ParkingLotMutex<T> = ::parking_lot::Mutex<T>;
    pub(crate) type ParkingLotRwLock<T> = ::parking_lot::RwLock<T>;
}

pub(super) trait KeyT:
    Send + Sync + Copy + Eq + Ord + From<u64> + ::std::hash::Hash + ::std::fmt::Debug + 'static
{
}

impl<T> KeyT for T where
    T: Send + Sync + Copy + Eq + Ord + From<u64> + ::std::hash::Hash + ::std::fmt::Debug + 'static
{
}

pub(super) trait HasherT:
    Send + Sync + Clone + Default + ::std::hash::BuildHasher + 'static
{
}

impl<T> HasherT for T where T: Send + Sync + Clone + Default + ::std::hash::BuildHasher + 'static {}

#[macro_export]
macro_rules! table {
    ($inner:ident, $value:ty, <K $(,$hasher:ident)?>, NOARC) => {
        paste::item! {
            #[derive(Clone)]
            pub struct [< $inner Table >]<K: $crate::adapters::KeyT $(,$hasher: $crate::adapters::HasherT)?>($inner<K, $value $(,$hasher)?>);
        }
    };

    ($inner:ident, $value:ty, <K $(,$hasher:ident)?>) => {
        paste::item! {
            #[derive(Clone)]
            pub struct [< $inner Table >]<K: $crate::adapters::KeyT $(,$hasher: $crate::adapters::HasherT)?>(std::sync::Arc<$inner<K, $value $(,$hasher)?>>);
        }
    };

    ($lock:ident, $inner:ident, $value:ty, <K $(,$hasher:ident)?>) => {
        paste::item! {
            #[derive(Clone)]
            pub struct [< $lock $inner Table >]<K: $crate::adapters::KeyT $(,$hasher: $crate::adapters::HasherT)?>(std::sync::Arc<$lock<$inner<K, $value $(,$hasher)?>>>);
        }
    };

    ($lock:ident, $inner:ident, $value:ty, <K $(,$hasher:ident)?>; CLONE |$self:ident| $clone:block) => {
        paste::item! {
            pub struct [< $lock $inner Table >]<K: $crate::adapters::KeyT $(,$hasher: $crate::adapters::HasherT)?>(std::sync::Arc<$lock<$inner<K, $value $(,$hasher)?>>>);
        }

        impl<K $(,$hasher)?> Clone for [< $lock $inner Table >]<K $(,$hasher)?> {
            fn clone($self: &Self) -> Self {
                $clone
            }
        }
    };
}

#[macro_export]
macro_rules! impl_collection {
    (|K $(,$hasher:ident)?| $ty:ty;
        with_capacity | $capacity:ident| $with_capacity:block;
        $($name:ident |$self:ident, $key:ident| $block:block);+
    ) => {
        impl<K $(,$hasher)?> bustle::Collection for $ty
        where
            K: $crate::adapters::KeyT,
            $($hasher: $crate::adapters::HasherT,)?
        {
            type Handle = Self;

            #[inline]
            fn with_capacity($capacity: usize) -> Self {
                let inner = $with_capacity;
                Self(inner.into())
            }

            #[inline]
            fn pin(&self) -> Self::Handle {
                self.clone()
            }
        }

        impl<K $(,$hasher)?> bustle::CollectionHandle for $ty
        where
            K: Send + Sync + Copy + Eq + Ord + From<u64> + std::hash::Hash + std::fmt::Debug + 'static,
            $($hasher: Send + Sync + Clone + Default + std::hash::BuildHasher + 'static,)?
        {
            type Key = K;

            $(
                #[inline]
                fn $name($self: &mut Self, $key: &Self::Key) -> bool {
                    $block
                }
            )+
        }
    };
}
