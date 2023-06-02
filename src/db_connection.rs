use diesel::pg::PgConnection;
use diesel::r2d2;
use diesel::r2d2::{ConnectionManager, Pool};
use dotenv::dotenv;
use std::env;

pub type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;

pub fn create_db_pool() -> Pool<ConnectionManager<PgConnection>> {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool.")
}

#[cfg(test)]
pub mod testing {
    use super::*;
    use diesel::r2d2::CustomizeConnection;
    use diesel::Connection;

    #[derive(Debug)]
    struct AlwaysTestingTransaction {}

    impl<C: Connection, E> CustomizeConnection<C, E> for AlwaysTestingTransaction {
        fn on_acquire(&self, conn: &mut C) -> Result<(), E> {
            conn.begin_test_transaction().unwrap();
            Ok(())
        }
    }

    pub fn create_testing_pool() -> DbPool {
        let _ = env_logger::try_init();
        dotenv().ok();
        let url = env::var("DATABASE_URL").expect("DATABASE_URL not set");

        let always_testing = AlwaysTestingTransaction {};
        let manager = ConnectionManager::<PgConnection>::new(url.as_str());
        r2d2::Builder::new()
            .max_size(1)
            .connection_customizer(Box::new(always_testing))
            .build(manager)
            .expect("Failed to create db pool")
    }
}
