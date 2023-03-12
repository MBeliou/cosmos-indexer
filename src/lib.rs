pub mod chains;
pub mod database;
pub mod parser;

use crate::chains::chains::FETCHHUB;
use cosmrs::rpc::query::{EventType, Query};
use cosmrs::rpc::{self, error::Error as TendermintRPCError, HttpClient};
use cosmrs::rpc::{Client, Response};
use cosmrs::tendermint::Block as TendermintBlock;
use cosmrs::Tx;
// use cosmrs::tendermint::Error;

use cosmrs::rpc::endpoint::block::Response as BlockResponse;

// use cosmrs::tendermint::error::Error;

use crate::parser::parser::ProtobufMessage;

pub async fn myfn() {
    let client = rpc::HttpClient::new(FETCHHUB.rpcs[0]).unwrap();

    let abci_info = client.abci_info().await.unwrap();

    let latest_height: u32 = 10034144;
    //let latest_height = abci_info.last_block_height;
    // println!("Got ABCI info: {:?}", abci_info);
    println!("{:?}", &latest_height);
    let latest_block = client.block(latest_height).await.unwrap();

    for tx in latest_block.block.data.iter() {
        let tx = Tx::from_bytes(tx).unwrap();
        for msg in tx.body.messages {
            match ProtobufMessage::from_any(&msg) {
                Ok(curr_msg) => println!("curr_msg: {:?}", curr_msg),
                Err(e) => println!("err: {:?}", e),
            }
        }
    }
}

pub async fn fetch_block(client: &HttpClient, height: u32) -> TendermintBlock {
    let BlockResponse { block, block_id } = client.block(height).await.unwrap();

    for tx in block.data.iter() {
        let tx = Tx::from_bytes(tx).unwrap();
        //tx.auth_info.fee
        let mut message_accumulator = vec![];

        for msg in tx.body.messages {
            if let Ok(decoded_message) = ProtobufMessage::from_any(&msg) {
                message_accumulator.push(decoded_message);
            };
        }
    }
    block
}

pub async fn fetch_transactions_in_block(
    client: &HttpClient,
    height: u32,
) -> Result<(), TendermintRPCError> {
    let query = Query::from(EventType::Tx).and_eq("tx.height", height);
    let search_result = client
        .tx_search(query, false, 1, 100, rpc::Order::Descending)
        .await?;
    // .unwrap();

    Tx::from_bytes(&search_result.txs[0].tx_result.data);
    Ok(())
}
