use std::{thread, time};

#[macro_use]
use rusty_fork::rusty_fork_test;

use std::sync::Arc;
use timed;

#[allow(dead_code)]
fn sleep() {
    thread::sleep(time::Duration::from_millis(10));
}

#[timed::timed(tracing(enabled = true))]
fn outer_sleep() {
    sleep();
    inner_sleep();
}

#[timed::timed(tracing(enabled = true))]
fn inner_sleep() {
    sleep();
}

#[test]
#[timed::timed(tracing(enabled = true))]
fn test_tracing_statistics() {
    let mut statistics = RecordBuffer::new();

    let _ = timed::init_tracing(
        timed::TraceCollectorChain::new().chain_output(Arc::clone(&statistics)),
    )
    .unwrap();

    println!("Running main");
    outer_sleep();

    statistics.lock().unwrap().get_statistics().printstd();
}

#[test]
#[timed::timed(tracing(enabled = true))]
fn test_tracing_chrome_trace() {
    let mut chrome_trace = RecordBuffer::new();

    let _ = timed::init_tracing(
        timed::TraceCollectorChain::new().chain_output(Arc::clone(&chrome_trace)),
    )
    .unwrap();

    println!("Running main");
    outer_sleep();

    println!(
        "{}",
        chrome_trace
            .lock()
            .unwrap()
            .get_chrome_trace()
            .to_chrome_trace()
    );
}
