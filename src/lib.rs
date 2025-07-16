//! CBC (Capability Binding Config)
//! 
//! This library provides support for dynamic provider discovery for AI MCP clients,
//! allowing standardized mapping of providers and dynamic discovery of additional
//! back-ends via MCP server features.

use std::error::Error;
use std::path::Path;

mod config;
mod discovery;
mod provider;

pub use config::Config;
pub use discovery::DiscoveryManager;
pub use provider::Provider;

/// Initialize the CBC system from a YAML configuration file
pub async fn init_from_file<P: AsRef<Path>>(
    config_path: P
) -> Result<Config, Box<dyn Error>> {
    let config = Config::from_file(config_path).await?;
    Ok(config)
}

/// Initialize the CBC system from a YAML string
pub fn init_from_str(
    yaml_str: &str
) -> Result<Config, Box<dyn Error>> {
    let config = Config::from_str(yaml_str)?;
    Ok(config)
}
