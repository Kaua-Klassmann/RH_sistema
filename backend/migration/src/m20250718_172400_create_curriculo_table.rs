use sea_orm_migration::prelude::*;

use crate::m20250718_173430_create_setor_table::Setor;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Curriculo::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Curriculo::Id)
                            .unsigned()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Curriculo::Nome).string().not_null())
                    .col(ColumnDef::new(Curriculo::Cpf).string_len(11).not_null())
                    .col(ColumnDef::new(Curriculo::Celular).string_len(11).not_null())
                    .col(ColumnDef::new(Curriculo::IdSetor).unsigned().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-curriculo-setor-id")
                            .from(Curriculo::Table, Curriculo::IdSetor)
                            .to(Setor::Table, Setor::Id),
                    )
                    .col(ColumnDef::new(Curriculo::DataEntrevista).date_time())
                    .col(
                        ColumnDef::new(Curriculo::Contratado)
                            .boolean()
                            .not_null()
                            .default(false),
                    )
                    .col(ColumnDef::new(Curriculo::Anexo).string())
                    .col(ColumnDef::new(Curriculo::Observação).text())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Curriculo::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Curriculo {
    Table,
    Id,
    Nome,
    Cpf,
    Celular,
    IdSetor,
    DataEntrevista,
    Contratado,
    Anexo,
    Observação,
}
