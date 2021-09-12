use main_error::MainResult;
use std::error::Error;
use std::fmt;

// You can use custom types as the error type, if they implement `Debug` and `Display`.

// Debug is required by the Error trait; we can just derive it.
#[derive(Debug)]
/// A custom error type. Can also be a more complex struct or enum.
struct MyError;

// The simplest implementation of the Error trait: no source().
impl Error for MyError {}

// Display is also required by Error trait.
impl fmt::Display for MyError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "human-readable description of MyError")
    }
}

fn main() -> MainResult {
    // NOTE: The try-operator `?` is necessary for implicit conversion to `MainError`.
    Err(MyError)?;

    Ok(())
}
