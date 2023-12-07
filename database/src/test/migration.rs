use crate::test::TestDatabase;
use migration::{Migrator, MigratorTrait};

//#[tokio::test]
//FIXME: this test fails because the testdatabase cannot be created
async fn test_migrator() {
    let test_database = TestDatabase::new("test_migrator_up_all").await;

    Migrator::up(&test_database.connection, None)
        .await
        .expect("failed to migrate database");
}
