# main_error

Print errors with `Display` instead of `Debug` when using `?` in `main()`. For example:

```Rust
use main_error::MainError;

fn main() -> Result<(), MainError> {
    // This prints
    //   "Error: invalid digit found in string"
    // instead of (if you used `Result<(), Box<dyn Error>>` or similar)
    //   "ParseIntError { kind: InvalidDigit }".
    let number: i32 = "not a number".parse()?;

    Ok(())
}
```

For more info, see: 
- [Package information](https://crates.io/crates/main_error) on crates.io
- [Documentation](https://docs.rs/main_error/) on Docs.rs
- [Usage examples](https://github.com/danleh/main_error/tree/master/examples) in the repo
