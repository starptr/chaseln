mod cli;

use std::path::{PathBuf, Path};
use std::fs;
use anyhow::Result;

fn get_cwd() -> Result<PathBuf> {
    let cwd = std::env::current_dir()?;
    Ok(cwd)
}

fn any_path_to_abs(parent: &Path, maybe_relative: &Path) -> PathBuf {
    if maybe_relative.is_absolute() {
        maybe_relative.to_path_buf()
    } else {
        parent.join(maybe_relative)
    }
}

#[derive(Debug)]
struct Entry {
    abs_location: PathBuf,
    display: String,
    kind: fs::FileType,
}
impl Entry {
    fn new(abs_location: &Path) -> Result<Self> {
        Ok(Self {
            abs_location: abs_location.to_owned(),
            display: abs_location.to_string_lossy().into_owned(),
            kind: fs::metadata(&abs_location)?.file_type(),
        })
    }
    fn new_with_display(abs_location: &Path, display: &str) -> Result<Self> {
        Ok(Self {
            abs_location: abs_location.to_owned(),
            display: display.to_owned(),
            kind: fs::metadata(&abs_location)?.file_type(),
        })
    }
}
impl std::fmt::Display for Entry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let sym = if self.kind.is_symlink() { '🔗' } else if self.kind.is_dir() { '📁' } else { '📄' };
        write!(f, "{} {}", sym, self.display)
    }
}
struct EntryIteratorContext {
    current_entry: Option<Entry>,
}
impl EntryIteratorContext {
    fn new(abs_location: &Path) -> Result<Self> {
        Ok(Self {
            current_entry: Some(Entry::new(abs_location)?),
        })
    }
}
impl Iterator for EntryIteratorContext {
    type Item = Entry;
    fn next(&mut self) -> Option<Self::Item> {
        if self.current_entry.is_none() {
            return None;
        }
        let entry = self.current_entry.as_ref().unwrap();
        if !entry.kind.is_symlink() {
            return self.current_entry.take();
        }
        let symlink_content = fs::read_link(&entry.abs_location).ok()?;
        let parent = entry.abs_location.parent()?;
        let next_abs_location = any_path_to_abs(parent, &symlink_content);
        let new_entry = Entry::new_with_display(&next_abs_location, &symlink_content.to_string_lossy()).ok()?;
        self.current_entry.replace(new_entry)
    }
}

fn main() -> Result<()> {
    let args = cli::get_args();
    let filename = args.filename;
    let abspath = any_path_to_abs(&get_cwd()?, &filename);
    let it = EntryIteratorContext::new(&abspath)?;
    let mut is_first = true;
    for entry in it {
        if !is_first {
            print!("→ ");
        }
        println!("{}", entry);
        is_first = false;
    }
    Ok(())
}
