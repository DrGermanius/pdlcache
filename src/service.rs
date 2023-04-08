use std::sync::{Arc};
use tokio::sync::Mutex;
use tonic::{Request, Response, Status};
use tonic::Code::InvalidArgument;
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
        return match key {
            None => Err(Status::invalid_argument(InvalidArgument.to_string())),
            Some(_key) => {
                let mut map = self.cache.lock().await;
                match map.get(_key) {
                    Some(v) => {
                        Ok(Response::new(Value {
                            value: Option::from(v),
                        }))
                    }
                    None => {
                        Ok(Response::new(Value {
                            value: None,
                        }))
                    }
                }
            }
        };
    }

    async fn set(&self, request: Request<KeyValue>) -> Result<Response<()>, Status> {
        // TODO avoid unwrap
        let req = request.into_inner();
        let v = req.value.unwrap();
        let k = req.key.unwrap();

        let mut map = self.cache.lock().await;
        map.set(k, v);
        Ok(Response::new(()))
    }
}