mod one {
    pub(crate) mod two {
        pub(crate) mod three {
            #[timed::timed(tracing(enabled = true), duration(disabled = true))]
            pub(crate) fn deep() {
                // println!("Deep");
            }
        }
    }
}

#[timed::timed(tracing(enabled = true), duration(disabled = true))]
fn foo() {
    bar();
    baz();
}

#[timed::timed(tracing(enabled = true), duration(disabled = true))]
fn baz() {
    // println!("Hello");
    for _ in 0..3 {
        one::two::three::deep();
    }
}

#[timed::timed(tracing(enabled = true), duration(disabled = true))]
fn bar() {
    for _ in 0..10 {
        baz();
    }
}

#[timed::timed(tracing(enabled = true), duration(disabled = true))]
fn main() {
    let trace = timed::Trace::new("Main");

    foo();
    std::thread::sleep(std::time::Duration::from_millis(1000));

    println!("{}", trace.statistics());
}
