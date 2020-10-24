use std::{thread, time};
use timed;

#[allow(dead_code)]
fn sleep() {
    thread::sleep(time::Duration::from_millis(10));
}

#[timed::timed(tracing = true)]
fn foo() {
    bar();
    sleep();
    baz();
}

#[timed::timed(tracing = true)]
fn bar() {
    sleep();
    baz();
    sleep();
}

#[timed::timed(tracing = true)]
fn baz() {
    sleep();
    foo::bar::baz::foobar();
}

pub mod foo {
    pub mod bar {
        pub mod baz {
            use timed::timed;

            #[timed(tracing = true)]
            pub fn foobar() {
                println!("Foobar");
            }
        }
    }
}

#[test]
#[timed::timed(tracing = true)]
fn test_tracing_chrome_tracing() {
    let trace = timed::TraceOptions::new()
        .with_chrome_trace(|x: &str| println!("{}", x))
        .build_named("Test");

    println!("Running main");
    sleep();
    foo();

    trace.finish();
}

#[test]
#[timed::timed(tracing = true)]
fn test_tracing_with_stats() {
    let trace = timed::TraceOptions::new()
        .with_statistics(|x: &str| println!("{}", x))
        .build_named("TestWithStats");

    println!("Running main");
    sleep();
    foo();

    trace.finish();
}

#[test]
#[timed::timed(tracing = true)]
fn test_tracing_with_both() {
    let trace = timed::TraceOptions::new()
        .with_statistics(|x: &str| println!("{}", x))
        .with_chrome_trace(|x: &str| println!("{}", x))
        .build_named("TestWithBoth");

    println!("Running main");
    sleep();
    foo();

    trace.finish();
}

#[test]
#[timed::timed(tracing = true)]
fn test_tracing_with_none() {
    let trace = timed::TraceOptions::new().build_named("TestWithNone");

    println!("Running main");
    sleep();
    foo();

    trace.finish();
}
