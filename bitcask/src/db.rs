use std::{collections::HashMap, fs, path::PathBuf, sync::Arc};

use bytes::Bytes;
use log::warn;
use parking_lot::RwLock;

use crate::{
    data::{
        data_file::{DATA_FILE_NAME_SUFFIX, DataFile},
        log_record::{LogRecord, LogRecordPos, LogRecordType},
    },
    errors::{Errors, Result},
    index,
    options::Options,
};

const INITIAL_FILE_ID: u32 = 0;

// bitcask 存储引擎实例结构体
pub struct Engine {
    options: Arc<Options>,
    older_files: Arc<RwLock<HashMap<u32, DataFile>>>, // 旧数据文件
    active_file: Arc<RwLock<DataFile>>,               // 当前活跃文件
    file_ids: Vec<u32>,                               // 数据库启动时的文件 id，只用于加载索引是使用
    index: Box<dyn index::Indexer>,                   // 数据内存索引
}

impl Engine {
    // 打开 bitcask 存储引擎
    pub fn open(opts: Options) -> Result<Self> {
        // 校验用户传递过来的配置项
        if let Some(e) = check_options(&opts) {
            return Err(e);
        }

        let options = opts.clone();
        // 判断数据目录是否存在，如果不存在则创建
        let dir_path = options.dir_path.clone();
        if !dir_path.is_dir() {
            if let Err(e) = fs::create_dir_all(dir_path.as_path()) {
                warn!("create database directory err: {}", e);
                return Err(Errors::FailedToCreateDatabaseDir);
            }
        }

        // 生成数据文件列表
        let mut data_files = load_data_files(dir_path.clone())?;

        // 生成 file id 列表
        let mut file_ids = Vec::new();
        for v in data_files.iter() {
            file_ids.push(v.get_file_id());
        }

        // 将旧数据文件保存到 older_files 中
        let mut older_files = HashMap::new();
        if data_files.len() > 1 {
            for _ in 0..=data_files.len() - 2 {
                let file = data_files.pop().unwrap();
                older_files.insert(file.get_file_id(), file);
            }
        }

        // 拿到当前活跃文件，即列表中的最后一个文件
        let active_file = match data_files.pop() {
            Some(v) => v,
            None => DataFile::new(dir_path.clone(), INITIAL_FILE_ID)?,
        };

        // 构造存储引擎实例
        let engine = Self {
            options: Arc::new(opts),
            older_files: Arc::new(RwLock::new(older_files)),
            active_file: Arc::new(RwLock::new(active_file)),
            file_ids: file_ids,
            index: Box::new(index::new_indexer(options.index_type)),
        };

        engine.load_index_from_data_files()?;

        Ok(engine)
    }

    // 存储 key/value 数据，key 不能为空
    pub fn put(&self, key: Bytes, value: Bytes) -> Result<()> {
        // 判断 key 的有效性
        if key.is_empty() {
            return Err(Errors::KeyIsEmpty);
        }

        // 构造 LogRecord
        let record = LogRecord {
            key: key.to_vec(),
            value: value.to_vec(),
            rec_type: LogRecordType::NORMAL,
        };

        // 追加写到当前活跃文件中
        let log_record_pos = self.append_log_record(&record)?;

        // 更新内存索引
        let ok = self.index.put(key.to_vec(), log_record_pos);
        if !ok {
            return Err(Errors::IndexUpdateFailed);
        }
        Ok(())
    }

    // 根据 key 删除对应的数据
    pub fn delete(&self, key: Bytes) -> Result<()> {
        // 判断 key 的有效性
        if key.is_empty() {
            return Err(Errors::KeyIsEmpty);
        }

        // 从内存索引中取出对应的数据，不存在直接返回
        let pos = self.index.get(key.to_vec());
        if pos.is_none() {
            return Ok(());
        }

        // 构造 logrecord，标识是被删除的
        let mut record = LogRecord {
            key: key.to_vec(),
            value: Default::default(),
            rec_type: LogRecordType::DELETED,
        };

        // 写入到数据文件当中
        self.append_log_record(&mut record)?;

        // 删除内存索引
        let ok = self.index.delete(key.to_vec());
        if !ok {
            return Err(Errors::IndexUpdateFailed);
        }

        Ok(())
    }

    // 根据 key 获取对应的数据
    pub fn get(&self, key: Bytes) -> Result<Bytes> {
        // 判断 key 的有效性
        if key.is_empty() {
            return Err(Errors::KeyIsEmpty);
        }

        // 从内存索引中拿到对应的位置信息
        let pos = self.index.get(key.to_vec());
        if pos.is_none() {
            return Err(Errors::KeyNotFound);
        }
        let log_record_pos = pos.unwrap();

        // 从对应的数据文件中读取对应的 LogRecord
        let active_file = self.active_file.read();
        let older_files = self.older_files.read();
        let log_record = match active_file.get_file_id() == log_record_pos.file_id {
            true => active_file.read_log_record(log_record_pos.offset)?.record,
            false => {
                let data_file = older_files.get(&log_record_pos.file_id);
                if data_file.is_none() {
                    return Err(Errors::DataFileNotFound);
                }
                data_file
                    .unwrap()
                    .read_log_record(log_record_pos.offset)?
                    .record
            }
        };

        // 判断 LogRecord 类型
        if log_record.rec_type == LogRecordType::DELETED {
            return Err(Errors::KeyNotFound);
        }

        // 返回对应的 value
        Ok(log_record.value.into())
    }

