use std::time::Duration;

use get_port::{Ops, Range};
use get_port::tcp::TcpPort;
use pg_embed::pg_enums::PgAuthMethod;
use pg_embed::pg_fetch::{PgFetchSettings, PostgresVersion};
use pg_embed::postgres::{PgEmbed, PgSettings};
use sea_orm::{DatabaseConnection, SqlxPostgresConnector};
use sqlx::PgPool;
use temp_testdir::TempDir as TestTempDir;
use tempdir::TempDir;

mod migration;

pub(crate) struct TestDatabase {
    pub connection: DatabaseConnection,
    pub pg_pool: PgPool,
    embedded_database: PgEmbed,
    test_temp_dir: TestTempDir,
}

impl TestDatabase {
    pub async fn new(name: &str) -> Self {
        let temp_dir = TempDir::new("test_database")
            .expect("failed to create temp directory for the test postgres database");

        let temp_dir = TestTempDir::new(temp_dir.path(), true);
        println!("creating test database in {}", temp_dir.display());
        assert!(temp_dir.exists());
        assert!(temp_dir.is_dir());

        let port = TcpPort::in_range(
            "127.0.0.1",
            Range {
                min: i16::MAX as u16,
                max: u16::MAX,
            },
        )
            .expect("failed to find an open port for the test postgres database");
        let settings = PgSettings {
            database_dir: temp_dir.to_path_buf(),
            port,
            auth_method: PgAuthMethod::Plain,
            user: "test".to_string(),
            password: "test".to_string(),
            persistent: false,
            timeout: Some(Duration::from_secs(30)),
            migration_dir: None,
        };
        let postgres_fetch_settings = PgFetchSettings {
            version: PostgresVersion("16.1.1"),
            ..Default::default()
        };

        let download_url = format!(
            "{}/maven2/io/zonky/test/postgres/embedded-postgres-binaries-{}/{}/embedded-postgres-binaries-{}-{}.jar",
            &postgres_fetch_settings.host,
            &postgres_fetch_settings.platform(),
            &postgres_fetch_settings.version.0,
            &postgres_fetch_settings.platform(),
            &postgres_fetch_settings.version.0
        );

        let mut embedded_database = PgEmbed::new(settings, postgres_fetch_settings)
            .await
            .expect("failed to create embedded postgres database");

        println!("downloading postgres binaries from {} if needed", download_url);
        embedded_database
            .setup()
            .await
            .expect("failed to setup embedded postgres database");

        embedded_database
            .start_db()
            .await
            .expect("failed to start embedded postgres database");

        embedded_database
            .create_database("test")
            .await
            .expect("failed to create database");
        let pg_pool = PgPool::connect(embedded_database.db_uri.as_str())
            .await
            .expect("failed to connect to embedded postgres database");

        let connection = SqlxPostgresConnector::from_sqlx_postgres_pool(pg_pool.clone());

        println!(
            "embedded test postgres database started on port {}...",
            port
        );
        Self {
            embedded_database,
            pg_pool,
            connection,
            test_temp_dir: temp_dir,
        }
    }
}

impl Drop for TestDatabase {
    fn drop(&mut self) {
        println!(
            "dropping embedded test postgres database and deleting {}",
            self.test_temp_dir.display()
        );
        self.embedded_database
            .stop_db_sync()
            .expect("failed to stop embedded postgres database");
    }
}
