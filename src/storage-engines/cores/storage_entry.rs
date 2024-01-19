use super::{StorageData, StorageKey, StorageValue};

pub struct StorageEntry {
    pub key: StorageKey,
    pub value: StorageValue,
}

impl StorageEntry {
    pub fn new(key: StorageKey, value: StorageValue) -> StorageEntry {
        StorageEntry {
            key,
            value,
        }
    }

    pub fn get_key(&self) -> &StorageKey {
        &self.key
    }

    pub fn get_data(&self) -> &StorageData {
        &self.value.data
    }

    pub fn get_ttl(&mut self) -> &Option<i128> {
        &self.value.ttl
    }

    pub fn set_value(&mut self, value: StorageValue) {
        self.value = value;
    }

    pub fn set_data(&mut self, data: StorageData) {
        self.value.data = data;
    }

    pub fn set_ttl(&mut self, ttl: Option<i128>) {
        self.value.ttl = ttl;
    }

    pub fn is_nil(&self) -> bool {
        match self.value.data {
            StorageData::Nil => true,
            _ => false,
        }
    }

    pub fn is_primitive(&self) -> bool {
        self.value.data.is_primitive()
    }
}
