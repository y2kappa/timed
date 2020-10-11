
#[macro_use]
extern crate log;
use std::{thread, time};
use timed::timed;
mod tests;

mod tracing_sub;

#[timed(tracing=true)]
fn main() {

    // TODO: crate timed::tracing::init!("Test");
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
