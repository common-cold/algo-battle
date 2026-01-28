use std::env;

use redis::{aio::ConnectionManager};

#[derive(Clone)]
pub struct RedisConnection {
    pub connection_manager: ConnectionManager
}

impl RedisConnection {
    pub async fn init_redis() -> anyhow::Result<Self> {
        let redis_url = env::var("REDIS_URL")?;
        let client=  redis::Client::open(redis_url)?;
        let connection_manager = ConnectionManager::new(client).await?;
        
        Ok(Self {
            connection_manager: connection_manager
        })
    }
}

