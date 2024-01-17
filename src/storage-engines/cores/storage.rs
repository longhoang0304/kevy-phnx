use super::{StorageEntry, StorageError, StorageKey};

pub trait Storage {
    fn read(&self, key: &StorageKey) -> Result<StorageEntry, Box<StorageError>>;
    fn write(&mut self, entry: StorageEntry) -> Result<(), Box<StorageError>>;
    fn delete(&mut self, key: &StorageKey) -> Result<bool, Box<StorageError>>;
}
