use cosmrs::proto::cosmos;
use cosmrs::proto::traits::TypeUrl;
use cosmrs::tx::Msg;
use cosmrs::Any;
use super::errors::LibError;

#[derive(Debug)]
pub enum ProtobufMessage {
    MsgSend(cosmrs::bank::MsgSend),
    MsgMultiSend(cosmrs::bank::MsgMultiSend),
    MsgSetWithdrawAddress(cosmrs::distribution::MsgSetWithdrawAddress),
    MsgWithdrawDelegatorReward(cosmrs::distribution::MsgWithdrawDelegatorReward),
    MsgWithdrawValidatorCommission(cosmrs::distribution::MsgWithdrawValidatorCommission),
    MsgFundCommunityPool(cosmrs::distribution::MsgFundCommunityPool),
    MsgGrantAllowance(cosmrs::feegrant::MsgGrantAllowance),
    MsgRevokeAllowance(cosmrs::feegrant::MsgRevokeAllowance),
    BasicAllowance(cosmrs::feegrant::BasicAllowance),
    PeriodicAllowance(cosmrs::feegrant::PeriodicAllowance),
    AllowedMsgAllowance(cosmrs::feegrant::AllowedMsgAllowance),
    MsgDelegate(cosmrs::staking::MsgDelegate),
    MsgUndelegate(cosmrs::staking::MsgUndelegate),
    MsgBeginRedelegate(cosmrs::staking::MsgBeginRedelegate),
    MsgData(cosmrs::abci::MsgData),
    TxMsgData(cosmrs::abci::TxMsgData),
    //BaseAccount(cosmrs::auth::BaseAccount),
    //ModuleAccount(cosmrs::auth::ModuleAccount),
    #[cfg(feature = "cosmwasm")]
    MsgStoreCode(cosmwasm::wasm::v1::MsgStoreCode),
    #[cfg(feature = "cosmwasm")]
    MsgInstantiateContract(cosmwasm::wasm::v1::MsgInstantiateContract),
    #[cfg(feature = "cosmwasm")]
    MsgExecuteContract(cosmwasm::wasm::v1::MsgExecuteContract),
    #[cfg(feature = "cosmwasm")]
    MsgMigrateContract(cosmwasm::wasm::v1::MsgMigrateContract),
    #[cfg(feature = "cosmwasm")]
    MsgUpdateAdmin(cosmwasm::wasm::v1::MsgUpdateAdmin),
    #[cfg(feature = "cosmwasm")]
    MsgClearAdmin(cosmwasm::wasm::v1::MsgClearAdmin),
}

fn parse_to_message<T: Msg>(data: &Any) -> Result<T, LibError> {
    return match T::from_any(data) {
        Ok(val) => Ok(val),
        Err(_) => Err(LibError::ParseError {}),
    };
}

impl ProtobufMessage {
    pub fn from_any(input: &Any) -> Result<Self, LibError> {
        let type_url = &input.type_url;
        match type_url.as_str() {
            cosmos::bank::v1beta1::MsgSend::TYPE_URL => {
                Ok(ProtobufMessage::MsgSend(parse_to_message(&input)?))
            }
            cosmos::bank::v1beta1::MsgMultiSend::TYPE_URL => {
                Ok(ProtobufMessage::MsgMultiSend(parse_to_message(&input)?))
            }
            cosmos::distribution::v1beta1::MsgSetWithdrawAddress::TYPE_URL => Ok(
                ProtobufMessage::MsgSetWithdrawAddress(parse_to_message(&input)?),
            ),
            cosmos::distribution::v1beta1::MsgWithdrawDelegatorReward::TYPE_URL => Ok(
                ProtobufMessage::MsgWithdrawDelegatorReward(parse_to_message(&input)?),
            ),

            cosmos::distribution::v1beta1::MsgWithdrawValidatorCommission::TYPE_URL => Ok(
                ProtobufMessage::MsgWithdrawValidatorCommission(parse_to_message(&input)?),
            ),
            cosmos::distribution::v1beta1::MsgFundCommunityPool::TYPE_URL => Ok(
                ProtobufMessage::MsgFundCommunityPool(parse_to_message(&input)?),
            ),

            cosmos::feegrant::v1beta1::MsgGrantAllowance::TYPE_URL => Ok(
                ProtobufMessage::MsgGrantAllowance(parse_to_message(&input)?),
            ),
            cosmos::feegrant::v1beta1::MsgRevokeAllowance::TYPE_URL => Ok(
                ProtobufMessage::MsgRevokeAllowance(parse_to_message(&input)?),
            ),
            cosmos::feegrant::v1beta1::BasicAllowance::TYPE_URL => {
                Ok(ProtobufMessage::BasicAllowance(parse_to_message(&input)?))
            }

            cosmos::feegrant::v1beta1::PeriodicAllowance::TYPE_URL => Ok(
                ProtobufMessage::PeriodicAllowance(parse_to_message(&input)?),
            ),
            cosmos::feegrant::v1beta1::AllowedMsgAllowance::TYPE_URL => Ok(
                ProtobufMessage::AllowedMsgAllowance(parse_to_message(&input)?),
            ),
            cosmos::staking::v1beta1::MsgDelegate::TYPE_URL => {
                Ok(ProtobufMessage::MsgDelegate(parse_to_message(&input)?))
            }
            cosmos::staking::v1beta1::MsgUndelegate::TYPE_URL => {
                Ok(ProtobufMessage::MsgUndelegate(parse_to_message(&input)?))
            }

            cosmos::staking::v1beta1::MsgBeginRedelegate::TYPE_URL => Ok(
                ProtobufMessage::MsgBeginRedelegate(parse_to_message(&input)?),
            ),
            cosmos::base::abci::v1beta1::MsgData::TYPE_URL => {
                Ok(ProtobufMessage::MsgData(parse_to_message(&input)?))
            }
            cosmos::base::abci::v1beta1::TxMsgData::TYPE_URL => {
                Ok(ProtobufMessage::TxMsgData(parse_to_message(&input)?))
            }
            /*
            cosmos::auth::v1beta1::BaseAccount::TYPE_URL => {
                Ok(ProtobufMessage::BaseAccount(parse_to_message(&input)?))
            }
            cosmos::auth::v1beta1::ModuleAccount::TYPE_URL => {
                Ok(ProtobufMessage::ModuleAccount(parse_to_message(&input)?))
            }
            */
            _ => {
                println!("Unkown msg: {}", type_url);
                return Err(LibError::UnknownTypeUrl {
                    type_url: type_url.to_owned(),
                });
            }
        }
    }
}
