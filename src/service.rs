use crate::cache_proto::cache_server::Cache;
use crate::cache_proto::{Key, KeyValue, Value};
use crate::lru::LRU;
use crate::storage::Storage;
use std::sync::Arc;
use tokio::sync::Mutex;
use tonic::{Request, Response, Status};

pub struct CacheService {
    cache: Arc<Mutex<LRU>>,
    storage: Arc<Mutex<Storage>>,
}

impl CacheService {
    pub(crate) fn new(n: u8, path: String) -> Self {
        CacheService {
            cache: Arc::new(Mutex::new(LRU::new(n))),
            storage: Arc::new(Mutex::new(Storage::new(path))),
        }
    }
}

impl Default for CacheService {
    fn default() -> Self {
        CacheService {
            cache: Arc::new(Mutex::new(LRU::default())),
            storage: Arc::new(Mutex::new(Storage::default())),
        }
    }
}

#[tonic::async_trait]
impl Cache for CacheService {
    async fn get(&self, request: Request<Key>) -> Result<Response<Value>, Status> {
        let key = request.into_inner().key;
        let mut cache = self.cache.lock().await;
        match cache.get(key) {
            Some(v) => Ok(Response::new(Value { value: v })),
            None => Ok(Response::new(Value { value: Vec::new() })),
        }
    }

    async fn set(&self, request: Request<KeyValue>) -> Result<Response<()>, Status> {
        let req = request.into_inner();
        let mut cache = self.cache.lock().await;
        cache.set(req.key, req.value);
        
        let mut storage = self.storage.lock().await;
        match storage.save(cache.to_json().unwrap().as_bytes()) {
                Ok(_) => {}
                Err(e) => println!("error during saving to file: {}", e)
        };

        Ok(Response::new(()))
    }
}
