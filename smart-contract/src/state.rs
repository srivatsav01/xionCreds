use cosmwasm_std::{Storage, Addr, CustomMsg};
use cw_storage_plus::Map;
use schemars::JsonSchema;
use serde::{Serialize, Deserialize};

use crate::ContractError;

pub struct IncentiveMap{}

pub const INCENTIVE_MAP: Map<String, String> =  Map::new("incentives-map");

pub fn add_incentive(store: &mut dyn Storage, name: &str, tokent_id: &str) -> Result<(), ContractError>{

    INCENTIVE_MAP.save(store, name.to_owned(), &tokent_id.to_owned())?;
    return Ok(());
}

pub fn get_incentive(store: &dyn Storage, name: &str) -> Result<String, ContractError>{
    return Ok(INCENTIVE_MAP.load(store, name.to_owned())?);
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct TokenInfo{

    pub owner: Addr,

    /// Universal resource identifier for this NFT
    /// Should point to a JSON file that conforms to the ERC721
    /// Metadata JSON Schema
    pub token_uri: String,
}

impl CustomMsg for TokenInfo {}

pub const NFTS_MAP: Map<&str, TokenInfo> = Map::new("tokens-map");

/// Mint nfts
pub fn add_nft(sender: &Addr, store: &mut dyn Storage, token_id: &str, token_uri: String) -> Result<TokenInfo, ContractError>{

    let token = TokenInfo { owner: sender.clone(), token_uri };

    NFTS_MAP.update(store, token_id, |old| match old{
        Some(_) => Err(ContractError::Claimed {  }),
        None => Ok(token)
    }
    )
}

/// Transfer nft
pub fn transfer_nft(sender: &Addr, store: &mut dyn Storage, token_id: &str, transfer_to: &Addr) -> Result<TokenInfo, ContractError>{

    let mut token = NFTS_MAP.load(store, token_id)?;

    if sender !=  token.owner{
        return Err(ContractError::Unauthorized { })
    }

    token.owner = transfer_to.clone();
    NFTS_MAP.save(store, token_id, &token)?;

    return Ok(token);
}
