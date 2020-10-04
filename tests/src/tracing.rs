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

#[derive(Debug, Serialize)]
struct Trace<'a> {
    cat: &'a str,
    pid: u32,
    tid: u32,
    ts: u128,
    ph: TraceEvent,
    name: &'a str,
    args: Vec<String>
}

#[derive(Debug, Serialize)]
enum TraceEvent {
    B,
    E
}

impl<'a> Trace<'a> {
    fn begin(cat: &'a str, name: &'a str) -> Self {
        Trace {
            cat,
            pid: 0,
            tid: 0,
            ts: SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_micros(),
            ph: TraceEvent::B,
            name,
            args: vec![]
        }
    }

    fn end(cat: &'a str, name: &'a str) -> Self {
        Trace {
            cat,
            pid: 0,
            tid: 0,
            ts: SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_micros(),
            ph: TraceEvent::E,
            name,
            args: vec![]
        }
    }
}


pub fn collect_trace(trace: String) {
    TRACES.lock().unwrap().push(trace)
}

impl Drop for Trace<'_> {
    fn drop(&mut self) {
        crate::collect_trace(serde_json::to_string(&self).unwrap());
    }
}

#[timed]
fn main() {
    let _trace = Tracing;
    println!("Running main");
    sleep();
    foo();
}

fn sleep() {
    thread::sleep(time::Duration::from_millis(10));
}

#[timed]
fn foo() {
    Trace::begin("Tracing", "foo");

    bar();
    sleep();
    baz();

    Trace::end("Tracing", "foo");
}

#[timed]
fn bar() {
    Trace::begin("Tracing", "bar");

    sleep();
    baz();
    sleep();

    Trace::end("Tracing", "bar");
}

#[timed]
fn baz() {
    Trace::begin("Tracing", "baz");

    sleep();

    Trace::end("Tracing", "baz");
}
