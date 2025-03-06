use starknet_address_checker::{check_address, is_valid_starknet_address, CheckRpcUrl};

use tokio;
const MAINNET_RPC: &str = "https://starknet-mainnet.g.alchemy.com/starknet/version/rpc/v0_7/OEXJ9TcADB3MesS1_JuEc-UXQ_rBMsPR";
const SEPOLIA_RPC: &str = "https://free-rpc.nethermind.io/sepolia-juno";

#[tokio::test]
async fn test_is_mainet_smart_wallet() {
    let options = CheckRpcUrl {
        rpc_url: Some(MAINNET_RPC.to_string()),
    };

    let address = "0x0554b4a27e6ba1e00a01deebdf486c9c0e7bffc5074f67dfbb79bbf011162a62";
    let result = check_address(address, &options).await;

    assert!(result.is_ok(), "API request failed");
    assert_eq!(result.unwrap().is_smart_wallet, true, "Expected a smart wallet");
}

#[tokio::test]
async fn test_is_sepolia_smart_contract() {
    let options = CheckRpcUrl {
        rpc_url: Some(SEPOLIA_RPC.to_string()),
    };

    let address = "0x04e49f15aba463e014216cfa37049d0dd5c4bcb6c5743a60b4854c30a35cce0e";
    let result = check_address(address, &options).await;

    assert!(result.is_ok(), "API request failed");
    assert_eq!(
        result.unwrap().is_smart_contract,
        true,
        "Expected a smart contract"
    );
}

#[tokio::test]
async fn test_is_smart_contract_on_mainnet() {
    let options = CheckRpcUrl {
        rpc_url: Some(MAINNET_RPC.to_string()),
    };

    let address = "0x006a06ca686c6193a3420333405fe6bfb065197d670c645bdc0722a36d88982f";
    let result = check_address(address, &options).await;

    assert!(result.is_ok(), "API request failed");
    assert_eq!(
        result.unwrap().is_smart_contract,
        true,
        "Expected a smart contract"
    );
}

#[tokio::test]
async fn test_is_smart_wallet_on_testnet() {
    let options = CheckRpcUrl {
        rpc_url: Some(SEPOLIA_RPC.to_string()),
    };

    let address = "0x06eC96291A904b8B62B446FB32fC9903b5f82D73D7CA319E03ba45D50788Ec30";
    let result = check_address(address, &options).await;

    assert!(result.is_ok());
    assert_eq!(result.unwrap().is_smart_wallet, true, "Expected a smart wallet");
}

#[tokio::test]
async fn test_is_valid_starket_address() {
    let address = "0x006a06ca686c6193a3420333405fe6bfb065197d670c645bdc0722a36d8892";
    let (result,_) = is_valid_starknet_address(address);

    assert_eq!(result, false, "Expected a valid address");
}

#[tokio::test]
async fn test_is_okay(){
    let address = "0x006a06ca686c6193a3420333405fe6bfb065197d670c645bdc0722a36d88982f";
    let (result,prefixed) = is_valid_starknet_address(address);
    assert_eq!(result, true, "Expected a valid address");
    assert_eq!(prefixed, address, "Expected a valid address");
}

#[tokio::test]
async fn test_prefix_to_be_okay(){
    let address = "0x06a06ca686c6193a3420333405fe6bfb065197d670c645bdc0722a36d88982f";
    let (result,prefixed) = is_valid_starknet_address(address);
    assert_eq!(result, true, "Expected a valid address");
    let adjusted = "0x006a06ca686c6193a3420333405fe6bfb065197d670c645bdc0722a36d88982f";
    assert_eq!(prefixed, adjusted, "Expected a valid address");
}

#[tokio::test]
async fn test_uneployed_address() {
    let options = CheckRpcUrl {
        rpc_url: Some(MAINNET_RPC.to_string()),
    };

    let address = "0x01729ce1AD61551F08A1A5d4A8a0d3753de028b26b229FF021Ad8a9D3c1c29C9";
    let result = check_address(address, &options).await;

    assert_eq!(
        result
            .unwrap_err()
            .to_string()
            .contains("Contract not found"),
        true
    );
}
