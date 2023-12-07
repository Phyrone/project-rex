use std::path::{Path, PathBuf};
use std::time::Duration;

use get_port::tcp::TcpPort;
use get_port::{Ops, Range};
use pg_embed::pg_enums::PgAuthMethod;
use pg_embed::pg_fetch::PgFetchSettings;
use pg_embed::postgres::{PgEmbed, PgSettings};
use sea_orm::{Database, DatabaseConnection};
use tokio::fs::{create_dir, create_dir_all};

mod migration;

pub(crate) struct TestDatabase {
    pub connection: DatabaseConnection,
    embedded_database: PgEmbed,
}

impl TestDatabase {
    pub async fn new(name: &str) -> Self {
        let mut path = PathBuf::from(".");
        path.push("test_data");
        path.push(name);
        path.push("database/");

        println!("creating test database in {}", path.display());

        let port = TcpPort::in_range(
            "127.0.0.1",
            Range {
                min: i16::MAX as u16,
                max: u16::MAX,
            },
        )
        .expect("failed to find an open port for the test postgres database");
        let settings = PgSettings {
            database_dir: path,
            port,
            auth_method: PgAuthMethod::Plain,
            user: "test".to_string(),
            password: "test".to_string(),
            persistent: false,
            timeout: Some(Duration::from_secs(30)),
            migration_dir: None,
        };
        let postgres_fetch_settings = PgFetchSettings::default();
        let mut embedded_database = PgEmbed::new(settings, postgres_fetch_settings)
            .await
            .expect("failed to create embedded postgres database");

        embedded_database
            .acquire_postgres()
            .await
            .expect("failed to acquire embedded postgres database");

        embedded_database
            .setup()
            .await
            .expect("failed to setup embedded postgres database");

        embedded_database
            .start_db()
            .await
            .expect("failed to start embedded postgres database");

        let connection = Database::connect(embedded_database.db_uri.as_str())
            .await
            .expect("failed to connect to embedded postgres database");

        println!("embedded test postgres database started on port {}", port);
        Self {
            embedded_database,
            connection,
        }
    }
}

impl Drop for TestDatabase {
    fn drop(&mut self) {
        println!("dropping embedded test postgres database");
        self.embedded_database
            .stop_db_sync()
            .expect("failed to stop embedded postgres database");
    }
}
