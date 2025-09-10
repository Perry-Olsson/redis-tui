use std::error::Error;

use redis::{cmd, Connection, RedisError, ToRedisArgs};

pub fn get_client() -> Result<RedisRsClient, RedisError> {
    let con = redis::Client::open("redis://127.0.0.1/")?.get_connection()?;
    Ok(RedisRsClient { con })
}

pub trait RedisClient {
    fn get<T: ToRedisArgs>(&mut self, key: &T) -> Result<Option<String>, Box<dyn Error>>;

    fn set<T: ToRedisArgs>(&mut self, key: &T, val: &T) -> Result<(), Box<dyn Error>>;
}

pub struct RedisRsClient {
    con: Connection
}

impl RedisClient for RedisRsClient {
    fn get<T: ToRedisArgs>(&mut self, key: &T) -> Result<Option<String>, Box<dyn Error>> {
        let res = cmd("GET").arg(key).query(&mut self.con)?;
        Ok(res)
    }

    fn set<T: ToRedisArgs>(&mut self, key: &T, val: &T) -> Result<(), Box<dyn Error>> {
        cmd("SET").arg(key).arg(val).exec(&mut self.con)?;
        Ok(())
    }
}
