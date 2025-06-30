use std::{
    fmt,
    error::Error
};

#[derive(Debug)]
pub enum ClassHandlerError {
    Property,
    ClassName
}

impl fmt::Display for ClassHandlerError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let description = match *self {
            ClassHandlerError::Property => "class didn't have the property value",
            ClassHandlerError::ClassName => "you didn't add a class name",
        };
        f.write_str(description)
    }
}

impl Error for ClassHandlerError {}