use dotenvy::dotenv;
use std::env;

use actix_web::{error, get, web, App, HttpServer, Responder, Result};
use cosmos_indexer::database::{
    models::{prelude::*, *},
    utils::setup_database,
};
use sea_orm::{ColumnTrait, Condition, DatabaseConnection, EntityTrait, QueryFilter};

mod models;
use models::*;

#[get("/status")]
async fn status() -> Result<impl Responder> {
    let fetchhub = GetChainStatusResponse {
        chain_id: "fetchhub-4".to_string(),
        blocks_parsed: 0,
    };
    Ok(web::Json(vec![fetchhub]))
}

#[get("/chain/{id}/block/{height}")]
async fn height(
    db: web::Data<DatabaseConnection>,
    path: web::Path<(String, u32)>,
) -> Result<impl Responder> {
    let (id, height) = path.into_inner();

    let block = Block::find()
        .filter(
            Condition::all()
                .add(block::Column::Height.eq(height))
                .add(block::Column::ChainId.eq(id)),
        )
        .one(db.get_ref())
        .await
        .unwrap();

    match block {
        Some(b) => {
            let def = GetBlockResponse {
                chain_id: b.chain_id,
                hash: b.hash,
                height: b.height,
            };
            return Ok(web::Json(def));
        }
        None => return Err(error::ErrorNotFound("not found".to_string())),
    }
}

#[actix_web::main] // or #[tokio::main]
async fn main() -> std::io::Result<()> {
    dotenv().expect(".env file not found");

    let database_name =
        env::var("DATABASE_NAME").expect(".env file should contain a database name");
        
    let db = setup_database(format!("sqlite://{}?mode=rwc", database_name).as_str())
        .await
        .unwrap();

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(db.clone()))
            .service(status)
            .service(height)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
