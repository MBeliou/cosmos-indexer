use sea_orm::{DatabaseConnection, DbErr, Database};


pub async fn setup_database(url: &str) -> Result<DatabaseConnection, DbErr> {
    let db = Database::connect(url).await?;
    // TODO: ideally handle all db types
    Ok(db)
}