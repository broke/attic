use sea_orm_migration::prelude::*;

use crate::database::entity::object::*;

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20261002_000002_add_object_nar_id_index"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // The garbage collector finds orphan NARs by joining nar to object on
        // nar_id, and deleting a NAR has to find the objects that reference
        // it. Without an index, SQLite runs that join as one scan of object
        // per NAR.
        manager
            .create_index(
                Index::create()
                    .name("idx-object-nar-id")
                    .table(Entity)
                    .col(Column::NarId)
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_index(
                Index::drop()
                    .name("idx-object-nar-id")
                    .table(Entity)
                    .to_owned(),
            )
            .await
    }
}
