use std::time::SystemTime;
use super::{StorageData, StorageKey, StorageValue};

#[derive(Debug, Clone)]
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
        if self.is_expired() {
            return &StorageData::Nil;
        }

        &self.value.data
    }

    pub fn get_ttl(&self) -> Option<i128> {
        if self.is_expired() {
            return Some(-1);
        }

        let now= SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_millis() as i128;
        let expired_at = self.value.expire_at?;

        Some(expired_at - now)
    }

    pub fn set_value(&mut self, value: StorageValue) {
        self.value = value;
    }

    pub fn set_data(&mut self, data: StorageData) {
        self.value.data = data;
    }

    pub fn set_ttl(&mut self, ttl: Option<i128>) {
        if ttl.is_none() {
            self.value.expire_at = None;
            return;
        }

        let ttl = ttl.unwrap();
        let now = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_millis() as i128;

        self.value.expire_at = Some(now + ttl);
    }

    pub fn set_expired_at(&mut self, expired_at: Option<i128>) {
        self.value.expire_at = expired_at;
    }

    pub fn is_nil(&self) -> bool {
        match self.value.data {
            StorageData::Nil => true,
            _ => false,
        }
    }

    pub fn is_expired(&self) -> bool {
        self.value.is_expired()
    }

    pub fn is_primitive(&self) -> bool {
        self.value.data.is_primitive()
    }
}
