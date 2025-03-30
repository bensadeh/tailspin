use std::fmt;

#[derive(Debug)]
pub enum Error {
    RegexErrors(Vec<regex::Error>),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::RegexErrors(errors) => {
                for error in errors {
                    writeln!(f, "{}", error)?;
                }
                Ok(())
            }
        }
    }
}

impl std::error::Error for Error {}
