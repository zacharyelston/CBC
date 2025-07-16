//! Basic usage example for CBC

use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Example configuration
    let yaml_config = r#"
    providers:
      redmine:
        provider_type: redmine
        connection:
          url: https://example.com/redmine
          api_key: sample_api_key
      
      filesystem:
        provider_type: filesystem
        connection:
          path: /path/to/files
    
    discovery:
      enable_roots_discovery: true
      enable_tools_discovery: true
      enable_resources_discovery: false
    "#;
    
    // Load configuration from string
    let config = cbc::init_from_str(yaml_config)?;
    
    // Print loaded providers
    for (name, provider) in &config.providers {
        println!("Loaded provider: {}", name);
    }
    
    // Create discovery manager
    let mut discovery = cbc::DiscoveryManager::new();
    
    // Discover roots if enabled
    if config.discovery.enable_roots_discovery {
        let roots = discovery.discover_roots().await?;
        println!("Discovered {} roots", roots.len());
    }
    
    // Discover tools if enabled
    if config.discovery.enable_tools_discovery {
        let tools = discovery.discover_tools().await?;
        println!("Discovered {} tools", tools.len());
    }
    
    // Discover resources if enabled
    if config.discovery.enable_resources_discovery {
        let resources = discovery.discover_resources().await?;
        println!("Discovered {} resources", resources.len());
    }
    
    Ok(())
}
