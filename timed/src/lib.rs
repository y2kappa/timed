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

#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate prettytable;
extern crate thiserror;

use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use thiserror::Error;

pub use interface::*;
pub use statistics::*;
pub use timed_proc_macros::timed;
pub use trace::*;

use crate::Phase::Finish;

mod interface;
mod trace;
mod statistics;
pub mod default_exts;

#[derive(Error, Debug)]
pub enum TimedError {
    #[error("Tracing can only be initialized once")]
    TracingInitializationFailed,
    #[error("Tracing finish failed: {0}")]
    TracingFinishFailed(String),
}

type Result<T> = std::result::Result<T, TimedError>;

lazy_static! {
    static ref CHAIN: Arc<Mutex<TraceCollectorChain>> = Arc::new(Mutex::new(TraceCollectorChain::new()));
}

pub fn collect(trace_record: TraceRecord) {
    CHAIN.lock().unwrap().buffers.iter_mut().for_each(|collector|
        collector.lock().unwrap().add(trace_record.clone()));
}

pub fn init_tracing(chain: &mut TraceCollectorChain) -> Result<()> {
    let mut trace = &mut *CHAIN.lock().unwrap();

    if trace.buffers.is_empty() {
        trace.buffers.append(&mut chain.buffers);

        return Ok(());
    }

    return Err(TimedError::TracingInitializationFailed);
}

#[macro_export]
macro_rules! anonymous_collector {
    ($closure:expr) => {
        Box::new((|| {
            struct Anonymous;
            impl timed::Collector for Anonymous {
                fn collect(&mut self, record: &timed::TraceRecord) {
                    $closure(record);
                }
            }
            Anonymous
        })())
    }
}