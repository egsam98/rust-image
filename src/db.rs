use dotenv::dotenv;
use diesel::{SqliteConnection, RunQueryDsl};
use diesel::r2d2::{ Pool as DieselPool, ConnectionManager, PooledConnection };

use std::env;
use diesel::expression::sql_literal::sql;


pub struct SqlitePool {
    pool: DieselPool<ConnectionManager<SqliteConnection>>
}

impl SqlitePool {
    pub fn new() -> SqlitePool {
        dotenv().ok();
        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        let manager = ConnectionManager::<SqliteConnection>::new(database_url);
        SqlitePool {
            pool: DieselPool::builder().build(manager).expect("Failed to create pool")
        }
    }

    pub fn connection(&self) -> PooledConnection<ConnectionManager<SqliteConnection>> {
        self.pool.get().expect("No connection")
    }

    pub fn last_rowid(&self, table_name: &str) -> i32 {
        let query = &format!("select seq from sqlite_sequence where name = '{}';", table_name);
        sql(query).get_result(&self.connection()).expect("Error getting last row ID")
    }
}