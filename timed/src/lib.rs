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

mod interface;
mod chrome_trace;
mod statistics;

pub use interface::*;
pub use chrome_trace::*;
pub use statistics::*;
pub use timed_proc_macros::timed;

#[macro_use]
extern crate lazy_static;
extern crate thiserror;

use crate::Phase::Finish;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum TimedError {
    #[error("Tracing can only be initialized once")]
    TracingInitializationFailed,
    #[error("Tracing finish failed: {0}")]
    TracingFinishFailed(String)
}

type Result<T> = std::result::Result<T, TimedError>;

lazy_static! {
    static ref TRACE: Arc<Mutex<Trace>> = Arc::new(Mutex::new(Trace::empty()));
}

pub struct Trace {
    options: TraceOptions,
    trace_data: Vec<ChromeTraceRecord>,
    is_initialized: bool,
    is_finished: bool,
}

impl Trace {
    pub(crate) fn empty() -> Trace {
        Trace {
            options: TraceOptions::default(),
            trace_data: vec![],
            is_initialized: false,
            is_finished: false
        }
    }
    pub(crate) fn set_options(&mut self, options: TraceOptions) {
        self.options = options;
        self.is_initialized = true;
    }

    pub fn finish(&mut self) {
        self.dump();
        self.is_finished = true;
    }

    fn dump(&self) {
        if self.options.chrome_trace.is_none() && self.options.statistics.is_none() {
            return;
        }

        let mut stats_map = HashMap::new();
        let mut chrome_trace_result = ChromeTraceResult::new();

        self.trace_data.iter().for_each(|chrome_trace_record| {
            if self.options.statistics.is_some() {
                if let Finish(d) = chrome_trace_record.ph {
                    stats_map.entry(chrome_trace_record.name.clone()).or_insert(vec![]).push(d);
                }
            }

            if self.options.chrome_trace.is_some() {
                chrome_trace_result.records.push(chrome_trace_record.clone());
            }
        });

        if let Some(chrome_trace_callback) = self.options.chrome_trace.as_ref() {
            chrome_trace_callback(&chrome_trace_result);
        }

        if let Some(statistics_callback) = self.options.statistics.as_ref() {
            statistics_callback(&StatisticsResult::from_raw_map(&stats_map));
        }
    }
}

pub fn collect(chrome_trace_record: ChromeTraceRecord) {
    TRACE.lock().unwrap().trace_data.push(chrome_trace_record);
}

pub fn init_tracing(options: TraceOptions) -> Result<()> {
    let mut trace = &mut *TRACE.lock().unwrap();

    if !trace.is_initialized {
        trace.set_options(options);

        return Ok(());
    }

    return Err(TimedError::TracingInitializationFailed);
}

pub fn finish_tracing() -> Result<()> {
    let mut trace = &mut *TRACE.lock().unwrap();

    if trace.is_initialized {
        if !trace.is_finished {
            trace.finish();
            return Ok(());
        }

        return Err(TimedError::TracingFinishFailed("Tracing finish can only be called once".to_string()));
    }

    return Err(TimedError::TracingFinishFailed("Tracing initialization must be called before finish".to_string()));
}

#[macro_export]
macro_rules! init_tracing {
    ($options:expr) => {
        {
            let mut trace = *TRACE.lock().unwrap();

            if !trace.is_initialized {
                trace.set_options($options);
            }

            panic!("init_tracing can only be called once");
        }
    }
}