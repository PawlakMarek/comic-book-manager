use std::env;

use config::{Config, ConfigError, File};
use dotenv::dotenv;

mod business_logic;
mod cli;
mod data;
mod utils;
mod web;

fn main() -> Result<(), ConfigError> {
    // Load environment variables from .env file
    dotenv().ok();

    // Initialize the configuration builder
    let settings = Config::builder()
        .add_source(File::with_name("config/default"))
        .build()?;

    // Retrieve configuration values
    let database_url: String = settings.get("database.url")?;
    let marvel_base_url: String = settings.get("api.marvel_base_url")?;
    let comicvine_base_url: String = settings.get("api.comicvine_base_url")?;

    // Retrieve API keys from environment variables
    let marvel_api_public_key =
        env::var("MARVEL_API_PUBLIC_KEY").expect("MARVEL_API_PUBLIC_KEY not set");
    let marvel_api_private_key =
        env::var("MARVEL_API_PRIVATE_KEY").expect("MARVEL_API_PRIVATE_KEY not set");
    let comicvine_api_key = env::var("COMICVINE_API_KEY").expect("COMICVINE_API_KEY not set");

    println!("Welcome to the Comic Book Manager!");

    // Pass configurations to the CLI if needed
    cli::run();

    Ok(())
}
