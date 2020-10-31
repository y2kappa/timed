use std::{thread, time};
use timed;
use timed::{StatisticsResult, ChromeTraceResult};
use rusty_fork::rusty_fork_test;

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

rusty_fork_test! {
    #[test]
    #[timed::timed(tracing = true)]
    fn test_tracing_chrome_tracing() {
        let _ = timed::init_tracing(*timed::TraceOptions::new()
            .with_chrome_trace(|x: &ChromeTraceResult| println!("{}", x.to_string()))).unwrap();

        println!("Running main");
        sleep();
        foo();

        let _ = timed::finish_tracing().unwrap();
    }
}

rusty_fork_test! {
    #[test]
    #[timed::timed(tracing = true)]
    fn test_tracing_with_stats() {
        let _ = timed::init_tracing(*timed::TraceOptions::new()
            .with_statistics(|x: &StatisticsResult| println!("{:?}", x))).unwrap();

        println!("Running main");
        sleep();
        foo();

        let _ = timed::finish_tracing().unwrap();
    }
}
rusty_fork_test! {
    #[test]
    #[timed::timed(tracing = true)]
    fn test_tracing_with_both() {
        let _ = timed::init_tracing(*timed::TraceOptions::new()
            .with_statistics(|x: &StatisticsResult| println!("{:?}", x))
            .with_chrome_trace(|x: &ChromeTraceResult| println!("{}", x.to_string()))).unwrap();

        println!("Running main");
        sleep();
        foo();

        let _ = timed::finish_tracing().unwrap();
    }
}

rusty_fork_test! {
    #[test]
    #[timed::timed(tracing = true)]
    fn test_tracing_with_none() {
        let _ = timed::init_tracing(timed::TraceOptions::new()).unwrap();

        println!("Running main");
        sleep();
        foo();

        let _ = timed::finish_tracing().unwrap();
    }
}