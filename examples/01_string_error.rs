use main_error::MainError;

fn main() -> Result<(), MainError> {
    // NOTE the try-operator ? is necessary for implicitly converting the String to MainError.
    Err("strings can be used as errors")?;

    Ok(())
}
