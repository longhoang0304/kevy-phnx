use super::StorageData;

#[derive(Debug, Clone)]
pub struct StorageValue {
    pub data: StorageData,
    pub ttl: Option<i128>,
}

impl StorageValue {
    pub fn new(data: StorageData, ttl: Option<i128>) -> StorageValue {
        StorageValue {
            data,
            ttl,
        }
    }
}
