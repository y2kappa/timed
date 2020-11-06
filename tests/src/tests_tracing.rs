use std::{thread, time};

use rusty_fork::rusty_fork_test;

use timed;
use timed::{TraceRecord, RecordBuffer};
use std::sync::{Arc, Mutex};
use timed::default_exts::statistics::StatisticsExt;
use timed::default_exts::chrome_trace::ChromeTraceExt;

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

// rusty_fork_test! {
#[test]
#[timed::timed(tracing = true)]
fn test_tracing_chrome_tracing() {
    let mut statistics = RecordBuffer::new();
    let mut chrome_trace = RecordBuffer::new();

    let _ = timed::init_tracing(timed::TraceCollectorChain::new()
        .chain_output(Arc::clone(&statistics))
        .chain_output(Arc::clone(&chrome_trace))).unwrap();

    println!("Running main");
    sleep();
    for i in 0..5 {
        foo();
    }

    // println!("{:?}", statistics.lock().unwrap().drain());
    statistics.lock().unwrap().get_statistics().printstd();
    println!("{}", chrome_trace.lock().unwrap().get_chrome_trace().to_chrome_trace());
}
// }

// rusty_fork_test! {
//     #[test]
//     #[timed::timed(tracing = true)]
//     fn test_tracing_with_stats() {
//         let _ = timed::init_tracing(*timed::TraceCollectorChain::new()
//             .with_statistics(|x: &StatisticsResult| println!("{:?}", x))).unwrap();
//
//         println!("Running main");
//         sleep();
//         foo();
//
//         let _ = timed::finish_tracing().unwrap();
//     }
// }
// rusty_fork_test! {
//     #[test]
//     #[timed::timed(tracing = true)]
//     fn test_tracing_with_both() {
//         let _ = timed::init_tracing(*timed::TraceCollectorChain::new()
//             .with_statistics(|x: &StatisticsResult| println!("{:?}", x))
//             .with_chrome_trace(|x: &ChromeTraceResult| println!("{}", x.to_chrome_trace()))).unwrap();
//
//         println!("Running main");
//         sleep();
//         foo();
//
//         let _ = timed::finish_tracing().unwrap();
//     }
// }
//
// rusty_fork_test! {
//     #[test]
//     #[timed::timed(tracing = true)]
//     fn test_tracing_with_none() {
//         let _ = timed::init_tracing(timed::TraceCollectorChain::new()).unwrap();
//
//         println!("Running main");
//         sleep();
//         foo();
//
//         let _ = timed::finish_tracing().unwrap();
//     }
// }