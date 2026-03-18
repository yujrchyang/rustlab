use std::path::PathBuf;

#[derive(Clone)]
pub struct Options {
    // 数据库目录
    pub dir_path: PathBuf,
    // 数据文件大小
    pub data_file_size: u64,
    // 是否每次都持久化到文件
    pub sync_writes: bool,
    // 索引类型
    pub index_type: IndexType,
}

#[derive(Clone)]
pub enum IndexType {
    BTree,    // BTree 索引
    SkipList, // 跳表索引
}
