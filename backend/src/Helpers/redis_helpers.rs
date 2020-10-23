use redis::*;

pub fn redis_conn() -> RedisResult<Connection> {
    let client = redis::Client::open("redis://redis");
    return match client {
        Ok(client) => client.get_connection(),
        Err(e) => Err(e)
    }
}