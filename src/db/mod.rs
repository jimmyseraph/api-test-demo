use mysql::{Opts, OptsBuilder, Pool};

pub struct MySQLConfig {
    pub host: String,
    pub port: u32,
    pub username: String,
    pub password: String,
    pub dbname: String,
    pub pool: Pool,
}

impl MySQLConfig {
    pub fn new(host: String, port: u32, username: String, password: String, dbname: String) -> Self {
        let opts_builder = OptsBuilder::new()
            .ip_or_hostname(Some(host.clone()))
            .tcp_port(port.try_into().unwrap())
            .user(Some(username.clone()))
            .pass(Some(password.clone()))
            .db_name(Some(dbname.clone()))
            .prefer_socket(false);
        let opts = Opts::from(opts_builder);
        let pool = Pool::new(opts).unwrap();
        Self {
            host,
            port,
            username,
            password,
            dbname,
            pool,
        }
    }

    pub fn get_conn(&self) -> mysql::PooledConn {
        self.pool.get_conn().unwrap()
    }

    pub fn get_pool(&self) -> &Pool {
        &self.pool
    }
}
