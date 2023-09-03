#[derive(Debug)]
pub enum MyError {
    InvalidFuture,
    // Add other error variants as needed
}

impl core::fmt::Display for MyError {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self {
            MyError::InvalidFuture => write!(f, "\nInvalid Task"),
            // Handle other error variants here
        }
    }
}
