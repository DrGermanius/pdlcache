use tonic::transport::Server;
use tonic_reflection::server::Builder;
use cache_proto::cache_server::CacheServer;
use crate::service::CacheService;

mod lru;
mod service;

mod cache_proto {
    include!("cache.rs");

    pub(crate) const FILE_DESCRIPTOR_SET: &[u8] =
        tonic::include_file_descriptor_set!("cache_descriptor");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "127.0.0.1:9001".parse()?;
    let cache_service = CacheService::default();

    let reflection_service = Builder::configure()
        .register_encoded_file_descriptor_set(cache_proto::FILE_DESCRIPTOR_SET)
        .build()
        .unwrap();

    println!("{} {}", "run on: ", addr);

    Server::builder()
        .add_service(CacheServer::new(cache_service))
        .add_service(reflection_service)
        .serve(addr)
        .await?;
    Ok(())
}