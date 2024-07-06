use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Moi::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(Moi::Id).uuid().not_null().primary_key())
                    .col(ColumnDef::new(Moi::Question).string().not_null())
                    .col(ColumnDef::new(Moi::Answer).string().not_null())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Moi::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Moi {
    Table,
    Id,
    Question,
    Answer,
}
