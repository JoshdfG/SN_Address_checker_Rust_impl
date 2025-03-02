use stark_address_checker::{check_address, is_valid_starknet_address, CheckRpcUrl};

use tokio;
const MAINNET_RPC: &str = "https://starknet-mainnet.g.alchemy.com/starknet/version/rpc/v0_7/OEXJ9TcADB3MesS1_JuEc-UXQ_rBMsPR";
const SEPOLIA_RPC: &str = "https://free-rpc.nethermind.io/sepolia-juno";

#[tokio::test]
async fn test_is_smart_wallet() {
    let options = CheckRpcUrl {
        node_url: Some(MAINNET_RPC.to_string()),
    };

    let address = "0x0554b4a27e6ba1e00a01deebdf486c9c0e7bffc5074f67dfbb79bbf011162a62";
    let result = check_address(address, &options).await;

    assert!(result.is_ok(), "API request failed");
    assert_eq!(result.unwrap().is_wallet, true, "Expected a smart wallet");
}

#[tokio::test]
async fn test_is_smart_contract() {
    let options = CheckRpcUrl {
        node_url: Some(SEPOLIA_RPC.to_string()),
    };

    let address = "0x04e49f15aba463e014216cfa37049d0dd5c4bcb6c5743a60b4854c30a35cce0e";
    let result = check_address(address, &options).await;

    assert!(result.is_ok(), "API request failed");
    assert_eq!(
        result.unwrap().is_contract,
        true,
        "Expected a smart contract"
    );
}

#[tokio::test]
async fn test_is_smart_contract_on_mainnet() {
    let options = CheckRpcUrl {
        node_url: Some(MAINNET_RPC.to_string()),
    };

    let address = "0x006a06ca686c6193a3420333405fe6bfb065197d670c645bdc0722a36d88982f";
    let result = check_address(address, &options).await;

    assert!(result.is_ok(), "API request failed");
    assert_eq!(
        result.unwrap().is_contract,
        true,
        "Expected a smart contract"
    );
}

#[tokio::test]
async fn test_is_smart_wallet_on_testnet() {
    let options = CheckRpcUrl {
        node_url: Some(SEPOLIA_RPC.to_string()),
    };

    let address = "0x06eC96291A904b8B62B446FB32fC9903b5f82D73D7CA319E03ba45D50788Ec30";
    let result = check_address(address, &options).await;

    assert!(result.is_ok());
    assert_eq!(result.unwrap().is_wallet, true, "Expected a smart wallet");
}

#[tokio::test]
async fn test_is_valid_starket_address() {
    let address = "0x006a06ca686c6193a3420333405fe6bfb065197d670c645bdc0722a36d88982";
    let result = is_valid_starknet_address(address);

    assert!(result == false, "Expected a valid address");
}

// #[tokio::test]
// async fn test_uneployed_address() {
//     let options = CheckRpcUrl {
//         node_url: Some(MAINNET_RPC.to_string()),
//     };

//     let address = "0x01729ce1AD61551F08A1A5d4A8a0d3753de028b26b229FF021Ad8a9D3c1c29C9";
//     let result = check_address(address, &options).await;

//     assert_eq!(result.unwrap(), false, "Expected an undeployed address");
// }
// #[tokio::test]
// async fn test_rpc_failure_handling() {
//     let options = CheckRpcUrl {
//         node_url: Some("https://invalid-rpc.example.com".to_string()),
//         ..Default::default()
//     };

//     // Test with an invalid RPC URL
//     let address = "0x0554b4a27e6ba1e00a01deebdf486c9c0e7bffc5074f67dfbb79bbf011162a62";
//     let result = is_smart_wallet(address, &options).await;

//     // Assert that the function fails with an error
//     assert!(result.is_err(), "Expected an error for an invalid RPC URL");
// }

// #[tokio::test]
// async fn test_sepolia_network() {
//     let options = CheckRpcUrl {
//         node_url: Some(SEPOLIA_RPC.to_string()),
//         ..Default::default()
//     };

//     // Test with a Sepolia address
//     let address = "0x057214e10bbdf9f30dc19e7fe33351ad0f33829647725f5f737aa2ffbeaf5348";
//     let result = check_address(address, &options).await;

//     // Assert that the function succeeds
//     assert!(result.is_ok(), "check_address should work on Sepolia");
// }
