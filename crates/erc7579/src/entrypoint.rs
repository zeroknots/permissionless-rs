use ethers::middleware::Middleware;
use ethers::{
    prelude::*,
    providers::{Http, Provider},
};
use std::convert::TryFrom;
use std::sync::Arc;

// This macro will be expanded at compile time
abigen!(
    EntryPoint,
    "./crates/erc7579/abi/entrypoint.json",
    event_derives(serde::Deserialize, serde::Serialize)
);

pub async fn get_entrypoint(
    rpc_url: &str,
    contract_address: &str,
) -> Result<EntryPoint<Provider<Http>>, Box<dyn std::error::Error>> {
    // Create the provider
    let provider = Provider::<Http>::try_from(rpc_url)?;

    // Create the client
    let client = Arc::new(provider);

    // Parse the contract address
    let address: Address = contract_address.parse()?;

    // Create the contract instance
    let contract = EntryPoint::new(address, client);

    Ok(contract)
}
