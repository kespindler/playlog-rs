# playlog

This is a toy project as I learn rust.

The goal was to make an incredibly easy to use logging API for Rust. What that might look like.


```rust
#[use_macro]
extern crate playlog;

fn main() {
    /// Note, I just wanted a JsonLogger. I wouldn't advocate that Json be the default
    /// handler if this were turned into a library.
    info!("request received",
        a: 1,
        b: 2
    );
}

// Output:
// {"ts":"2018-07-22T07:14:03.870173Z","level":"INFO","name":"playlog::tests","host":"computer.local","pid":1234,"msg":"request received","data":{"a":1,"b":2}}
```


```rust
#[use_macro]
extern crate playlog;

fn main() {
    /// It's not implemented yet, but configuring it might look something like this.
    /// setup_logger(module_path!())
    ///     .level(Level::Info)
    ///     .addHandler(JsonHandler(stdout()))

    info!("request received",
        a: 1,
        b: 2
    );
}

// Output:
// {"ts":"2018-07-22T07:14:03.870173Z","level":"INFO","name":"playlog::tests","host":"computer.local","pid":1234,"msg":"request received","data":{"a":1,"b":2}}
```
