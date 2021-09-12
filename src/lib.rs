#![deny(missing_docs)]

//! Print errors with [`Display`] instead of [`Debug`] when using `?` in `main()`.
//!
//! Use by returning [`MainError`] as the error type from `main()`.
//! Example:
//! 
//! ```should_panic
//! use main_error::MainError;
//! 
//! fn main() -> Result<(), MainError> {
//!     // This prints
//!     //   "Error: invalid digit found in string"
//!     // instead of (if you used `Result<(), Box<dyn Error>>` or similar)
//!     //   "ParseIntError { kind: InvalidDigit }".
//!     let number: i32 = "not a number".parse()?;
//! 
//!     Ok(())
//! }
//! ```
//!
//! For convenience, you can also use the [`MainResult`] type.
//! See below for more details.
//!
//! # The Issue
//!
//! Since [Rust 1.26](https://blog.rust-lang.org/2018/05/10/Rust-1.26.html#main-can-return-a-result), `main()` function can return a [`Result<T, E>`](core::result).
//! This enables the use of `?` for convenient error handling in small programs and quick examples ([RFC](https://github.com/rust-lang/rfcs/pull/1937)).
//!
//! Unfortunately, the error is printed via [`Debug`] ([hardcoded in the standard library](https://doc.rust-lang.org/src/std/process.rs.html), search for "Error:"),
//! which gives not very pretty or human-friendly output.
//! 
//! For example, this program:
//! 
//! ```should_panic
//! # use std::num::ParseIntError;
//! fn main() -> Result<(), ParseIntError> {
//!     let num: i32 = "not a number".parse()?;
//!     // ...
//! #     Ok(())
//! }
//! ```
//!
//! will print
//!
//! ```text
//! Error: ParseIntError { kind: InvalidDigit }
//! ```
//!
//! # Solution
//!
//! This crate provides [`MainError`] as a drop-in replacement for the error type `E` in your `main`'s `Result<T, E>`.
//! It prints the error via [`Display`] instead of [`Debug`], which yields a nicer error message.
//! For example, the program above can be changed to
//!
//! ```should_panic
//! use main_error::MainError;
//!
//! fn main() -> Result<(), MainError> {
//!     let _: i32 = "not a number".parse()?;
//!     // ...
//! #    Ok(())
//! }
//! ```
//!
//! and now prints:
//! ```output
//! Error: invalid digit found in string
//! ```
//!
//! # Details and Drawbacks
//!
//! - [`MainError`] stores the original error as `Box<dyn Error>`.
//!   This incurs one allocation (on conversion) and one virtual call (on printing).
//!   Since there can be exactly one error like this before the program ends, this cost is insignificant.
//! - [`MainError`] implements [`From`] for all types that can be converted into a `Box<dyn Error>`.
//!     1. This allows it to be used in place of any type that implements the [`Error`] trait (see example above).
//!     2. It can also be used in place of any type that can be _converted_ to a `Box<dyn Error>`, e.g., `String`.
//! - [`MainError`] does not implement the [`Error`] trait itself. Reasons:
//!     1. It's not necessary, because the standard library only requires `E: Debug` for `main() -> Result<T, E>`.
//!     2. You should only be using `MainError` for `main()` anyway, whereas the `Error` trait is more for interoparability between libraries.
//!     3. One simply _cannot_ implement `Error` for `MainError`, because this would create an overlapping `impl`.  
//!        Explanation:  
//!        - `MainError` can be converted from a `T: Into<Box<dyn Error>>`.
//!        - `Into<Box<dyn Error>>` [is implemented](std::error::Error#implementors) for `E: Error` itself.
//!        - If `MainError` would implement `Error`, it could be converted from itself.
//!        - This collides with the [reflexive `impl<T> From<T> for T` in core](core::convert::From#generic-implementations).
//! - [`MainError`] implements [`Debug`] in terms of [`Display`] of the underlying error.
//!   This is hacky, but unfortunately [`Debug`] as the output for the `main` error case is stable now.
//!   The `"Error: "` part at the beginning of the output comes [from the standard library](https://doc.rust-lang.org/src/std/process.rs.html), thus it cannot be changed.

use std::error::Error;
use std::fmt::{self, Debug, Display};

/// Newtype wrapper around a boxed [`std::error::Error`].
/// - It implements [`Debug`] so that it can be used in `fn main() -> Result<(), MainError>`.
/// - It implements [`From<E>`](From) for `E: Into<Box<dyn Error>>` so that it works as a drop-in for any type that can be converted into a boxed [`Error`] (i.e., an `Error` trait object).
///
/// `MainError` can only be constructed through its [`From`] impl:
/// Explicitly with `from`/`into` or implicitly through the `?` operator.
///
/// # Example
///
/// Explicit construction via `MainError::from`:
/// ```
/// # use main_error::MainError;
/// let e = MainError::from("something convertible to Box<dyn Error>");
/// ```
///
/// Or via `into()` when the target type can be inferred from the context:
/// ```should_panic
/// # use main_error::MainError;
/// fn main() -> Result<(), MainError> {
///     Err("something convertible to Box<dyn Error>".into())
/// }
/// ```
///
/// Or even easier via `?`:
/// ```should_panic
/// # use main_error::MainError;
/// fn main() -> Result<(), MainError> {
///     Err("something convertible to Box<dyn Error>")?
/// }
/// ```
pub struct MainError(Box<dyn Error>);

impl<E: Into<Box<dyn Error>>> From<E> for MainError {
    fn from(e: E) -> Self {
        MainError(e.into())
    }
}

// impl Debug (to satisfy trait bound for main()-Result error reporting), but use Display of wrapped
// error internally (for nicer output).
impl Debug for MainError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        Display::fmt(&self.0, f)?;
        let mut source = self.0.source();
        while let Some(error) = source {
            write!(f, "\ncaused by: {}", error)?;
            source = error.source();
        }
        Ok(())
    }
}

/// Convenience type as a shorthand return type for `main()`.
pub type MainResult = Result<(), MainError>;
