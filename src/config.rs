//! Configuration handling for CBC
//!
//! This module provides functionality for loading and parsing CBC configuration files.

use std::collections::HashMap;
use std::error::Error;
use std::path::Path;
use serde::{Deserialize, Serialize};
use tokio::fs;

use crate::provider::Provider;

/// Represents a complete CBC configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    /// Static provider configurations
    pub providers: HashMap<String, Provider>,
    
    /// Optional discovery settings
    #[serde(default)]
    pub discovery: DiscoveryConfig,
}

/// Configuration for dynamic discovery
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct DiscoveryConfig {
    /// Whether to enable discovery of filesystem workspaces
    #[serde(default)]
    pub enable_roots_discovery: bool,
    
    /// Whether to enable discovery of available tools
    #[serde(default)]
    pub enable_tools_discovery: bool,
    
    /// Whether to enable discovery of resources
    #[serde(default)]
    pub enable_resources_discovery: bool,
}

impl Config {
    /// Load configuration from a YAML file
    pub async fn from_file<P: AsRef<Path>>(path: P) -> Result<Self, Box<dyn Error>> {
        let content = fs::read_to_string(path).await?;
        Self::from_str(&content)
    }
    
    /// Parse configuration from a YAML string
    pub fn from_str(yaml_str: &str) -> Result<Self, Box<dyn Error>> {
        let config: Config = serde_yaml::from_str(yaml_str)?;
        Ok(config)
    }
}
