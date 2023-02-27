use crate::error_handler::CustomError;
use diesel::pg::PgConnection;
use diesel::r2d2::ConnectionManager;
use lazy_static::lazy_static;
use r2d2;
use std::env;

type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;
pub type DbConnection = r2d2::PooledConnection<ConnectionManager<PgConnection>>;

// embed_migrations!();

// use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
// pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("../migrations");

lazy_static! {
    static ref POOL: Pool = {
        let db_url = env::var("DATABASE_URL").expect("Database url not set");
        let manager = ConnectionManager::<PgConnection>::new(db_url);
        Pool::new(manager).expect("Failed to create db pool")
    };
}

pub fn init() {
    lazy_static::initialize(&POOL);
    let conn = connection().expect("Failed to get db connection");
    // run_migrations(conn);
    // embedded_migrations::run(&conn).unwrap();
}

pub fn connection() -> Result<DbConnection, CustomError> {
    POOL.get()
        .map_err(|e| CustomError::new(500, format!("Failed getting db connection: {}", e)))
}

// fn run_migrations(
//     connection: &mut impl MigrationHarness<DbConnection>,
// ) -> Result<(), Box<dyn Error + Send + Sync + 'static>> {
//     // This will run the necessary migrations.
//     //
//     // See the documentation for `MigrationHarness` for
//     // all available methods.
//     connection.run_pending_migrations(MIGRATIONS)?;

//     Ok(())
// }
