# Backtrace error submission crate

# Installation
```
[dependencies]
backtrace-rust = "0.1"
```

# Usage

```rust
extern crate backtrace_rust;
```

## Global error handler

Pass your custom token and upload url from your Backtrace account and a report
modification closure/function to the `backtrace_rust::register_error_handler`
function.

```
backtrace_rust::register_error_handler(
    "https://UNIVERSE.sp.backtrace.io:6098",
    "YOURTOKEN",
    closure
);

```

### `Report` modification function
The error handler will pass the `Report` and `PanicInfo` objects back to the
user, in case there are additional attributes/annotations to be defined
(described more in detail [here][1]). It should accept `&mut Report`,
`&PanicInfo` and return a modified `Report`.

# Example

```rust
extern crate backtrace_rust;
extern crate num_cpus;

use backtrace_rust::Report;
use std::panic::PanicInfo;

fn main() {
    backtrace_rust::register_error_handler(
        "https://UNIVERSE.sp.backtrace.io:6098",
        "YOUR_TOKEN",
        |r: &mut Report, _: &PanicInfo| -> Report {
            let cpus = num_cpus::get();
            let cpus = cpus.to_string();
            r.attributes.insert(String::from("cpu.cores"), cpus);

            r.clone()
        },
    );

    println!("Hello, world!");
    panic!("{:?}", 69);
}

```

[1]: https://api.backtrace.io/#tag/submit-crash