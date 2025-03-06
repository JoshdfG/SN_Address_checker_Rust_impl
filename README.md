# StarkNet Address Checker
`License: MIT`

A Rust library for validating and checking StarkNet addresses. This crate provides utilities to verify if an address is a valid StarkNet address, pad incomplete addresses, and check if an address belongs to a smart contract.

# Features
- **Validate StarkNet Addresses**: Check if an address is a valid StarkNet address.

- **Pad Incomplete Addresses**: Automatically pad addresses with zeros to match the required length.

- **Check Smart Contracts**: Verify if an address belongs to a smart contract on StarkNet.

# Installation
Add the following to your `Cargo.toml`:

```toml
[dependencies]
starknet_address_checker = "0.1.0" # Replace with the latest version check crates.io for latest version
starknet = "0.3.0" # Required for StarkNet provider functionality
```

# Usage
### Validating a StarkNet Address
The `is_valid_starknet_address` function checks if an address is a valid StarkNet address. If the address is valid, it returns the address as-is. If the address is incomplete but valid, it pads it with zeros to match the required length. If the address is invalid, it returns `false`.

```rust
use starknet_address_checker::utility::is_valid_starknet_address;

fn main() {
let address = "0x006a06ca686c6193a3420333405fe6bfb065197d670c645bdc0722a36d88982f";
let (result, prefixed) = is_valid_starknet_address(address);

    assert_eq!(result, true, "Expected a valid address");
    assert_eq!(prefixed, address, "Expected a valid address");
}
```
## Checking if an Address is a Smart Contract
The `check_address` function verifies if a given address belongs to a smart contract on StarkNet. It requires an RPC URL to interact with the StarkNet network.


```rust
use starknet_address_checker::utility::{check_address, CheckRpcUrl};

#[tokio::main]
async fn main() {
// Define the RPC URL for the StarkNet Sepolia testnet
const SEPOLIA_RPC: &str = "https://free-rpc.nethermind.io/sepolia-juno";

    // Configure the options for the address check
    let options = CheckRpcUrl {
        rpc_url: Some(SEPOLIA_RPC.to_string()),
    };

    // Define the address to check
    let address = "0x04e49f15aba463e014216cfa37049d0dd5c4bcb6c5743a60b4854c30a35cce0e";

    // Perform the address check
    match check_address(address, &options).await {
        Ok(result) => {
            if result.is_smart_contract {
                println!("The address is a smart contract.");
            } else {
                println!("The address is not a smart contract.");
            }
        }
        Err(e) => {
            eprintln!("Failed to check address: {}", e);
        }
    }
}
```

## API Reference
`is_valid_starknet_address(address: &str) -> (bool, String)`
- Input: A StarkNet address as a string.

- Output: A tuple containing:


    ∘ A boolean indicating if the address is valid.

    ∘ The validated address (padded if necessary) or the original address if already valid.

`check_address(address: &str, options: &CheckRpcUrl) -> Result<CheckAddressResult>`

- **Input**:

    ∘ A StarkNet address as a string.

    ∘ A `CheckRpcUrl` struct containing the RPC URL for the StarkNet network.


- **Output**: A `Result` containing:

    ∘ `CheckAddressResult` with a boolean field `is_smart_contract` indicating if the address is a smart contract.

An error if the request fails.

# License
This project is licensed under the [MIT](https://choosealicense.com/licenses/mit/) License. See the LICENSE file for details.
