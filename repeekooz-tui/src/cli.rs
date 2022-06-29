use clap::Parser;

use crate::config::Config;
use anyhow::Result;

#[derive(Parser, Debug)]
#[clap(
    author,
    version,
    about,
    long_about = "A TUI toolkit to view HelloGitHub"
)]
pub struct Args {
    #[clap(short, long, help = "配置文件路径")]
    pub path: Option<String>,

    #[clap(short, long, help = "是否显示帮助")]
    pub show_help: bool,

    #[clap(long, help = "显示内置样式列表")]
    pub show_themes: bool,
}

pub fn parse_args() -> Result<Config> {
    let args = Args::parse();
    Ok(Config::from(args))
}
