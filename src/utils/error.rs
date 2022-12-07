pub enum BadAppleError {
    IoError(std::io::Error),
    OpencvError(opencv::Error),
}

impl std::fmt::Display for BadAppleError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BadAppleError::IoError(error) => write!(f, "Error occured during IO: {}", error),
            BadAppleError::OpencvError(error) => write!(f, "Error occured during OpenCV: {}", error),
        }
    }
}

impl From<std::io::Error> for BadAppleError {
    fn from(error: std::io::Error) -> Self {
        BadAppleError::IoError(error)
    }
}

impl From<opencv::Error> for BadAppleError {
    fn from(error: opencv::Error) -> Self {
        BadAppleError::OpencvError(error)
    }
}

pub fn error_handler(error: BadAppleError) {
    eprintln!("Error: {}", error);
}