use std::{error::Error, fmt::Debug, io, sync::Arc};

use bustle::Measurement;
use clap::Parser;

mod adapters;
mod args;
mod deps;
mod plot;
mod record;

#[cfg(feature = "alloc_mimalloc")]
#[global_allocator]
static GLOBAL: mimalloc::MiMalloc = mimalloc::MiMalloc;

#[cfg(feature = "alloc_jemalloc")]
#[global_allocator]
static GLOBAL: tikv_jemallocator::Jemalloc = tikv_jemallocator::Jemalloc;

#[cfg(all(feature = "alloc_mimalloc", feature = "alloc_jemalloc"))]
compile_error!("only one allocator can be specified");

fn main() -> Result<(), Box<dyn Error>> {
    tracing_subscriber::fmt::init();

    match args::Args::parse().command {
        args::Command::Bench {
            workload,
            operations,
            threads,
            hasher,
            gc_sleep_ms,
            skip,
            csv,
            csv_no_headers,
        } => {
            let mut handler = if csv {
                let mut wr = csv::WriterBuilder::new()
                    .has_headers(!csv_no_headers)
                    .from_writer(io::stderr());

                Box::new(move |name: &str, n, m: &Measurement| {
                    wr.serialize(record::Record {
                        name: name.into(),
                        total_ops: m.total_ops,
                        threads: n,
                        spent: m.spent,
                        throughput: m.throughput,
                        latency: m.latency,
                    })
                    .expect("cannot serialize");
                    wr.flush().expect("cannot flush");
                }) as BenchHandler
            } else {
                Box::new(|_: &str, n, m: &Measurement| {
                    eprintln!(
                        "total_ops={}\tthreads={}\tspent={:.1?}\tlatency={:?}\tthroughput={:.0}op/s",
                        m.total_ops, n, m.spent, m.latency, m.throughput,
                    );
                }) as BenchHandler
            };

            // * register bench cases

            macro_rules! add_bench_case {
                (@internal => $ty:ty) => {
                    compile_error!("missing name or dependency");
                };
                (@internal => $dep:ident, $ty:ty) => {
                    add_bench_case::<$ty>(
                        &dep_name_with_version!($dep),
                        &skip,
                        &threads,
                        workload,
                        operations,
                        gc_sleep_ms,
                        &mut handler,
                    );
                };
                (@internal => $name:literal, $ty:ty) => {
                    add_bench_case::<$ty>(
                        &stringify!($name).trim_matches('"'),
                        &skip,
                        &threads,
                        workload,
                        operations,
                        gc_sleep_ms,
                        &mut handler,
                    );
                };
                (@internal => $dep:ident, $name:literal, $ty:ty) => {
                    add_bench_case::<$ty>(
                        &dep_name_with_version!($dep, $name),
                        &skip,
                        &threads,
                        workload,
                        operations,
                        gc_sleep_ms,
                        &mut handler,
                    );
                };
                ($($($dep:ident)? $($name:literal)? => $ty:ty);*) => {
                    $(
                        add_bench_case!(@internal => $($dep,)? $($name,)? $ty );
                    )*
                };
            }

            add_bench_case! {
                // "std(btreemap)" => adapters::StdRwLockBTreeMapTable<u64>;
                // "std(parking_lot, btreemap)" => adapters::ParkingLotRwLockBTreeMapTable<u64>;
                chashmap => adapters::CHashMapTable<u64>;
                crossbeam_skiplist => adapters::CrossbeamSkipMapTable<u64>
            };

            // custom Hasher
            match hasher {
                args::HasherKind::Std => {
                    add_bench_case! {
                        // std / parking_lot reference
                        "std" => adapters::StdRwLockStdHashMapTable<u64, std::hash::RandomState>;
                        "std(parking_lot)" => adapters::ParkingLotRwLockStdHashMapTable<u64, std::hash::RandomState>;

                        // 3rd party
                        contrie => adapters::ContrieTable<u64, std::hash::RandomState>;
                        dashmap => adapters::DashMapTable<u64, std::hash::RandomState>;
                        dashmap5 => adapters::DashMap5Table<u64, std::hash::RandomState>;
                        evmap => adapters::EvmapTable<u64, std::hash::RandomState>;
                        flurry => adapters::FlurryTable<u64, std::hash::RandomState>;
                        hashlink "std" => adapters::StdRwLockHashLinkMapTable<u64, std::hash::RandomState>;
                        hashlink "parking_lot" => adapters::ParkingLotRwLockHashLinkMapTable<u64, std::hash::RandomState>;
                        papaya => adapters::PapayaTable<u64, std::hash::RandomState>;
                        scc "HashMap" => adapters::SccMapTable<u64, std::hash::RandomState>;
                        scc "HashIndex" => adapters::SccIndexTable<u64, std::hash::RandomState>
                    };
                }
                args::HasherKind::AHash => {
                    add_bench_case! {
                        // std / parking_lot reference
                        "std" => adapters::StdRwLockStdHashMapTable<u64, ahash::RandomState>;
                        "std(parking_lot)" => adapters::ParkingLotRwLockStdHashMapTable<u64, ahash::RandomState>;

                        // 3rd party
                        contrie => adapters::ContrieTable<u64, ahash::RandomState>;
                        dashmap => adapters::DashMapTable<u64, ahash::RandomState>;
                        dashmap5 => adapters::DashMap5Table<u64, ahash::RandomState>;
                        evmap => adapters::EvmapTable<u64, ahash::RandomState>;
                        flurry => adapters::FlurryTable<u64, ahash::RandomState>;
                        hashlink "std" => adapters::StdRwLockHashLinkMapTable<u64, ahash::RandomState>;
                        hashlink "parking_lot" => adapters::ParkingLotRwLockHashLinkMapTable<u64, ahash::RandomState>;
                        papaya => adapters::PapayaTable<u64, ahash::RandomState>;
                        scc "HashMap" => adapters::SccMapTable<u64, ahash::RandomState>;
                        scc "HashIndex" => adapters::SccIndexTable<u64, ahash::RandomState>
                    };
                }
            }
        }
        args::Command::Plot {
            dir,
            name,
            width,
            height,
            latency_limit_ns,
        } => {
            let dir = dir.to_string_lossy();
            plot::Groups::init()
                .plot_throughput(&dir, &name, width, height)?
                .plot_latency(&dir, &name, width, height, latency_limit_ns)?;
        }
    }

    Ok(())
}

