
#[macro_use]
extern crate log;
use std::{thread, time};
use std::time::{Duration, SystemTime};
use timed::timed;
use serde::{Serialize, Deserialize};
mod tests;
use serde_json::Result;
use lazy_static::lazy_static;
use std::sync::Mutex;

mod tracing_sub;

lazy_static! {
    static ref TRACES: Mutex<Vec<String>> = Mutex::new(vec![]);
}

struct Tracing;
impl Drop for Tracing {
    fn drop(&mut self) {
        let traces = TRACES.lock().unwrap();
        println!("Begin Dumping traces:\n-----");
        println!("[");
        for i in 0..traces.len() {
            println!("    {}{}", traces[i], if i == traces.len() - 1 { "" } else { ","});
        }
        println!("]");
        println!("-----\nEnd Dumping traces");
    }
}

pub fn collect_trace(trace: String) {
    TRACES.lock().unwrap().push(trace)
}


#[timed(tracing=true)]
fn main() {

    let _trace = timed_tracing::Trace::new("Test".to_string());

    println!("Running main");
    sleep();
    foo();
}

fn sleep() {
    thread::sleep(time::Duration::from_millis(10));
}

#[timed(tracing=true)]
fn foo() {
    bar();
    sleep();
    baz();
}

#[timed(tracing=true)]
fn bar() {
    sleep();
    baz();
    sleep();
}

#[timed(tracing=true)]
fn baz() {
    sleep();
    tracing_sub::foo::bar::baz::foobar();
}
