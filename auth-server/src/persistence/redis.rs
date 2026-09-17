#[derive(Debug)]
pub(super) enum RedisError {
    ConnectionError,
}

pub(super) async fn get_redis_connection() -> Result<redis::aio::MultiplexedConnection, RedisError>
{
    let redis_url = std::env::var("REDIS_URL").expect("REDIS_URL must be set");

    let client = redis::Client::open(redis_url).map_err(|error| {
        eprintln!("Failed to create redis client: {:?}", error);
        RedisError::ConnectionError
    })?;

    client
        .get_multiplexed_async_connection()
        .await
        .map_err(|error| {
            eprintln!("Failed to connect to redis: {:?}", error);
            RedisError::ConnectionError
        })
}
