# Rust `timed` macro to time function execution

✅ Works with `async`
✅ Works with `main`
✅ Custom printers, like `println!`, `info!`, or your own function.

## Usage

```toml
[dependencies]
timed = "0.1.1"
log = "0.4"
```

```rust
use timed::timed;
#[macro_use] extern crate log;

#[timed::timed]
fn add(x: i32, y: i32) -> i32 {
    x + y
}

#[timed]
fn mul(x: i32, y: i32) -> i32 {
    x * y
}

#[timed(printer = "println!")]
fn mul_println(x: i32, y: i32) -> i32 {
    x * y
}

#[timed(printer = "info!")]
fn mul_info(x: i32, y: i32) -> i32 {
    x * y
}


#[test]
fn timing() {
    assert_eq!(add(1, 2), 3);
    assert_eq!(mul(1, 2), 2);
}
```


Output:

```
$ cargo test -- --nocapture
running 1 test

function=add duration=114ns
function=mul duration=97ns


test timing ... ok
```

Also works with main and tokio:

```rs 
#[tokio::main]
#[timed]
async fn main() {
    println!("Running main");
    reqwest::get("https://google.com").await;
}
```

## Contribution
Contributions are welcome. Please submit PR.
See [TODO](TODO.md)