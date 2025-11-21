use sea_orm::{Statement, ConnectionTrait};
use sea_orm_migration::{prelude::*, sea_query::Expr};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(producto::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(producto::id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(producto::nombre).string().not_null())
                    .col(ColumnDef::new(producto::precio).integer().not_null())
                    .col(ColumnDef::new(producto::stock).integer().not_null())
                    .col(
                        ColumnDef::new(producto::creado_el)
                            .timestamp()
                            .default(Expr::current_timestamp()),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(producto::Table).to_owned())
            .await
    }
}

#[derive(Iden)]
enum producto {
    Table,
    id,
    nombre,
    precio,
    stock,
    creado_el,
}