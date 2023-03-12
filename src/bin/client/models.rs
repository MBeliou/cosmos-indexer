use serde::Serialize;

#[derive(Serialize)]
pub struct GetChainStatusResponse {
    pub chain_id: String,
    pub blocks_parsed: i64,
}



#[derive(Serialize)]
pub struct GetBlockResponse {
    pub hash: String,
    pub height: i64,
    pub chain_id: String,
}