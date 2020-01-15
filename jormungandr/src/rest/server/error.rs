use std::io;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("failed to open server certificate file")]
    ServerCertFileOpenFailed(#[source] io::Error),
    #[error("failed to parse server certificate file")]
    ServerCertFileParsingFailed,
    #[error("server certificate file contains no certificates")]
    ServerCertFileEmpty,
    #[error("failed to open server private key file")]
    ServerPrivKeyFileOpenFailed(#[source] io::Error),
    #[error("failed to parse server private key file")]
    ServerPrivKeyFileParsingFailed,
    #[error("server private key file should contain 1 key, contains {0}")]
    ServerPrivKeyFileKeyCountInvalid(usize),
    #[error("failed to set server certificate")]
    SetCertFailed(#[source] rustls::TLSError),
    #[error("failed to bind the port")]
    BindFailed(#[source] io::Error),
    #[error("couldn't stop server, it's already stopped")]
    ServerAlreadyStopped,
    #[error("timeout during server stopping")]
    ServerStopTimeout,
    #[error("failed to stop server")]
    ServerStopFailed,
}
