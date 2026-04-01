use std::{path::PathBuf, sync::Arc};

use parking_lot::RwLock;

use crate::{data::log_record::LogRecord, errors::Result, fio};

// 数据文件
pub struct DataFile {
    // 数据文件 ID
    file_id: Arc<RwLock<u32>>,
    // 当前写偏移
    write_off: Arc<RwLock<u64>>,
    // IO 管理接口
    io_manager: Box<dyn fio::IOManager>,
}

impl DataFile {
    pub fn new(dir_path: PathBuf, file_id: u32) -> Result<DataFile> {
        todo!()
    }

    pub fn write(&self, buf: &[u8]) -> Result<usize> {
        todo!()
    }

    pub fn sync(&self) -> Result<()> {
        todo!()
    }

    pub fn get_write_off(&self) -> u64 {
        let read_guard = self.write_off.read();
        *read_guard
    }

    pub fn get_file_id(&self) -> u32 {
        let read_guard = self.file_id.read();
        *read_guard
    }

    pub fn read_log_record(&self, offset: u64) -> Result<LogRecord> {
        todo!()
    }
}
