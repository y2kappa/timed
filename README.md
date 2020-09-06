# Rust `timed` macro to time function execution


## Usage

```toml
[dependencies]
timed = "*"
```

```rust
extern crate timed;

#[timed::timed]
fn add(x: i32, y: i32) -> i32 {
    x + y
}

#[timed::timed]
fn mul(x: i32, y: i32) -> i32 {
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

## Contribution
Contributions are welcome. Please submit PR.

## TODO:
- [x] Add and test async
- [x] Use syn, remove manual parser
- [ ] Attribute to macro such as log level if needed:
    - macro/function to call when timing such as "info", "println", "lambda"
- [ ] formatting ``fn=name dur=25sec``
    - allow for custom formatting
- [ ] add possibility to print start, end 
    - [ ] eventually generate flamegraph out of it 
    - [ ] or (vertical) timegraph with overlaps
- [ ] inspect https://github.com/gustavla/timeit/pulls see how to combine
- [ ] add timed macro for blocks 
    - timeit does it many times (looks like already done)
    - this is just for profiling

- [ ] can have a global state (for testing maybe) that tracks how many times something was called

## References
- https://github.com/dtolnay/quote
- https://github.com/alexcrichton/proc-macro2#usage

