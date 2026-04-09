pub mod file_io;

use crate::errors::Result;

/// IO 管理抽象接口
pub trait IOManager: Sync + Send {
    /// 从文件的给定位置读取对应的数据
    fn read(&self, buf: &mut [u8], offset: u64) -> Result<usize>;
    /// 将数据写入文件
    fn write(&self, buf: &[u8]) -> Result<usize>;
    /// 持久化数据
    fn sync(&self) -> Result<()>;
}
