use std::{collections::BTreeMap, sync::Arc};

use parking_lot::RwLock;

use crate::{data::log_record::LogRecordPos, index::Indexer};

/// BTree 索引，主要封装了标准库的 BTreeMap 结构
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
        let mut write_gurad = self.tree.write();
        write_gurad.insert(key, pos);
        true
    }

    fn get(&self, key: Vec<u8>) -> Option<LogRecordPos> {
        let read_gurad = self.tree.read();
        read_gurad.get(&key).copied()
    }

    fn delete(&self, key: Vec<u8>) -> bool {
        let mut write_gurad = self.tree.write();
        let remove_res = write_gurad.remove(&key);
        remove_res.is_some()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_btree() {
        let btree = BTree::new();
        let res1 = btree.put(
            "".as_bytes().to_vec(),
            LogRecordPos {
                file_id: 1,
                offset: 10,
            },
        );
        assert_eq!(res1, true);
        let res2 = btree.put(
            "aa".as_bytes().to_vec(),
            LogRecordPos {
                file_id: 2,
                offset: 20,
            },
        );
        assert_eq!(res2, true);

        let res3 = btree.get("aa".as_bytes().to_vec());
        assert_eq!(
            res3,
            Some(LogRecordPos {
                file_id: 2,
                offset: 20
            })
        );
        let res4 = btree.get("bb".as_bytes().to_vec());
        assert_eq!(res4, None);

        let res5 = btree.delete("aa".as_bytes().to_vec());
        assert_eq!(res5, true);
        let res6 = btree.delete("aa".as_bytes().to_vec());
        assert_eq!(res6, false);

        let res7 = btree.get("aa".as_bytes().to_vec());
        assert_eq!(res7, None);
    }
}
