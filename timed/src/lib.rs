//! # Timed
//!
//! Macros for measuring function execution.
//! ```
//! #[timed::timed]
//! fn add(x: i32, y: i32) -> i32 {
//!     x + y
//! }
//! ```
//! It will output:
//! ```
//! // function=add duration=112ns
//! ```
//! Times the execution of the function
//!
//! # Examples
//!
//! ```
//! use timed::timed;
//!
//! #[timed]
//! fn add(x: i32, y: i32) -> i32 {
//!     x + y
//! }
//!
//! #[timed(printer = "println!")]
//! async fn google()  {
//!     // reqwest::get("https://google.com").await;
//! }
//! ```
//!
//! ```ignore
//! #[timed(printer = "info!")]
//! fn add_info(x: i32, y: i32) -> i32 {
//!     x + y
//! }
//! ```
//!
//! ```ignore
//! #[tokio::main]
//! #[timed]
//! async fn main() {
//!     reqwest::get("https://google.com").await;
//! }
//!
//! ```

pub use timed_proc_macros::timed;

#[macro_use]
extern crate lazy_static;

use std::collections::HashMap;
use std::sync::Mutex;
use std::time::Duration;
use crate::Phase::Finish;

lazy_static! {
    static ref TRACES: Mutex<HashMap<String, Vec<Hop>>> = Mutex::new(HashMap::new());
}

#[derive(Eq, PartialEq)]
pub enum TracingStats {
    None,
    Statistics,
}

pub struct Trace {
    id: String,
    processor: fn(&str),
    stats: TracingStats,
}

#[derive(Clone)]
pub enum Phase {
    Start,
    Finish(Duration),
}

impl Phase {
    fn to_string(&self) -> String {
        match self {
            Phase::Start => "Start".to_string(),
            Finish(_) => "Finish".to_string()
        }
    }
}

#[derive(Clone)]
pub struct Hop {
    pub ts: u128,
    pub ph: Phase,
    pub name: String,
}

impl Trace {
    pub fn register(id: &str) {
        TRACES.lock().unwrap().insert(id.to_string(), vec![]);
    }

    pub fn collect(hop: Hop) {
        for trace_group in TRACES.lock().unwrap().iter_mut() {
            trace_group.1.push(hop.clone());
        }
    }

    pub fn dump(id: &str, processor: fn(&str), stats: &TracingStats) {
        let start = std::time::Instant::now();
        let mut traces = TRACES.lock().unwrap();
        let entry = traces.entry(id.to_string()).or_insert(vec![]);
        let mut stats_map = HashMap::new();
        let mut total_time_nanos: u128 = 0;

        processor("[");
        for (i, hop) in entry.iter().enumerate() {
            if stats == &TracingStats::Statistics {
                if let Finish(d) = hop.ph {
                    stats_map.entry(hop.name.clone())
                        .or_insert(vec![])
                        .push(d);
                    total_time_nanos += d.as_nanos();
                }
            }

            let is_last = i == entry.len() - 1;
            let trace = format!(
                "{{ \"pid\": 0, \"ts\": {},  \"ph\": \"{}\", \"name\": \"{}\" }}",
                hop.ts, hop.ph.to_string(), hop.name
            );
            processor(&format!("    {}{}", trace, if !is_last { "," } else { "" }));
        }
        processor("]");

        match stats {
            TracingStats::None => {}
            TracingStats::Statistics => {
                processor("========================\n      Statistics\n========================");
                stats_map.iter().for_each(|(k, v)| {
                    let current_total: u128 = v.iter().map(|d| d.as_nanos()).sum();
                    processor(&format!("- {}\n\t> calls: {:>6}\n\t> total time: {:<11} ({:.5}%)",
                                       k,
                                       v.len(),
                                       format!("{:?}", Duration::from_nanos(current_total as u64)),
                                       100.0 * current_total as f64 / total_time_nanos as f64
                    ));
                });
                processor(&format!("========================\nall functions total time: {:?}", Duration::from_nanos(total_time_nanos as u64)));
            }
        }

        processor(&format!("Dumping traces took {:?}", start.elapsed()));
    }

    pub fn new(id: &str, processor: Option<fn(&str)>, stats: Option<TracingStats>) -> Trace {
        let trace = Trace {
            id: id.into(),
            processor: processor.unwrap_or(|x: &str| {
                println!("{}", x);
            }),
            stats: stats.unwrap_or(TracingStats::None),
        };
        Self::register(&trace.id);
        trace
    }
}

impl Drop for Trace {
    fn drop(&mut self) {
        Trace::dump(&self.id, self.processor, &self.stats);
    }
}

#[macro_export]
macro_rules! init_tracing {
    () => {
        let __trace = timed::Trace::new("Tracing", None, None);
    };
    ($name:expr) => {
        let __trace = timed::Trace::new($name, None, None);
    };
    ($name:expr, $closure:tt) => {
        let __trace = timed::Trace::new($name, Some($closure), None);
    };
    ($name:expr, $closure:tt, $stats:expr) => {
        let __trace = timed::Trace::new($name, Some($closure), Some($stats));
    };
    ($name:expr, $stats:expr) => {
        let __trace = timed::Trace::new($name, None, Some($stats));
    };
}
