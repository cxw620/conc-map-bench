//! Args definition

use std::{path::PathBuf, str::FromStr, sync::Arc};

use clap::{Parser, Subcommand};

#[derive(Debug, Parser)]
pub(crate) struct Args {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Debug, Subcommand)]
pub(crate) enum Command {
    /// Run benchmark
    Bench {
        #[arg(short('w'), long)]
        /// Select work load type.
        workload: WorkloadType,

        #[arg(short, long, default_value = "1")]
        /// Set the number of operations to run as a multiple of the initial capacity.
        operations: f64,

        #[arg(long)]
        /// Set the number of threads to use.
        threads: Option<Vec<u32>>,

        #[arg(short, long)]
        /// Set the hasher to use.
        ///
        /// Must be one of 'std' or 'ahash'.
        hasher: HasherKind,

        #[arg(long, default_value = "2000")]
        /// Set the number of milliseconds to sleep between GC cycles.
        gc_sleep_ms: u64,

        #[arg(long, default_value = "crossbeam_skiplist,chashmap,evmap", value_delimiter = ',')]
        /// Skip the given cases.
        ///
        /// Since the following crates are with much worser perf, they are skipped by default:
        ///
        /// - crossbeam_skiplist (>> 300ns latency, < 20Mops throughput in 16 threads)
        /// - chashmap (2019.3) (>> 300ns latency, about 20Mops throughput in 16 threads)
        /// - evmap (2020.12) (>> 300ns latency, << 10 Mops throughput in 16 threads)
        skip: Vec<Arc<str>>,

        #[arg(long)]
        /// Output results in CSV format.
        csv: bool,

        #[arg(long)]
        /// Output results in CSV format without headers.
        csv_no_headers: bool,
    },

    /// Plot results
    Plot {
        /// Set the directory to export the plots to.
        ///
        /// The plots will be exported as:
        /// <dir>/<name>.throughput.svg
        /// <dir>/<name>.latency.svg
        dir: PathBuf,

        /// Set the name of the plot.
        name: String,

        #[arg(short, long, default_value = "640")]
        /// Set the width of the plot.
        width: u32,

        #[arg(short, long, default_value = "480")]
        /// Set the height of the plot.
        height: u32,

        #[arg(long, default_value = "2000")]
        /// Set the latency limit in nanoseconds.
        latency_limit_ns: u64,
    },
}

#[derive(Debug, Clone, Copy)]
pub(crate) enum WorkloadType {
    /// Read-heavy workload.
    ReadHeavy,

    /// Exchange workload.
    Exchange,

    /// Rapid grow workload.
    RapidGrow,
}

impl FromStr for WorkloadType {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_ascii_lowercase().as_str() {
            "readheavy" => Ok(Self::ReadHeavy),
            "read_heavy" => Ok(Self::ReadHeavy),
            "exchange" => Ok(Self::Exchange),
            "rapidgrow" => Ok(Self::RapidGrow),
            "rapid_grow" => Ok(Self::RapidGrow),

            _ => Err("unknown workload"),
        }
    }
}

impl WorkloadType {
    /// Create a [bustle::Workload] based on the given options.
    pub(crate) fn create(&self, threads: u32, operations: f64) -> bustle::Workload {
        let mut workload = match self {
            Self::ReadHeavy => Self::read_heavy(threads),
            Self::Exchange => Self::exchange(threads),
            Self::RapidGrow => Self::rapid_grow(threads),
        };

        workload.operations(operations);

        workload
    }

    fn read_heavy(threads: u32) -> bustle::Workload {
        let mix = bustle::Mix {
            read: 98,
            insert: 1,
            remove: 1,
            update: 0,
            upsert: 0,
        };

        *bustle::Workload::new(threads as usize, mix)
            .initial_capacity_log2(25)
            .prefill_fraction(0.75)
    }

    fn rapid_grow(threads: u32) -> bustle::Workload {
        let mix = bustle::Mix {
            read: 5,
            insert: 80,
            remove: 5,
            update: 10,
            upsert: 0,
        };

        *bustle::Workload::new(threads as usize, mix)
            .initial_capacity_log2(25)
            .prefill_fraction(0.0)
    }

    fn exchange(threads: u32) -> bustle::Workload {
        let mix = bustle::Mix {
            read: 10,
            insert: 40,
            remove: 40,
            update: 10,
            upsert: 0,
        };

        *bustle::Workload::new(threads as usize, mix)
            .initial_capacity_log2(25)
            .prefill_fraction(0.75)
    }
}

#[derive(Debug, Clone, Copy)]
pub(crate) enum HasherKind {
    /// Standard hasher
    Std,

    /// AHash hasher by default
    AHash,
}

impl FromStr for HasherKind {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "std" => Ok(Self::Std),
            "ahash" => Ok(Self::AHash),
            _ => Err("invalid hasher, must be one of 'std' or 'ahash'"),
        }
    }
}
