use main_error::MainError;

// You can also have different error types in main, as long as they can be converted to Box<dyn Error>.
fn main() -> Result<(), MainError> {
    // Produces a ParseIntError.
    let _: i32 = "not a number".parse()?;

    // Produces a Utf8Error.
    let _: &str = std::str::from_utf8(&[159])?;

    // Produces a &str Err() variant.
    Err("str")?;

    Ok(())
}
