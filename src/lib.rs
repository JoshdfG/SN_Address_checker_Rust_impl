use anyhow::{anyhow, Result};
use regex::Regex;
use starknet::core::types::{BlockId, BlockTag, ContractClass, FieldElement};
use starknet::core::utils::get_selector_from_name;
use starknet::providers::jsonrpc::{HttpTransport, JsonRpcClient};
use starknet::providers::Provider;
use std::sync::Arc;
use std::time::Duration;
use url::Url;
#[derive(Debug)]
pub struct CheckRpcUrl {
    pub rpc_url: Option<String>,
}

#[derive(Debug)]
pub struct CheckAddressResponse {
    pub is_valid_address: bool,
    pub is_smart_wallet: bool,
    pub is_smart_contract: bool,
    pub message: String,
}

async fn retry_operation<F, Fut, T>(operation: F, retries: u32) -> Result<T>
where
    F: Fn() -> Fut,
    Fut: std::future::Future<Output = Result<T>>,
{
    let mut attempt = 0;
    loop {
        match operation().await {
            Ok(result) => return Ok(result),
            Err(e) if attempt < retries => {
                let delay = Duration::from_secs((attempt + 1) as u64);
                tokio::time::sleep(delay).await;
                attempt += 1;
            }
            Err(e) => return Err(anyhow!("Max retries reached: {}", e)),
        }
    }
}

/// This function takes in two parameters which are the address you want to
/// check and the struct that contains the field for the rpc_url
/// It checks to know if the address is a smart-wallet.
 async fn is_smart_wallet(address: &str, options: &CheckRpcUrl) -> Result<bool> {
    let provider = get_provider(options).await?;
    let address_fe = parse_address(address)?;

    let class_hash = retry_operation(
        || async {
            provider
                .get_class_hash_at(BlockId::Tag(BlockTag::Latest), address_fe)
                .await
                .map_err(|e| anyhow!("Provider error: {}", e))
        },
        3,
    )
        .await?;

    if class_hash == FieldElement::ZERO {
        println!("Invalid or missing class hash");
        return Ok(false);
    }

    let contract_class = retry_operation(
        || async {
            provider
                .get_class(BlockId::Tag(BlockTag::Latest), class_hash)
                .await
                .map_err(|e| anyhow!("Provider error: {}", e))
        },
        3,
    )
        .await?;

    let external_selectors = match contract_class {
        ContractClass::Legacy(class) => class
            .entry_points_by_type
            .external
            .into_iter()
            .map(|ep| ep.selector)
            .collect::<Vec<_>>(),
        ContractClass::Sierra(class) => class
            .entry_points_by_type
            .external
            .into_iter()
            .map(|ep| ep.selector)
            .collect::<Vec<_>>(),
    };

    let required_selectors = vec!["__execute__", "__validate__"]
        .iter()
        .map(|name| get_selector_from_name(name).unwrap())
        .collect::<Vec<_>>();

    let has_required_selectors = required_selectors
        .iter()
        .all(|selector| external_selectors.contains(selector));

    if !has_required_selectors {
        println!("❌ No external entry points, not a wallet");
    }

    Ok(has_required_selectors)
}

/// This function takes in two parameters which are the address you want to
/// check and the struct that contains the field for the rpc_url
/// It checks to know if the address is a smart-contract.
 async fn is_smart_contract(address: &str, options: &CheckRpcUrl) -> Result<bool> {
    let is_smart_wallet = is_smart_wallet(address, options).await?;
    Ok(!is_smart_wallet)
}


