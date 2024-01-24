use std::time::SystemTime;

use super::StorageData;

#[derive(Debug, Clone)]
pub struct StorageValue {
    pub data: StorageData,
    pub expire_at: Option<i128>,
}

impl StorageValue {
    pub fn new(data: StorageData, expire_at: Option<i128>) -> StorageValue {
        StorageValue {
            data,
            expire_at,
        }
    }

    pub fn is_expired(&self) -> bool {
        let expire_at = &self.expire_at;
        if expire_at.is_none() {
            return false;
        }

        let now = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_millis() as i128;
        let expire_at = expire_at.unwrap();

        if expire_at <= now {
            return true;
        }

        false
    }
}
