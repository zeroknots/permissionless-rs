use ethers::providers::{Http, Provider};
use std::convert::TryFrom;
use url::Url;

pub fn create_provider(rpc_url: &str) -> Result<Provider<Http>, Box<dyn std::error::Error>> {
    // Parse the URL to ensure it's valid
    let url = Url::parse(rpc_url)?;

    // Create the provider
    let provider = Provider::<Http>::try_from(url.as_str())?;

    Ok(provider)
}
