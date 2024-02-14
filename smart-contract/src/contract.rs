#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{Binary, Deps, DepsMut, Env, MessageInfo, Response, to_json_binary, Empty};

use crate::error::ContractError;
use crate::msg::{ExecuteMsg, QueryMsg};
use crate::state::{add_incentive, get_incentive, TokenInfo, transfer_nft, add_nft};

/*
// version info for migration info
const CONTRACT_NAME: &str = "crates.io:cwnft";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");
*/

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    _msg: Empty,
) -> Result<Response, ContractError> {

    let store = deps.storage;

    add_incentive(store, "IITH", "token1")?;
    add_incentive(store, "Twitter100K", "token2")?;

    return Ok(Response::new());
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response<TokenInfo>, ContractError> {

     match  msg{

        ExecuteMsg::TransferNft { recipient, token_id } => {

            let transfer_to = deps.api.addr_validate(&recipient)?;
            transfer_nft(&info.sender, deps.storage, &token_id, &transfer_to)?;

            Ok(Response::new()
                .add_attribute("action", "transfer_nft")
                .add_attribute("sender", info.sender)
                .add_attribute("recipient", transfer_to)
                .add_attribute("token_id", token_id))
        }

        //TODO: add restrection on adding nft
        ExecuteMsg::MintNft {  token_id, token_uri } => {

            add_nft(&info.sender, deps.storage, &token_id, token_uri)?;

            Ok(Response::new()
                .add_attribute("action", "mint")
                .add_attribute("minter", &info.sender)
                .add_attribute("owner", &info.sender)
                .add_attribute("token_id", token_id))
        }
    }

}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> Result<Binary, ContractError> {
    let store = deps.storage;
    match msg {
        QueryMsg::Incentive { incentive } => {
            let token = get_incentive(store, &incentive)?;
            return Ok(to_json_binary(&token)?);
        }
        QueryMsg::AllNFTs {  } => {
            return Ok(to_json_binary("")?);
        }
    }
}

#[cfg(test)]
mod tests {

    use cosmwasm_std::{Empty, testing::{mock_dependencies, mock_info, mock_env}, from_json, Addr};
    use super::*;


    #[test]
    fn incentive_query(){

        let mut deps = mock_dependencies();
        let info = mock_info("creator", &[]);
        instantiate(deps.as_mut(), mock_env(), info, Empty {}).unwrap();

        let res = query(deps.as_ref(), mock_env(), QueryMsg::Incentive { incentive: "IITH".to_owned() }).unwrap();
        let value: String = from_json(&res).unwrap();
        assert_eq!(value, "token1");

        let res = query(deps.as_ref(), mock_env(), QueryMsg::Incentive { incentive: "Twitter100K".to_owned() }).unwrap();
        let value: String = from_json(&res).unwrap();
        assert_eq!(value, "token2");
    }

    #[test]
    fn transfer_nft(){
        let mut deps = mock_dependencies();

        let info = mock_info("creator", &[]);
        instantiate(deps.as_mut(), mock_env(), info, Empty {}).unwrap();

        let info = mock_info("minter", &[]);
        execute(deps.as_mut(), mock_env(), info,
            ExecuteMsg::MintNft {
                token_id: "token1".to_owned(), token_uri: "token_uri_1".to_owned()
            }).unwrap();

        let info = mock_info("minter", &[]);
        let token = execute(deps.as_mut(), mock_env(), info.clone(),
            ExecuteMsg::TransferNft { recipient: "getter".to_owned(), token_id: "token1".to_owned()
            }).unwrap();

        let correct_token: Response<TokenInfo> = Response::new()
                .add_attribute("action", "transfer_nft")
                .add_attribute("sender", info.sender)
                .add_attribute("recipient", "getter")
                .add_attribute("token_id", "token1".to_owned());

        assert_eq!(token, correct_token);
    }
}
