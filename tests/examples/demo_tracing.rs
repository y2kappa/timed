#[timed::timed(tracing = true)]
fn foo() {
    bar();
    baz();
}

#[timed::timed(tracing = true)]
fn baz() {
    println!("Hello")
}

#[timed::timed(tracing = true)]
fn bar() {
    baz();
}

#[timed::timed(tracing = true)]
fn main() {
    let trace = timed::TraceCollectorChain::new()
        .with_chrome_trace(|x: &str| println!("{}", x))
        .with_statistics(|x: &str| println!("{}", x))
        .build_named("Main");

    foo();

    trace.finish();
}
