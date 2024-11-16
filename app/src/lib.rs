pub mod entry;
pub mod cli;
mod util;

use std::path::Path;

pub fn chaseln(cwd: &Path, filename_arg: &Path) -> entry::EntriesChain {
    let abspath = util::any_path_to_abs(cwd, filename_arg);
    entry::EntriesChain::new(&abspath)
}