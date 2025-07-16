//! Provider definitions and implementations
//!
//! This module contains the Provider trait and implementations for various provider types.

use serde::{Deserialize, Serialize};

/// Represents a capability provider
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Provider {
    /// Type of provider
    pub provider_type: ProviderType,
    
    /// Connection parameters
    pub connection: ConnectionParams,
    
    /// Optional provider-specific settings
    #[serde(default)]
    pub settings: serde_yaml::Value,
}

/// Types of supported providers
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum ProviderType {
    /// Redmine project management
    Redmine,
    
    /// Jira project management
    Jira,
    
    /// Filesystem provider
    Filesystem,
    
    /// Generic MCP server
    MCP,
    
    /// Custom provider type
    Custom(String),
}

/// Connection parameters for providers
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ConnectionParams {
    /// HTTP-based connection
    Http {
        url: String,
        api_key: Option<String>,
        username: Option<String>,
        password: Option<String>,
    },
    
    /// Filesystem-based connection
    Filesystem {
        path: String,
    },
    
    /// Custom connection parameters
    Custom(serde_yaml::Value),
}
