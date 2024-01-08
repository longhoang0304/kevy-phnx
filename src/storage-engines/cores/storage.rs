use super::{StorageEntry, StorageError, StorageKey};

pub trait Storage {
    fn read(&self, key: &StorageKey) -> Result<Option<StorageEntry>, Box<StorageError>>;
    fn write(&mut self, entry: StorageEntry) -> Result<(), Box<StorageError>>;
}