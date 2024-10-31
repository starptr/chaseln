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

#[derive(Debug, PartialEq)]
enum EntryKind {
    Symlink,
    Directory,
    File,
    DoesNotExist,
}

impl From<std::io::Result<fs::Metadata>> for EntryKind {
    fn from(result: std::io::Result<fs::Metadata>) -> Self {
        match result {
            Ok(metadata) => metadata.file_type().into(),
            Err(_) => EntryKind::DoesNotExist,
        }
    }
}

impl From<fs::FileType> for EntryKind {
    fn from(file_type: fs::FileType) -> Self {
        if file_type.is_symlink() {
            EntryKind::Symlink
        } else if file_type.is_dir() {
            EntryKind::Directory
        } else {
            EntryKind::File
        }
    }
}

impl std::fmt::Display for EntryKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            EntryKind::Symlink => write!(f, "🔗"),
            EntryKind::Directory => write!(f, "📁"),
            EntryKind::File => write!(f, "📄"),
            EntryKind::DoesNotExist => write!(f, "❌"),
        }
    }
}

#[derive(Debug)]
struct Entry {
    abs_location: PathBuf,
    display: String,
    kind: EntryKind,
}
impl Entry {
    fn new(abs_location: &Path) -> Self {
        Self {
            abs_location: abs_location.to_owned(),
            display: abs_location.to_string_lossy().into_owned(),
            kind: fs::symlink_metadata(&abs_location).into(),
        }
    }
    fn new_with_display(abs_location: &Path, display: &str) -> Self {
        Self {
            abs_location: abs_location.to_owned(),
            display: display.to_owned(),
            kind: fs::symlink_metadata(&abs_location).into(),
        }
    }
}
impl std::fmt::Display for Entry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {}", self.kind, self.display)
    }
}

#[derive(Debug)]
struct EntryIteratorContext {
    current_entry: Option<Entry>,
}
impl EntryIteratorContext {
    fn new(abs_location: &Path) -> Self {
        Self {
            current_entry: Some(Entry::new(abs_location)),
        }
    }
}
impl Iterator for EntryIteratorContext {
    type Item = Entry;
    fn next(&mut self) -> Option<Self::Item> {
        if self.current_entry.is_none() {
            return None;
        }
        let entry = self.current_entry.as_ref().unwrap();
        if entry.kind != EntryKind::Symlink {
            return self.current_entry.take();
        }
        let symlink_content = fs::read_link(&entry.abs_location).ok()?;
        let parent = entry.abs_location.parent()?;
        let next_abs_location = any_path_to_abs(parent, &symlink_content);
        let new_entry = Entry::new_with_display(&next_abs_location, &symlink_content.to_string_lossy());
        self.current_entry.replace(new_entry)
    }
}

fn main() -> Result<()> {
    let args = cli::get_args();
    let filename = args.filename;
    // Slightly normalize the input path
    let filename = filename.components().as_path();
    let abspath = any_path_to_abs(&get_cwd()?, &filename);
    let it = EntryIteratorContext::new(&abspath);
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
