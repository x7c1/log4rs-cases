pub type Result<T> = ::std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    FailedToCreateAppender(std::io::Error),
    Log4rsErrors(log4rs::config::Errors),
    LogError(log::SetLoggerError),
    LogParseLevelError(log::ParseLevelError),
}

impl From<log4rs::config::Errors> for Error {
    fn from(e: log4rs::config::Errors) -> Self {
        Error::Log4rsErrors(e)
    }
}

impl From<log::SetLoggerError> for Error {
    fn from(e: log::SetLoggerError) -> Self {
        Error::LogError(e)
    }
}

impl From<log::ParseLevelError> for Error {
    fn from(e: log::ParseLevelError) -> Self {
        Error::LogParseLevelError(e)
    }
}
