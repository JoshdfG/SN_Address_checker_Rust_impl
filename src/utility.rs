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
    pub is_valid: bool,
    pub is_wallet: bool,
    pub is_contract: bool,
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

pub async fn is_smart_wallet(address: &str, options: &CheckRpcUrl) -> Result<bool> {
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

pub async fn is_smart_contract(address: &str, options: &CheckRpcUrl) -> Result<bool> {
    let is_wallet = is_smart_wallet(address, options).await?;
    Ok(!is_wallet)
}

pub async fn check_address(address: &str, options: &CheckRpcUrl) -> Result<CheckAddressResponse> {
    let mut response = CheckAddressResponse {
        is_valid: false,
        is_wallet: false,
        is_contract: false,
        message: String::new(),
    };

    if !is_valid_starknet_address(address) {
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

    let is_wallet = is_smart_wallet(address, options).await?;
    if is_wallet {
        response.is_valid = true;
        response.is_wallet = true;
        response.message =
            "🛡️ Is Smart Wallet: ✅ Yes\nYou are interacting with a smart-wallet".to_string();
    } else {
        let is_contract = is_smart_contract(address, options).await?;
        if is_contract {
            response.is_valid = true;
            response.is_contract = true;
            response.message = "🛡️ Is Smart Wallet: ❌ No\n🛡️ Is Smart Contract: ✅ Yes\nYou are interacting with a smart-contract".to_string();
        } else {
            response.message = "🛡️ Is Smart Wallet: ❌ No\n🛡️ Is Smart Contract: ❌ No\nThis address is not a smart wallet or smart contract".to_string();
        }
    }

    Ok(response)
}
pub fn is_valid_starknet_address(address: &str) -> bool {
    let re = Regex::new(r"^0x0[0-9a-fA-F]{63}$").unwrap();
    if !re.is_match(address) {
        println!("❌ Invalid format - Must be 66 characters starting with 0x0");
        return false;
    }

    match FieldElement::from_hex_be(address) {
        Ok(fe) if fe == FieldElement::ZERO => {
            println!("❌ Invalid - Zero address (0x000...000)");
            false
        }
        Ok(_) => {
            println!("✅ Valid Starknet address");
            true
        }
        Err(_) => {
            println!("❌ Invalid hex value - Contains non-hex characters");
            false
        }
    }
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
