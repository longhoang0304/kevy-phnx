use std::collections::HashMap;

use crate::storage::cores::{Storage, StorageData, StorageEntries, StorageEntry, StorageError, StorageKey, StorageValue};

#[derive(Debug)]
pub struct MemoryStorage {
    pub entries: StorageEntries,
}

impl Storage for MemoryStorage {
    fn read(&self, key: &StorageKey) -> Result<StorageEntry, Box<StorageError>> {
        let value = self.entries.get(key);
        let result: StorageEntry;

        if value.is_none() {
            result = StorageEntry::new(key.to_owned(), StorageValue::new(StorageData::Nil, None))
        } else {
            result = StorageEntry::new(key.to_owned(), value.unwrap().to_owned())
        }

        Ok(result)
    }

    fn write(&mut self, entry: StorageEntry) -> Result<(), Box<StorageError>> {
        self.entries.insert(entry.key, entry.value);

        Ok(())
    }

    fn delete(&mut self, key: &StorageKey) -> Result<bool, Box<StorageError>> {
        let prev_value = self.entries.remove(key);

        Ok(prev_value.is_none())
    }
}

impl MemoryStorage {
    pub fn new(entries: Option<StorageEntries>) -> MemoryStorage {
        MemoryStorage {
            entries: entries.unwrap_or(HashMap::new()),
        }
    }
}
