#![warn(missing_docs)]

//! This is main entry point for this project

use std::panic;

use anyhow::Result;
use app::start;
use cli::parse_args;

mod app;
mod app_global;
mod cli;
mod config;
mod draw;
mod events;
mod theme;
mod utils;
mod widget;

fn main() -> Result<()> {
    better_panic::install();
    let config = parse_args()?;
    // println!("{:?}", config);
    setup_panic_hook();
    start(&config)?;

    Ok(())
}

fn setup_panic_hook() {
    panic::set_hook(Box::new(|panic_info| {
        // cleanup_terminal();
        better_panic::Settings::auto().create_panic_handler()(panic_info);
    }));
}