    // 追加写数据到当前活跃文件中
    fn append_log_record(&self, record: &LogRecord) -> Result<LogRecordPos> {
        let dir_path = self.options.dir_path.clone();

        // 输入数据进行编码
        let enc_record = record.encode();
        let record_len = enc_record.len() as u64;

        // 获取到当前活跃文件
        let mut active_file = self.active_file.write();

        // 判断当前活跃文件大小是否达到阈值
        if active_file.get_write_off() + record_len > self.options.data_file_size {
            // 持久化当前活跃文件
            active_file.sync()?;

            // 将旧数据文件插入到 map 中
            let current_fid = active_file.get_file_id();
            let mut older_files = self.older_files.write();
            let old_file = DataFile::new(dir_path.clone(), current_fid)?;
            older_files.insert(current_fid, old_file);

            // 打开新的数据文件
            let new_file = DataFile::new(dir_path.clone(), current_fid + 1)?;
            *active_file = new_file;
        }

        // 追加写数据到当前活跃文件中
        let write_off = active_file.get_write_off();
        active_file.write(&enc_record)?;

        // 根据配置项决定是否 sync
        if self.options.sync_writes {
            active_file.sync()?;
        }

        // 构造内存索引信息
        Ok(LogRecordPos {
            file_id: active_file.get_file_id(),
            offset: write_off,
        })
    }

    // 从数据文件中加载内存索引
    // 遍历数据文件中的内容，并依次处理其中的记录
    fn load_index_from_data_files(&self) -> Result<()> {
        // 数据文件为空，直接返回
        if self.file_ids.is_empty() {
            return Ok(());
        }

        let active_file = self.active_file.read();
        let older_files = self.older_files.read();

        // 遍历每个文件 id，取出对应的数据文件，并加载其中的数据
        for (i, file_id) in self.file_ids.iter().enumerate() {
            let mut offset = 0;
            loop {
                let log_record_res = match *file_id == active_file.get_file_id() {
                    true => active_file.read_log_record(offset),
                    false => {
                        let data_file = older_files.get(file_id).unwrap();
                        data_file.read_log_record(offset)
                    }
                };

                let (log_record, size) = match log_record_res {
                    Ok(result) => (result.record, result.size),
                    Err(e) => {
                        if e == Errors::ReadDataFileEOF {
                            break;
                        }
                        return Err(e);
                    }
                };

                // 构建内存索引
                let log_record_pos = LogRecordPos {
                    file_id: *file_id,
                    offset: offset,
                };

                let ok = match log_record.rec_type {
                    LogRecordType::NORMAL => {
                        self.index.put(log_record.key.to_vec(), log_record_pos)
                    }
                    LogRecordType::DELETED => self.index.delete(log_record.key.to_vec()),
                };
                if !ok {
                    return Err(Errors::IndexUpdateFailed);
                }

                // 递增 offset
                offset += size;
            }

            // 设置当前活跃文件的 offset
            if i == self.file_ids.len() - 1 {
                active_file.set_write_off(offset);
            }
        }

        Ok(())
    }
}

fn load_data_files(dir_path: PathBuf) -> Result<Vec<DataFile>> {
    let dir = fs::read_dir(dir_path.clone());
    if dir.is_err() {
        return Err(Errors::DataFileNotFound);
    }

    let mut file_ids: Vec<u32> = Vec::new();
    let mut data_files: Vec<DataFile> = Vec::new();
    for file in dir.unwrap() {
        if let Ok(entry) = file {
            // 拿到文件名
            let file_os_str = entry.file_name();
            let file_name = file_os_str.to_str().unwrap();

            // 判断文件名称是否以 .data 结尾
            if file_name.ends_with(DATA_FILE_NAME_SUFFIX) {
                let split_name: Vec<&str> = file_name.split(".").collect();
                let file_id = match split_name[0].parse::<u32>() {
                    Ok(fid) => fid,
                    Err(_) => {
                        return Err(Errors::DataDirectoryCorrupted);
                    }
                };
                file_ids.push(file_id);
            }
        }
    }

    // 如果没有数据文件，则直接返回
    if file_ids.is_empty() {
        return Ok(data_files);
    }

    // 对文件 id 进行排序，从小到大进行加载
    file_ids.sort();
    // 遍历所有的文件 id，依次打开对应的数据文件
    for file_id in file_ids.iter() {
        let data_file = DataFile::new(dir_path.clone(), *file_id)?;
        data_files.push(data_file);
    }
    Ok(data_files)
}

fn check_options(opts: &Options) -> Option<Errors> {
    let dir_path = opts.dir_path.to_str();
    if dir_path.is_none() || dir_path.unwrap().len() == 0 {
        return Some(Errors::DirPathIsEmpty);
    }
    if opts.data_file_size <= 0 {
        return Some(Errors::FailedToReadDatabaseDir);
    }
    None
}
