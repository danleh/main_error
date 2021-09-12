use main_error::MainResult;

// You can use plain strings (owned or not) as the error type.

// NOTE: Uses the `MainResult` type as a shorthand for `Result<(), MainError>`.
fn main() -> MainResult {
    // NOTE: The try-operator `?` is necessary for implicit conversion to `MainError`.
    Err("strings can be used as errors")?;

    Ok(())
}
