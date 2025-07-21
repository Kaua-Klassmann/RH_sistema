use sea_orm_migration::prelude::*;

use crate::m20250718_173430_create_sector_table::Sector;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Resume::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Resume::Id)
                            .unsigned()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Resume::Name).string().not_null())
                    .col(ColumnDef::new(Resume::Cpf).string_len(11).not_null())
                    .col(ColumnDef::new(Resume::Phone).string_len(11).not_null())
                    .col(ColumnDef::new(Resume::IdSector).unsigned().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-Resume-setor-id")
                            .from(Resume::Table, Resume::IdSector)
                            .to(Sector::Table, Sector::Id),
                    )
                    .col(ColumnDef::new(Resume::InterviewDate).date_time())
                    .col(
                        ColumnDef::new(Resume::Hired)
                            .boolean()
                            .not_null()
                            .default(false),
                    )
                    .col(ColumnDef::new(Resume::Attachment).string())
                    .col(ColumnDef::new(Resume::Observation).text().not_null())
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx-resume-interview_date")
                    .table(Resume::Table)
                    .col(Resume::InterviewDate)
                    .to_owned(),
            )
            .await?;
        manager
            .create_index(
                Index::create()
                    .name("idx-resume-cpf")
                    .table(Resume::Table)
                    .col(Resume::Cpf)
                    .to_owned(),
            )
            .await?;
        manager
            .create_index(
                Index::create()
                    .name("idx-resume-name")
                    .table(Resume::Table)
                    .col(Resume::Name)
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Resume::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Resume {
    Table,
    Id,
    Name,
    Cpf,
    Phone,
    IdSector,
    InterviewDate,
    Hired,
    Attachment,
    Observation,
}
