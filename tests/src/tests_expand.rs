#[timed::timed(tracing(enabled = true))]
fn foo() {
    std::thread::sleep(std::time::Duration::from_micros(1000));
    bar();
    baz();
}

#[timed::timed(tracing(enabled = true))]
fn baz() {
    std::thread::sleep(std::time::Duration::from_micros(1000));
}

#[timed::timed(tracing(enabled = true))]
fn bar() {
    baz();
    std::thread::sleep(std::time::Duration::from_micros(1000));
}

#[timed::timed(tracing(enabled = true))]
fn main() {
    let trace = timed::Trace::new("Main");

    foo();

}
