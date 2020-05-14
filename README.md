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

foobar, foo, bar, baz, qux, quux, quuz, corge, grault, garply, waldo, fred, plugh, xyzzy, and thud, Wibble, wobble, wubble, and flob

## Contribution
Contributions are welcome. Please submit PR.

## TODO:
- [ ] attribute to macro such as log level if needed
- [ ] formatting ``fn=name dur=25sec``
- [ ] inspect https://github.com/gustavla/timeit/pulls see how to combine
- [ ] add timeit macro for blocks (looks like already done)

## References
- https://github.com/dtolnay/quote
- https://github.com/alexcrichton/proc-macro2#usage

