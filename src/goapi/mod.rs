use std::env;

pub mod sayhi;
pub mod modify;

// 环境参数结构体
#[derive(Debug)]
pub struct DBConfig {
    pub host: String,
    pub port: u32,
    pub username: String,
    pub password: String,
}

#[derive(Debug)]
pub struct RedisConfig {
    pub url: String,
}

#[derive(Debug)]
pub struct Env {
    pub url: String,
    pub db_conf: DBConfig,
    pub redis_conf: RedisConfig,
}

pub fn get_env() -> Env {
    // 假设两个环境，一个是test，一个是prod
    let env_test = Env {
        url: "http://localhost:10004".to_string(),
        db_conf: DBConfig {
            host: "localhost".to_string(),
            port: 3306,
            username: "coffee".to_string(),
            password: "Coffee123!".to_string(),
        },
        redis_conf: RedisConfig {
            url: "redis://localhost:6379".to_string(),
        },
    };
    let env_prod: Env = Env{
        url: "http://testops.vip:10004".to_string(),
        db_conf: DBConfig {
            host: "testops.vip".to_string(),
            port: 3306,
            username: "coffee".to_string(),
            password: "Coffee123!".to_string(),
        },
        redis_conf: RedisConfig {
            url: "redis://testops.vip:6379".to_string(),
        },
    };
    env::var("ENV").unwrap_or("test".to_string())
        .eq("prod")
        .then(|| env_prod)
        .unwrap_or(env_test)
}