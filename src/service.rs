use std::sync::{Arc};
use tokio::sync::Mutex;
use tonic::{Request, Response, Status};
use crate::cache_proto::cache_server::Cache;
use crate::cache_proto::{Key, KeyValue, Value};
use crate::lru::LRU;

pub struct CacheService {
    cache: Arc<Mutex<LRU>>,
}

impl Default for CacheService {
    fn default() -> Self {
        CacheService {
            cache: Arc::new(Mutex::new(LRU::default()))
        }
    }
}

#[tonic::async_trait]
impl Cache for CacheService {
    async fn get(&self, request: Request<Key>) -> Result<Response<Value>, Status> {
        let key = request.into_inner().key;
        let mut map = self.cache.lock().await;
        match map.get(key) {
            Some(v) => {
                Ok(Response::new(Value {
                    value: v,
                }))
            }
            None => {
                Ok(Response::new(Value {
                    value: Vec::new(),
                }))
            }
        }
    }

    async fn set(&self, request: Request<KeyValue>) -> Result<Response<()>, Status> {
        let req = request.into_inner();
        let mut map = self.cache.lock().await;
        map.set(req.key, req.value);
        Ok(Response::new(()))
    }
}