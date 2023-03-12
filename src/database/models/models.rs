use super::block;
use cosmrs::tendermint::Block as TendermintBlock;
use sea_orm::ActiveValue::Set;

impl block::ActiveModel {
    pub fn from_tendermint(block: TendermintBlock, chain_id: &str) -> Self {
        block::ActiveModel {
            chain_id: Set(chain_id.to_string()),
            hash: Set(block.header.hash().to_string()),
            height: Set(block.header.height.into()),
        }
    }
}
