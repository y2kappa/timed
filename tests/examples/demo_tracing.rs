mod one {
    pub(crate) mod two {
        pub(crate) mod three {
            #[timed::timed(tracing(enabled = true), duration(disabled = true))]
            pub(crate) fn deep() {
                // println!("Deep");
                std::thread::sleep(std::time::Duration::from_millis(1));
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
    baz();
}

#[timed::timed(tracing(enabled = true), duration(disabled = true))]
fn main() {
    let trace = timed::Trace::new("Main");

    foo();
    std::thread::sleep(std::time::Duration::from_millis(10));

    println!("{}", trace.chrome_tracing());
}
