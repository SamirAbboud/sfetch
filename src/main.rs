mod cache;
mod colors;
mod config;
mod fetch;
mod logos;
mod render;
mod utils;

use clap::Parser;
use render::render;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(name = "sfetch")]
struct Cli {
    /// Use a custom configuration file
    #[arg(long)]
    config: Option<PathBuf>,

    /// Publish the default configuration
    #[arg(long)]
    publish_config: bool,
}

fn resolve_config(config: Option<PathBuf>) -> Option<PathBuf> {
    if let Some(config) = config {
        return Some(config);
    }

    let home_config = dirs::config_dir()?.join("sfetch/config.toml");

    if home_config.exists() {
        Some(home_config)
    } else {
        None
    }
}

fn main() {
    let cli = Cli::parse();

    if cli.publish_config {
        let path = dirs::config_dir()
            .expect("Could not determine config directory")
            .join("sfetch/config.toml");

        if let Err(error) = config::publish(&path) {
            eprintln!("Failed to publish config: {error}");
            std::process::exit(1);
        }

        return;
    }

    let config = match config::load(resolve_config(cli.config)) {
        Ok(config) => config,
        Err(error) => {
            eprintln!("Failed to load config: {error}");
            std::process::exit(1);
        }
    };

    render(&config);
}
