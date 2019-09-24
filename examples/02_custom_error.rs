use main_error::MainError;
use std::error::Error;
use std::fmt;

// Debug is required by Error trait.
#[derive(Debug)]
/// My custom error type; can also be a more complex struct or enum.
struct MyError();

// The simplest Error impl: no implementation for source().
impl Error for MyError {}

// Display is also required by Error trait.
impl fmt::Display for MyError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "human-readable description of MyError")
    }
}

fn main() -> Result<(), MainError> {
    // NOTE the try-operator ? is necessary for implicitly converting to MainError.
    Err(MyError())?;

    Ok(())
}
