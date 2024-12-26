use diesel::r2d2::{self, ConnectionManager};
use diesel::PgConnection;

pub type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;

pub fn establish_connection() -> DbPool {
    let manager = ConnectionManager::<PgConnection>::new(
        std::env::var("DATABASE_URL").expect("DATABASE_URL must be set"),
    );
    r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool.")
}
