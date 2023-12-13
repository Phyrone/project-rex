use sea_orm_migration::prelude::*;

use crate::sea_orm::DatabaseBackend;

const SCRIPT_UP1: &str = include_str!("../resources/m001_up.sql");
const SCRIPT_DOWN1: &str = include_str!("../resources/m001_down.sql");

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        if manager.get_database_backend() != DatabaseBackend::Postgres {
            return Err(DbErr::Custom("everything but postgres is not supported (yet?)".to_string()));
        }
        //using raw sql since we want to rely on database features like triggers, stored procedures, and check constraints
        //these are not supported by sea_orm yet
        //since we also only support postgresql, we can use raw sql without worrying about portability
        manager.get_connection().execute_unprepared(SCRIPT_UP1).await?;
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .get_connection()
            .execute_unprepared(SCRIPT_DOWN1)
            .await?;
        Ok(())
    }
}

#[derive(DeriveIden)]
enum Account {
    Table,
    Id,
    Username,
}
