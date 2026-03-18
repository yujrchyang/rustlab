#[derive(PartialEq)]
pub enum LogRecordType {
    // 正常 put 的数据
    NORMAL = 1,
    // 被删除的数据标识，墓碑
    DELETED = 2,
}
// 数据日志结构体，表示实际写到数据文件中的数据
// 之所以叫日志，是因为数据文件中的数据是追加写入的，类似日志的格式
pub struct LogRecord {
    pub(crate) key: Vec<u8>,
    pub(crate) value: Vec<u8>,
    pub(crate) rec_type: LogRecordType,
}

impl LogRecord {
    pub fn encode(&self) -> Vec<u8> {
        todo!()
    }
}

// 数据位置索引信息，描述数据存储到了哪个位置
#[derive(Clone, Copy, Debug)]
pub struct LogRecordPos {
    pub(crate) file_id: u32,
    pub(crate) offset: u64,
}

// 从数据文件中读取的 log_record 信息
pub struct ReadLogRecord {
    pub(crate) record: LogRecord,
    pub(crate) size: u64,
}
