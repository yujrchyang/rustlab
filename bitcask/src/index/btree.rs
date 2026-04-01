use parking_lot::RwLock;
use std::{collections::BTreeMap, sync::Arc};

use crate::{data::log_record::LogRecordPos, index::Indexer};

// 主要封装了标准库中的 BTreeMap 结构
pub struct BTree {
    tree: Arc<RwLock<BTreeMap<Vec<u8>, LogRecordPos>>>,
}

impl BTree {
    pub fn new() -> Self {
        Self {
            tree: Arc::new(RwLock::new(BTreeMap::new())),
        }
    }
}

impl Indexer for BTree {
    fn put(&self, key: Vec<u8>, pos: LogRecordPos) -> bool {
        let mut write_guard = self.tree.write();
        write_guard.insert(key, pos);
        true
    }

    fn get(&self, key: Vec<u8>) -> Option<LogRecordPos> {
        let read_guard = self.tree.read();
        read_guard.get(&key).copied()
    }

    fn delete(&self, key: Vec<u8>) -> bool {
        let mut write_guard = self.tree.write();
        let remove_res = write_guard.remove(&key);
        remove_res.is_some()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_btree_put() {
        let bt = BTree::new();
        let res1 = bt.put(
            "".as_bytes().to_vec(),
            LogRecordPos {
                file_id: 1,
                offset: 10,
            },
        );
        assert_eq!(true, res1);

        let res2 = bt.put(
            "aaa".as_bytes().to_vec(),
            LogRecordPos {
                file_id: 11,
                offset: 100,
            },
        );
        assert_eq!(true, res2);
    }

    #[test]
    fn test_btree_get() {
        let bt = BTree::new();
        let res1 = bt.put(
            "".as_bytes().to_vec(),
            LogRecordPos {
                file_id: 1,
                offset: 10,
            },
        );
        assert_eq!(true, res1);

        let res2 = bt.put(
            "aaa".as_bytes().to_vec(),
            LogRecordPos {
                file_id: 11,
                offset: 100,
            },
        );
        assert_eq!(true, res2);

        let pos1 = bt.get("aaa".as_bytes().to_vec());
        assert_eq!(true, pos1.is_some());
        assert_eq!(11, pos1.unwrap().file_id);
        assert_eq!(100, pos1.unwrap().offset);

        let pos2 = bt.get("bbb".as_bytes().to_vec());
        assert_eq!(true, pos2.is_none());
    }

    #[test]
    fn test_btree_del() {
        let bt = BTree::new();
        let res1 = bt.put(
            "".as_bytes().to_vec(),
            LogRecordPos {
                file_id: 1,
                offset: 10,
            },
        );
        assert_eq!(true, res1);

        let res2 = bt.put(
            "aaa".as_bytes().to_vec(),
            LogRecordPos {
                file_id: 11,
                offset: 100,
            },
        );
        assert_eq!(true, res2);

        let pos1 = bt.get("aaa".as_bytes().to_vec());
        assert_eq!(true, pos1.is_some());
        assert_eq!(11, pos1.unwrap().file_id);
        assert_eq!(100, pos1.unwrap().offset);

        let res3 = bt.delete("aaa".as_bytes().to_vec());
        assert_eq!(true, res3);
        let res4 = bt.delete("aaa".as_bytes().to_vec());
        assert_eq!(false, res4);

        let pos2 = bt.get("aaa".as_bytes().to_vec());
        assert_eq!(true, pos2.is_none());
    }
}
