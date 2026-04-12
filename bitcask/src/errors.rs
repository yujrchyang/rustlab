use std::result;

#[derive(thiserror::Error, Debug, PartialEq)]
pub enum Errors {
    #[error("failed to read from data file")]
    FailedReadFromDataFile,

    #[error("failed to write to data file")]
    FailedWriteToDataFile,

    #[error("failed to sync data file")]
    FailedSyncDataFile,

    #[error("failed to open data file")]
    FailedOpenDataFile,

    #[error("the key is empty")]
    KeyIsEmpty,

    #[error("memory index failed to update")]
    IndexUpdateFailed,

    #[error("key is not found in database")]
    KeyNotFound,

    #[error("data file is not found in database")]
    DataFileNotFound,

    #[error("database directory path can not be empty")]
    DirPathIsEmpty,

    #[error("database data file size must be greater than 0")]
    DataFileSizeIsZero,

    #[error("failed to create database directory")]
    FailedCreateDatabaseDirectory,

    #[error("failed to read database directory")]
    FailedReadDatabaseDirectory,

    #[error("database directory maybe corrupted")]
    DatabaseDirectoryCorrupted,

    #[error("read data file eof")]
    ReadDataFileEOF,
}

pub type Result<T> = result::Result<T, Errors>;
