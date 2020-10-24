#[timed::timed(tracing = true)]
fn foo() {
    std::thread::sleep(std::time::Duration::from_micros(1000));
    bar();
    baz();
}

#[timed::timed(tracing = true)]
fn baz() {
    std::thread::sleep(std::time::Duration::from_micros(1000));
}

#[timed::timed(tracing = true)]
fn bar() {
    baz();
    std::thread::sleep(std::time::Duration::from_micros(1000));
}

#[timed::timed(tracing = true)]
fn main() {
    timed::init_tracing!("Test", timed::TraceOptions::new()
    .with_chrome_trace(
        |x: &str| println!("{}", x)
    ).build());
    foo();
}
