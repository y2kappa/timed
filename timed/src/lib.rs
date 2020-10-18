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
use std::sync::Mutex;
use std::collections::HashMap;

lazy_static! {
    static ref TRACES: Mutex<HashMap<String, Vec<String>>> = Mutex::new(HashMap::new());
}

pub enum TracingStats {
    None,
}

pub struct Trace {
    id: String,
    processor: fn(&str),
    stats: TracingStats,
}

impl Trace {

    pub fn register(id: &str) {
        TRACES.lock().unwrap().insert(id.to_string(), vec![]);
    }

    pub fn collect(trace: String) {
        for trace_group in TRACES.lock().unwrap().iter_mut() {
            trace_group.1.push(trace.clone());
        }
    }

    pub fn dump(id: &str, processor: fn(&str)) {
        let start = std::time::Instant::now();
        let mut traces = TRACES.lock().unwrap();
        let entry = traces.entry(id.to_string()).or_insert(vec![]);
        for (i, trace) in entry.iter().enumerate() {
            if i == 0 {
                processor("[");
            }
            let is_last = i == entry.len() - 1;
            processor(&format!("    {}{}", trace, if !is_last { "," } else { "" }));
            if is_last {
                processor("]");
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
        Trace::dump(&self.id, self.processor);
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
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
