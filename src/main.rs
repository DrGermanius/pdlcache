use crate::service::CacheService;
use cache_proto::cache_server::CacheServer;
use config::Config;
use std::collections::HashMap;
use tonic::transport::Server;
use tonic_reflection::server::Builder;

mod lru;
mod service;
mod storage;

mod cache_proto {
    include!("cache.rs");

    pub(crate) const FILE_DESCRIPTOR_SET: &[u8] =
        tonic::include_file_descriptor_set!("cache_descriptor");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cfg_file = Config::builder()
        .add_source(config::File::with_name("config"))
        // .add_source(config::Environment::with_prefix("APP"))
        .build()
        .unwrap();

    let cfg = cfg_file
        .try_deserialize::<HashMap<String, String>>()
        .unwrap();
    let size = cfg.get("cache_size");
    let cache_service: CacheService;

    match size {
        None => {
            println!("{}", "default cache size");
            cache_service = CacheService::default()
        }
        Some(s) => {
            let cache_size = s.parse::<u8>().unwrap();
            cache_service = CacheService::new(cache_size, "./cache.json".to_string());
            println!("{} : {}", "cache size", cache_size);
        }
    }

    let addr = "127.0.0.1:9001".parse()?;
    let reflection_service = Builder::configure()
        .register_encoded_file_descriptor_set(cache_proto::FILE_DESCRIPTOR_SET)
        .build()
        .unwrap();

    println!("{} : {}", "run on", addr);

    Server::builder()
        .add_service(CacheServer::new(cache_service))
        .add_service(reflection_service)
        .serve(addr)
        .await?;
    Ok(())
}
