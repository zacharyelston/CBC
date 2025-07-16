//! CBC (Capability Binding Config) CLI
//!
//! Command-line interface for working with CBC configurations.

use std::path::PathBuf;
use std::process;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize logging
    tracing_subscriber::fmt::init();
    
    println!("CBC - Capability Binding Config");
    println!("Dynamic provider discovery for AI MCP clients");
    
    // Example of loading a config file
    let args: Vec<String> = std::env::args().collect();
    
    if args.len() > 1 {
        let config_path = PathBuf::from(&args[1]);
        match cbc::init_from_file(config_path).await {
            Ok(config) => {
                println!("\nLoaded configuration with {} static providers", config.providers.len());
                println!("Discovery settings:");
                println!("  - Roots discovery: {}", config.discovery.enable_roots_discovery);
                println!("  - Tools discovery: {}", config.discovery.enable_tools_discovery);
                println!("  - Resources discovery: {}", config.discovery.enable_resources_discovery);
            },
            Err(e) => {
                eprintln!("Error loading configuration: {}", e);
                process::exit(1);
            }
        }
    } else {
        println!("\nUsage: cbc <config_file>");
    }
    
    Ok(())
}