/// This function takes in two parameters which are the address you want to
/// check and the struct that contains the field for the rpc_url
/// It checks to know if the address is a smart-wallet or a smart-contract.
/// It returns a message from the CheckAddressResponse struct to confirm the
/// type of address you are interacting with.
///
/// There is an address response struct where you can select the variant for the
/// response you are expecting. I'd do a demo below.
/// ```
/// pub struct CheckAddressResponse {
///    pub is_valid_address: bool,
///    pub is_smart_wallet: bool,
///    pub is_smart_contract: bool,
///    pub message: String,
/// }
///```
/// # Example: Checking if an Address is a Smart Contract
///
/// This example demonstrates how to use the `check_address` function to verify if a given address
/// is a smart contract on the Ethereum Sepolia testnet.
///
/// ```
/// use starknet_address_checker::{check_address, CheckRpcUrl};
///
/// #[tokio::main]
/// async fn main() {
///     // Define the RPC URL for the Sepolia testnet / mainnet
///     const SEPOLIA_RPC: &str = "https://free-rpc.nethermind.io/sepolia-juno";
///
///     // Configure the options for the address check
///     let options = CheckRpcUrl {
///         rpc_url: Some(SEPOLIA_RPC.to_string()),
///     };
///
///     // Define the address to check
///     let address = "0x04e49f15aba463e014216cfa37049d0dd5c4bcb6c5743a60b4854c30a35cce0e";
///
///     // Perform the address check
///     match check_address(address, &options).await {
///         Ok(result) => {
///             if result.is_smart_contract {
///                 println!("The address is a smart contract.");
///             } else {
///                 println!("The address is not a smart contract.");
///             }
///         }
///         Err(e) => {
///             eprintln!("Failed to check address: {}", e);
///         }
///     }
/// }
/// ```
///
/// ### Explanation:
/// - The `CheckRpcUrl` struct is used to configure the RPC URL for the Ethereum network.
/// - The `check_address` function is called with the address and options.
/// - The result is handled to determine if the address is a smart contract or not.
/// - Errors are gracefully handled using the `match` statement.
pub async fn check_address(address: &str, options: &CheckRpcUrl) -> Result<CheckAddressResponse> {
    let mut response = CheckAddressResponse {
        is_valid_address: false,
        is_smart_wallet: false,
        is_smart_contract: false,
        message: String::new(),
    };

    if !is_valid_starknet_address(address).0 {
        response.message = "❌ Invalid address format".to_string();
        return Ok(response);
    }

    let provider = get_provider(options).await?;
    let address_fe = parse_address(address)?;

    let class_hash = retry_operation(
        || async {
            provider
                .get_class_hash_at(BlockId::Tag(BlockTag::Latest), address_fe)
                .await
                .map_err(|e| anyhow!("Provider error: {}", e))
        },
        3,
    )
        .await?;

    if class_hash == FieldElement::ZERO {
        response.message = "❌ No contract at this address".to_string();
        return Ok(response);
    }

    let is_smart_wallet = is_smart_wallet(address, options).await?;
    if is_smart_wallet {
        response.is_valid_address = true;
        response.is_smart_wallet = true;
        response.message =
            "🛡️ Is Smart Wallet: ✅ Yes\nYou are interacting with a smart-wallet".to_string();
    } else {
        let is_smart_contract = is_smart_contract(address, options).await?;
        if is_smart_contract {
            response.is_valid_address = true;
            response.is_smart_contract = true;
            response.message = "🛡️ Is Smart Wallet: ❌ No\n🛡️ Is Smart Contract: ✅ Yes\nYou are interacting with a smart-contract".to_string();
        } else {
            response.message = "🛡️ Is Smart Wallet: ❌ No\n🛡️ Is Smart Contract: ❌ No\nThis address is not a smart wallet or smart contract".to_string();
        }
    }

    Ok(response)
}


/// This function takes in an address and if the address is a valid starknet
/// address it returns it as it is else it pads it with the required zero to
/// make it a complete address length,This works with starknet addresses that
/// are one bit shorter than the required length, if it's a valid address it returns
/// it as it is, else it modifies it and returns the modified address,
/// However if the address is not a valid
/// starknet address you get false as your response.
/// # Examples
///```
/// use starknet_address_checker::is_valid_starknet_address;
/// let address = "0x006a06ca686c6193a3420333405fe6bfb065197d670c645bdc0722a36d88982f";
/// let (result,prefixed) = is_valid_starknet_address(address);
/// assert_eq!(result, true, "Expected a valid address");
/// assert_eq!(prefixed, address, "Expected a valid address");
/// ```
pub fn is_valid_starknet_address(address: &str) -> (bool, String) {
    let re = Regex::new(r"^0x[0-9a-fA-F]{64}$").unwrap();

    if re.is_match(address) {
        return (true, address.to_string());
    }

    if address.len() == 65 && address.starts_with("0x") {
        let without_prefix = &address[2..];
        if without_prefix.chars().all(|c| c.is_ascii_hexdigit()) {
            let fixed_address = format!("0x0{}", without_prefix);
            if re.is_match(&fixed_address) {
                return (true, fixed_address);
            }
        }
    }

    (false, address.to_string())
}

async fn get_provider(options: &CheckRpcUrl) -> Result<Arc<JsonRpcClient<HttpTransport>>> {
    let rpc_url = options
        .rpc_url
        .as_ref()
        .ok_or_else(|| anyhow!("Missing node URL"))?;

    let url = Url::parse(rpc_url)?;
    Ok(Arc::new(JsonRpcClient::new(HttpTransport::new(url))))
}

fn parse_address(address: &str) -> Result<FieldElement> {
    FieldElement::from_hex_be(address).map_err(|_| anyhow!("Invalid address format"))
}
