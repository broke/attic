use sea_orm::DatabaseBackend;
use sea_orm_migration::prelude::*;

use crate::database::entity::nar::*;

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20261002_000001_restore_nar_hash_index"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // On SQLite, m20230112_000005_drop_old_nar_columns rebuilds the nar
        // table and drops idx-nar-nar-hash along with the old one. Every
        // SQLite database that ran it has no index on nar_hash, so each NAR
        // lookup scans the whole table. Other backends drop the columns in
        // place and still have the index.
        if manager.get_database_backend() != DatabaseBackend::Sqlite {
            return Ok(());
        }

        manager
            .create_index(
                Index::create()
                    .name("idx-nar-nar-hash")
                    .table(Entity)
                    .col(Column::NarHash)
                    .if_not_exists()
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        if manager.get_database_backend() != DatabaseBackend::Sqlite {
            return Ok(());
        }

        manager
            .drop_index(
                Index::drop()
                    .name("idx-nar-nar-hash")
                    .table(Entity)
                    .if_exists()
                    .to_owned(),
            )
            .await
    }
}