type BenchHandler = Box<dyn FnMut(&str, u32, &Measurement)>;

fn add_bench_case<C>(
    name: &str,
    skip: &Vec<Arc<str>>,
    threads: &Option<Vec<u32>>,
    workload: args::WorkloadType,
    operations: f64,
    gc_sleep_ms: u64,
    handler: &mut BenchHandler,
) where
    C: bustle::Collection,
    <C::Handle as bustle::CollectionHandle>::Key: Send + Debug,
{
    if skip.iter().find(|s| name.starts_with(s.as_ref())).is_some() {
        println!("-- {} [skipped]", name);
        return;
    } else {
        println!("-- {}", name);
    }

    let threads = threads.as_ref().cloned().unwrap_or_else(|| {
        let n = num_cpus::get();

        match n {
            0..=10 => (1..=n as u32).collect(),
            11..=16 => std::iter::once(1)
                .chain((0..=n as u32).step_by(2).skip(1))
                .collect(),
            _ => std::iter::once(1)
                .chain((0..=n as u32).step_by(4).skip(1))
                .collect(),
        }
    });

    #[inline]
    fn gc_cycle(gc_sleep_ms: u64) {
        use std::{thread::sleep, time::Duration};

        sleep(Duration::from_millis(gc_sleep_ms));

        let mut new_guard = crossbeam_epoch::pin();

        new_guard.flush();

        for _ in 0..32 {
            new_guard.repin();
        }
    }

    for n in &threads {
        let m = workload.create(*n, operations).run_silently::<C>();

        handler(name, *n, &m);

        gc_cycle(gc_sleep_ms);
    }

    println!();
}
