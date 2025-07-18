use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Setor::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Setor::Id)
                            .unsigned()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Setor::Nome).string_len(20).not_null())
                    .to_owned(),
            )
            .await?;

        manager
            .exec_stmt(
                Query::insert()
                    .into_table(Setor::Table)
                    .columns([Setor::Nome])
                    .values_panic(["Acabamento".into()])
                    .values_panic(["Administrativo".into()])
                    .values_panic(["Comercial".into()])
                    .values_panic(["Expedição".into()])
                    .values_panic(["Extrusão".into()])
                    .values_panic(["Impressão".into()])
                    .values_panic(["PCP".into()])
                    .values_panic(["Refile".into()])
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Setor::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum Setor {
    Table,
    Id,
    Nome,
}
