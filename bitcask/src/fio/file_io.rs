use log::error;
use parking_lot::RwLock;
use std::fs::OpenOptions;
use std::io::Write;
use std::path::PathBuf;
use std::{fs::File, os::unix::fs::FileExt, sync::Arc};

use crate::errors::{Errors, Result};
use crate::fio::IOManager;

/// 标准文件系统 IO 管理抽象接口
pub struct FileIO {
    fd: Arc<RwLock<File>>, // 文件句柄
}

impl FileIO {
    pub fn new(file_name: PathBuf) -> Result<Self> {
        match OpenOptions::new()
            .create(true)
            .read(true)
            .write(true)
            .append(true)
            .open(file_name)
        {
            Ok(file) => {
                return Ok(Self {
                    fd: Arc::new(RwLock::new(file)),
                });
            }
            Err(e) => {
                error!("open data file error: {}", e);
                return Err(Errors::FailedOpenDataFile);
            }
        }
    }
}

impl IOManager for FileIO {
    fn read(&self, buf: &mut [u8], offset: u64) -> Result<usize> {
        let read_guard = self.fd.read();
        match read_guard.read_at(buf, offset) {
            Ok(n) => return Ok(n),
            Err(e) => {
                error!("read from data file error: {}", e);
                return Err(Errors::FailedReadFromDataFile);
            }
        };
    }

    fn write(&self, buf: &[u8]) -> Result<usize> {
        let mut write_guard = self.fd.write();
        match write_guard.write(buf) {
            Ok(n) => return Ok(n),
            Err(e) => {
                error!("write to data file error: {}", e);
                return Err(Errors::FailedWriteToDataFile);
            }
        };
    }

    fn sync(&self) -> Result<()> {
        let read_guard = self.fd.read();
        if let Err(e) = read_guard.sync_all() {
            error!("sync data file error: {}", e);
            return Err(Errors::FailedSyncDataFile);
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use std::{fs, path::PathBuf};

    use super::*;

    #[test]
    fn test_file_io() {
        let path = PathBuf::from("/tmp/a.data");
        let fio_res = FileIO::new(path.clone());
        assert!(fio_res.is_ok());

        let fio = fio_res.unwrap();
        let res1 = fio.write("key-a".as_bytes());
        assert!(res1.is_ok());
        let res2 = fio.write("key-b".as_bytes());
        assert!(res2.is_ok());
        let res3 = fio.write("key-c".as_bytes());
        assert!(res3.is_ok());
        let res4 = fio.sync();
        assert!(res4.is_ok());

        let mut buf1 = [0; 5];
        let read_res1 = fio.read(&mut buf1, 0);
        assert!(read_res1.is_ok());
        assert_eq!(read_res1.unwrap(), 5);
        let mut buf2 = [0; 5];
        let read_res2 = fio.read(&mut buf2, 5);
        assert!(read_res2.is_ok());
        assert_eq!(read_res2.unwrap(), 5);

        let _ = fs::remove_file(path);
    }
}
