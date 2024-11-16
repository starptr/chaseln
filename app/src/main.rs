use anyhow::Result;
use std::path::PathBuf;

fn get_cwd() -> Result<PathBuf> {
    let cwd = std::env::current_dir()?;
    Ok(cwd)
}

fn show_entries(entry_it: chaseln::entry::EntriesChain) {
    for entry in entry_it {
        println!("{}", entry);
    }
}

fn main() -> Result<()> {
    let args = chaseln::cli::get_args();
    let filename_arg = args.filename;
    let entry_it = chaseln::chaseln(&get_cwd()?, &filename_arg);
    show_entries(entry_it);
    Ok(())
}
