mod one {
    pub(crate) mod two {
        pub(crate) mod three {
            #[timed::timed(tracing(enabled = true))]
            pub(crate) fn deep() {
                println!("Deep");
            }
        }
    }
}

#[timed::timed(tracing(enabled = true))]
fn foo() {
    bar();
    baz();
}

#[timed::timed(tracing(enabled = true))]
fn baz() {
    println!("Hello");
    for _ in 0..3 {
        one::two::three::deep();
    }
}

#[timed::timed(tracing(enabled = true))]
fn bar() {
    for _ in 0..10 {
        baz();
    }
}

#[timed::timed(tracing(enabled = true))]
fn main() {

    let trace = timed::Trace::new("Main");

    foo();

    println!("{}", trace.statistics());

}
