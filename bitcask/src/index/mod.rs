pub mod btree;

use crate::data::log_record::LogRecordPos;

/// 索引抽象接口，后续如果想要接入不同的索引，只需要实现这个接口
pub trait Indexer: Send + Sync {
    /// 向索引中存储 key 对应的数据位置信息
    fn put(&self, key: Vec<u8>, pos: LogRecordPos) -> bool;
    /// 从索引中获取 key 对应的数据位置信息
    fn get(&self, key: Vec<u8>) -> Option<LogRecordPos>;
    /// 删除 key 对应的数据位置信息
    fn delete(&self, key: Vec<u8>) -> bool;
}
