use cosmwasm_schema::{cw_serde, QueryResponses};

#[cw_serde]
pub struct InstantiateMsg {}

#[cw_serde]
pub enum ExecuteMsg {
    TransferNft { recipient: String, token_id: String },
    MintNft { token_id: String, token_uri: String },
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(String)]
    Incentive { incentive: String },
    #[returns(String)]
    AllNFTs {},
}

#[cw_serde]
pub enum Incentive{
    IITH,
    Twitter100K
}
