use cosmos_indexer::database::utils::setup_database;
use cosmos_indexer::fetch_block;
use dotenvy::dotenv;
use std::env;

use clap::Parser;
use cosmos_indexer::chains::chains::get_chain;
use cosmos_indexer::database::models::{prelude::*, *};
use cosmrs::rpc::HttpClient;
use sea_orm::EntityTrait;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(author, version, about, name="An indexer for cosmos blockchains", long_about = None)]
struct Args {
    /// id of the chain we want to index, e.g. fetchhub-4
    #[arg(short, long)]
    chain_id: String,
}

#[tokio::main]
async fn main() {
    dotenv().expect(".env file not found");
    let database_name =
        env::var("DATABASE_NAME").expect(".env file should contain a database name");

    let args = Args::parse();

    let chain_config = get_chain(args.chain_id.clone())
        .expect(format!("Unknown chain ID {}", args.chain_id.clone()).as_str());

    let db = setup_database(format!("sqlite://{}?mode=rwc", database_name).as_str())
        .await
        .unwrap();

    let client = HttpClient::new(chain_config.rpcs[0]).unwrap();

    let height: u32 = 10034145;
    let tendermint_block = fetch_block(&client, height).await;

    let db_block = block::ActiveModel::from_tendermint(tendermint_block, chain_config.id);
    let res = Block::insert(db_block).exec(&db).await.unwrap();

    let found_block = Block::find_by_id(res.last_insert_id)
        .one(&db)
        .await
        .unwrap();
    println!("found block in DB: {:?}", found_block);
}
