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
//! #[timed(duration(printer = "println!"))]
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

use std::collections::HashMap;
use std::sync::Mutex;

#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate prettytable;

mod chrome_trace;
mod hop;
mod statistics;
mod trace;

// export Trace
pub use hop::{Hop, Phase};
pub use trace::Trace;

// Re-exporting the timed proc macro
pub use timed_proc_macros::timed;

// Keeping track of traces
lazy_static! {
    static ref TRACES: Mutex<HashMap<String, Vec<hop::Hop>>> = Mutex::new(HashMap::new());
}
