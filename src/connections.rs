use std::env;

use diesel::prelude::*;

use redis::Client;


pub fn db_connection() -> PgConnection {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}
pub fn redis_connection() -> redis::Connection {
    let redis_url = env::var("REDIS_URL").expect("REDIS_URL must be set");
    let client = Client::open(redis_url).unwrap();
    // return client.get_connection().expect("Error to connection to redis");
    
    return client.get_connection()
        .unwrap_or_else(|err| panic!("Error connecting to {}", err))
}
