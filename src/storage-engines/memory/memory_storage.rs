use std::collections::HashMap;
use std::ops::Index;

use crate::storage::cores::{Storage, StorageEntries, StorageEntry, StorageError, StorageKey};

#[derive(Debug)]
pub struct MemoryStorage {
    pub entries: StorageEntries,
}

impl Storage for MemoryStorage {
    fn read(&self, key: &StorageKey) -> Result<Option<StorageEntry>, Box<StorageError>> {
        let value = self.entries.index(key);
        let result = Some(StorageEntry::new(key.to_owned(), value.to_owned()));

        Ok(result)
    }

    fn write(&mut self, entry: StorageEntry) -> Result<(), Box<StorageError>> {
        self.entries.insert(entry.key, entry.value);

        Ok(())
    }
}

impl MemoryStorage {
    pub fn new(entries: Option<StorageEntries>) -> MemoryStorage {
        MemoryStorage {
            entries: entries.unwrap_or(HashMap::new()),
        }
    }
}
