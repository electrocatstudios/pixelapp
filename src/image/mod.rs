use std::fmt;

pub mod gif;
pub mod png;

pub enum ImageRenderError {
    PercentageNotValid(String)
}

impl fmt::Display for ImageRenderError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ImageRenderError::PercentageNotValid(err_str) => write!(f, "Percentage not valid: {}", err_str),
        }
    }
}