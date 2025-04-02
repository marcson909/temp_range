use sea_orm_migration::{prelude::*, schema::*};
use sea_orm_tstzrange::TstzRange;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, m: &SchemaManager) -> Result<(), DbErr> {
        m
        .create_table(
            sea_query::Table::create()
                .table(Reservation::Table)
                .if_not_exists()
                .col(pk_auto(Reservation::Id))
                .col(string(Reservation::ReservationId))
                .col(ColumnDef::new_with_type(Reservation::Timespan, TstzRange::column_type()))
                .to_owned()
        )
        .await?;
        Ok(())
    }

    async fn down(&self, m: &SchemaManager) -> Result<(), DbErr> {
        m.drop_table(Table::drop().table(Reservation::Table).to_owned()).await?;
        Ok(())
    }
}

#[derive(DeriveIden)]
enum Reservation {
    Table,
    Id,
    ReservationId,
    Timespan,
}