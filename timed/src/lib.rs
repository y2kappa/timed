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

pub use timed_proc_macros::timed as timed;

#[macro_use]
extern crate lazy_static;

use std::sync::Mutex;

pub enum Action {
    Init(String),
    Dump(String),
    Collect(String)
}

use std::collections::HashMap;

lazy_static! {
    static ref TRACES: Mutex<HashMap<String, Vec<String>>> = Mutex::new(HashMap::new());
}

pub fn collect(action: Action) {
    match action {
        Action::Init(id) => {
            TRACES.lock().unwrap().insert(id.clone(), vec![]);
        },
        Action::Dump(id) => {
            let mut traces = TRACES.lock().unwrap();
            let entry = traces.entry(id.clone()).or_insert(vec![]);
            for (i, trace) in entry.iter().enumerate() {
                if i == 0 {
                    println!("[");
                }
                let is_last = i == entry.len() - 1;
                println!("    {}{}", trace, if !is_last { "," } else {""});
                if is_last {
                    println!("]");
                }
            }
        },
        Action::Collect(trace) => {
            for trace_group in TRACES.lock().unwrap().iter_mut() {
                trace_group.1.push(trace.clone());
            }
        }
    }
}

pub struct Trace(String);

impl Trace {
    pub fn new(id: String) -> Trace {
        let trace = Trace(id);
        collect(Action::Init(trace.0.clone()));
        trace
    }
}

impl Drop for Trace {
    fn drop(&mut self) {
        collect(Action::Dump(self.0.clone()));
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
