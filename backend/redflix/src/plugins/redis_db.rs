extern crate redis;

pub struct RedisEngine { }

#[allow(dead_code)]
impl RedisEngine {

    pub fn run() -> redis::RedisResult<redis::Connection> {
        let client = redis::Client::open("redis://localhost:6379")?;
        let con = client.get_connection()?;
        Ok(con)
    }

    pub fn get(key: &str) -> redis::RedisResult<String> {
        let name = redis::cmd("GET").arg(key).query(&mut RedisEngine::run()?)?;
        Ok(name)
    }

    pub fn set<T: redis::ToRedisArgs>(key: &str, value: T) -> redis::RedisResult<()> {
        redis::cmd("SET").arg(key).arg(value).query(&mut RedisEngine::run()?)?;
        Ok(())
    }
}