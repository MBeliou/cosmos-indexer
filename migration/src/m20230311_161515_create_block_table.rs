use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        //todo!();

        manager
            .create_table(
                Table::create()
                    .table(Block::Table)
                    .if_not_exists()
                    /*.col(
                        ColumnDef::new(Block::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    ) */
                    .col(ColumnDef::new(Block::Hash).string().not_null().primary_key())
                    .col(ColumnDef::new(Block::Height).integer().not_null())
                    .col(ColumnDef::new(Block::ChainID).string().not_null())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        // todo!();

        manager
            .drop_table(Table::drop().table(Block::Table).to_owned())
            .await
    }
}

/// Learn more at https://docs.rs/sea-query#iden
#[derive(Iden)]
enum Block {
    Table,
    //Id,
    Hash,
    Height,
    // #[iden="chain_id"]
    ChainID,
}
