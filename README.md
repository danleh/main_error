# main_error

Print errors with `Display` instead of `Debug` when using `?` in `main()`. For example:

```Rust
use main_error::MainError;

fn main() -> Result<(), MainError> {
    Err("string or a custom error type")? // prints using Display, not Debug
}
```

For more info, see: 
- [Package information](https://crates.io/crates/main_error) on crates.io
- [Documentation](https://docs.rs/main_error/) on Docs.rs
- [Usage examples](https://github.com/danleh/main_error/tree/master/examples) in the repo