//! Tests for CBC configuration handling

use std::collections::HashMap;
use cbc::{Config, Provider};

#[test]
fn test_parse_valid_config() {
    // Define a valid YAML configuration
    let yaml = r#"
    providers:
      redmine:
        provider_type: redmine
        connection:
          url: https://example.com/redmine
          api_key: sample_key
    discovery:
      enable_roots_discovery: true
      enable_tools_discovery: false
      enable_resources_discovery: false
    "#;
    
    // Parse the configuration
    let config = cbc::init_from_str(yaml).expect("Failed to parse valid config");
    
    // Validate the results
    assert_eq!(config.providers.len(), 1);
    assert!(config.providers.contains_key("redmine"));
    assert!(config.discovery.enable_roots_discovery);
    assert!(!config.discovery.enable_tools_discovery);
    assert!(!config.discovery.enable_resources_discovery);
}

#[test]
fn test_parse_missing_providers() {
    // Define a YAML configuration with no providers
    let yaml = r#"
    discovery:
      enable_roots_discovery: true
    "#;
    
    // Parse the configuration
    let config = cbc::init_from_str(yaml).expect("Failed to parse config");
    
    // Validate the results
    assert_eq!(config.providers.len(), 0);
    assert!(config.discovery.enable_roots_discovery);
}

#[test]
fn test_parse_invalid_yaml() {
    // Define an invalid YAML configuration
    let yaml = r#"
    providers: - invalid
      yaml: structure
    "#;
    
    // Attempt to parse the configuration
    let result = cbc::init_from_str(yaml);
    assert!(result.is_err());
}
