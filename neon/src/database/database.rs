use crate::config;

use std::time::Duration;

use diesel::{
    r2d2::{ConnectionManager, Pool, PooledConnection},
    MysqlConnection,
};

lazy_static::lazy_static! {
   static ref CONNECTION_POOL: Pool<ConnectionManager<MysqlConnection>> = {
      let config = config::get_config();
      let database_url = config.database.connection_url.clone().unwrap_or("mysql://root@localhost:3306/kobayashi".into());

      let build_result = Pool::builder()
         .max_size(config.database.pool_size.unwrap_or(num_cpus::get() as u32 * 4))
         .connection_timeout(Duration::from_secs(5))
         .build(ConnectionManager::new(database_url));

      match build_result {
         Ok(pool) => {
            log::info!("Successfully created database pool.");
            pool
         },
         Err(error) => {
            log::error!("Couldn't create database pool: {}, exiting.", error);
            std::process::exit(1)
         },
      }
   };
}

#[inline(always)]
pub fn get_pool<'a>() -> &'a Pool<ConnectionManager<MysqlConnection>> {
    &*CONNECTION_POOL
}

#[inline(always)]
pub fn get_connection() -> PooledConnection<ConnectionManager<MysqlConnection>> {
    match get_pool().get() {
        Ok(connection) => connection,
        Err(error) => {
            log::error!("Couldn't establish database connection: {}, exiting.", error);
            std::process::exit(1)
        },
    }
}
