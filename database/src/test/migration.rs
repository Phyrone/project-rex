use tokio::time::Instant;

use migration::{Migrator, MigratorTrait};

use crate::test::TestDatabase;

#[tokio::test]
async fn test_schema() {
    let test_database = TestDatabase::new("test_migrator_up_all").await;

    println!("running migration (up) on test database");
    let time = Instant::now();
    Migrator::up(&test_database.connection, None)
        .await
        .expect("failed to migrate database");
    let time = time.elapsed();
    println!("run migration (up) in {:?}", time);

    println!("running migration (refresh) on test database");
    let time = Instant::now();
    Migrator::refresh(&test_database.connection)
        .await
        .expect("failed to refresh database");
    let time = time.elapsed();
    println!("run migration (refresh) in {:?}", time);

    println!("running migration (down) on test database");
    let time = Instant::now();
    Migrator::down(&test_database.connection, None)
        .await
        .expect("failed to drop database schema");
    let time = time.elapsed();
    println!("run migration (down) in {:?}", time);
}
