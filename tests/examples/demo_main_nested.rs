use timed::timed;

#[timed]
fn foo() {
    println!("foo");
}

#[timed]
fn main() {
    println!("Running main");
    foo();
}
