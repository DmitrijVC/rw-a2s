use thiserror::Error;


#[derive(Error, Debug)]
pub enum A2SClientError {
    #[error("io error has occurred")]
    IoError(std::io::Error),

    #[error("trying to connect again to the master server")]
    ToMasterConnectionRepeated,
}


#[derive(Error, Debug)]
pub enum ServerError {
    #[error("io error has occurred")]
    IoError(std::io::Error),

    #[error("server is offline")]
    TimedOut,

    #[error("response doesn't contain the prefix")]
    InvalidResponsePrefix,
}
