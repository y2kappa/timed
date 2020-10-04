# Ideas / Plans / Wishlist

- [x] Add and test async
- [x] Use syn, remove manual parser
- [x] Attribute to macro such as log level if needed:
    - macro/function to call when timing such as "info", "println", "lambda"
- [ ] formatting ``fn=name dur=25sec``
    - allow for custom formatting
- [ ] add possibility to print start, end
    - [ ] eventually generate flamegraph out of it
    - [ ] or (vertical) timegraph with overlaps
    - [ ] use chrome::tracing (need to generate a new file or print to stdout)
- [ ] add possibility to happen only during debug mode, not release
- [ ] inspect https://github.com/gustavla/timeit/pulls see how to combine
- [ ] add timed macro for blocks
    - timeit does it many times (looks like already done)
    - this is just for profiling
- [ ] allow hooks `|name, duration| {}` to be inserted
- [ ] can have a global state (for testing maybe) that tracks how many times something was called


## Chrome tracing:
- [ ] Keep accumulating in a global state
- [ ] use timed::dump/flush on stdout/file when useful

## info!()
- write another wrapper for info! that dumps to chrome::tracing as well