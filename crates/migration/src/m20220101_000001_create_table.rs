use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(QuestionAnswer::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(QuestionAnswer::Id)
                            .uuid()
                            .not_null()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(QuestionAnswer::Source).string().not_null())
                    .col(ColumnDef::new(QuestionAnswer::Question).string())
                    .col(ColumnDef::new(QuestionAnswer::Answer).string())
                    .to_owned(),
            )
            .await?;
        manager
            .create_table(
                Table::create()
                    .table(Conversation::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Conversation::Id)
                            .uuid()
                            .not_null()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Conversation::Role).string().not_null())
                    .col(
                        ColumnDef::new(Conversation::Datetime)
                            .date_time()
                            .not_null(),
                    )
                    .col(ColumnDef::new(Conversation::Content).string())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(QuestionAnswer::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(Conversation::Table).to_owned())
            .await
    }
}

#[derive(Iden)]
enum QuestionAnswer {
    Table,
    Id,
    Source,
    Question,
    Answer,
}

#[derive(Iden)]
enum Conversation {
    Table,
    Id,
    Role,
    Content,
    Datetime,
}
