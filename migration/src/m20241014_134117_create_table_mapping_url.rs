use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[derive(DeriveIden)]
enum UrlMapping {
    Table,
    Id,
    SourceUrl,
    TargetUrlProtocol,
    TargetUrlDomain,
    TargetUrlPath,
    CreatedAt,
    ExpiredAt,
    DeletedAt,
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(UrlMapping::Table)
                    .if_not_exists()
                    .col(pk_auto(UrlMapping::Id))
                    .col(string(UrlMapping::SourceUrl).char_len(255).not_null())
                    .col(
                        string(UrlMapping::TargetUrlProtocol)
                            .char_len(255)
                            .not_null(),
                    )
                    .col(string(UrlMapping::TargetUrlDomain).char_len(255).not_null())
                    .col(string(UrlMapping::TargetUrlPath).char_len(255).not_null())
                    .col(date_time(UrlMapping::CreatedAt).not_null())
                    .col(date_time(UrlMapping::ExpiredAt).null())
                    .col(date_time(UrlMapping::DeletedAt))
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(UrlMapping::Table).to_owned())
            .await
    }
}
