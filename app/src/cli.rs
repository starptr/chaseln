use clap::Parser;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    #[arg(required = true)]
    pub filename: PathBuf,
}

pub fn get_args() -> Args {
    Args::parse()
}