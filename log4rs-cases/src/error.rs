extern crate failure;

pub type Result<T> = ::std::result::Result<T, Error>;

#[derive(Fail, Debug)]
pub enum Error {
    #[fail(display = "std::io error")]
    StdIoError(std::io::Error),

    #[fail(display = "log4rs::config::Errors")]
    Log4rsErrors(log4rs::config::Errors),

    #[fail(display = "log::SetLoggerError")]
    LogError(log::SetLoggerError),

    #[fail(display = "log::ParseLevelError: RUST_LOG invalid?")]
    LogParseLevelError(log::ParseLevelError),

//    #[fail(display = "std::env::VarError")]
//    EnvVarError {
//        cause: std::env::VarError,
//        name: String,
//    },
}

impl From<std::io::Error> for Error {
    fn from(e: std::io::Error) -> Self {
        Error::StdIoError(e)
    }
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
