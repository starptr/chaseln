mod cli;

use anyhow::Result;
use std::collections::HashSet;
use std::ffi::OsString;
use std::fs;
use std::path::{Path, PathBuf};

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

#[derive(Debug, Clone, Copy, PartialEq)]
enum EntryPrefix {
    /// No prefix for the first entry
    First,
    /// Most entries are a dereferenced symlink of the last entry
    Dereferenced,
    /// This entry is the same as the previous entry, without the trailing slash
    TrimmedTrailingSlash,
    /// This entry has been seen before, so the curent symlink chase is cyclic
    Seen,
}

impl std::fmt::Display for EntryPrefix {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            EntryPrefix::First => write!(f, ""),
            EntryPrefix::Dereferenced => write!(f, "→ "),
            EntryPrefix::TrimmedTrailingSlash => write!(f, "← "),
            EntryPrefix::Seen => write!(f, "🔁 "),
        }
    }
}

#[derive(Debug)]
struct Entry {
    /// The canonical way to access this file in the filesystem
    abs_location: PathBuf,
    /// How this file was referred to in the parent symlink
    /// i.e. the data in the symlink that pointed to this file
    display: String,
    kind: EntryKind,
    prefix: EntryPrefix,
}
impl Entry {
    fn new(abs_location: &Path, prefix: EntryPrefix) -> Self {
        Self {
            abs_location: abs_location.to_owned(),
            display: abs_location.to_string_lossy().into_owned(),
            kind: fs::symlink_metadata(&abs_location).into(),
            prefix,
        }
    }
    fn new_with_display(abs_location: &Path, prefix: EntryPrefix, display: &str) -> Self {
        Self {
            abs_location: abs_location.to_owned(),
            display: display.to_owned(),
            kind: fs::symlink_metadata(&abs_location).into(),
            prefix,
        }
    }
}
impl std::fmt::Display for Entry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{} {}", self.prefix, self.kind, self.display)
    }
}

#[derive(Debug)]
struct EntryIteratorContext {
    current_entry: Option<Entry>,
    /// Use OsString, because the PartialEq for PathBuf is not strict enough
    seen: HashSet<OsString>,
}
impl EntryIteratorContext {
    fn new(abs_location: &Path) -> Self {
        let initial_entry = Entry::new(abs_location, EntryPrefix::First);
        let seen = HashSet::from([initial_entry.abs_location.clone().into_os_string()]);
        Self {
            current_entry: Some(initial_entry),
            seen,
        }
    }
    fn has_seen(&self, abs_location: &Path) -> bool {
        self.seen.contains(abs_location.as_os_str())
    }
}
impl Iterator for EntryIteratorContext {
    type Item = Entry;
    fn next(&mut self) -> Option<Self::Item> {
        if self.current_entry.is_none() {
            return None;
        }
        let entry = self.current_entry.as_ref().unwrap();
        // If the entry was already seen, this entry should be the last one shown
        if entry.prefix == EntryPrefix::Seen {
            return self.current_entry.take();
        }
        match entry.kind {
            EntryKind::Symlink => {
                let symlink_content = fs::read_link(&entry.abs_location).ok()?;
                let parent = entry.abs_location.parent()?;
                let next_abs_location = any_path_to_abs(parent, &symlink_content);
                let new_entry = {
                    let mut entry = Entry::new_with_display(
                        &next_abs_location,
                        EntryPrefix::Dereferenced,
                        &symlink_content.to_string_lossy(),
                    );
                    if self.has_seen(&entry.abs_location) {
                        entry.prefix = EntryPrefix::Seen;
                    }
                    entry
                };
                self.current_entry.replace(new_entry)
            }
            EntryKind::Directory => {
                let loc_str = entry.abs_location.to_string_lossy();
                // If the directory doesn't end in a slash, the entry is not a symlink pretending to be a directory
                if !loc_str.ends_with("/") {
                    return self.current_entry.take();
                }
                // Slightly normalize the directory path
                let next_abs_location = entry.abs_location.components().as_path();
                let new_entry = {
                    let mut entry = Entry::new_with_display(
                        next_abs_location,
                        EntryPrefix::TrimmedTrailingSlash,
                        &next_abs_location.to_string_lossy(),
                    );
                    if self.has_seen(&entry.abs_location) {
                        entry.prefix = EntryPrefix::Seen;
                    }
                    entry
                };
                self.current_entry.replace(new_entry)
            }
            _ => self.current_entry.take(),
        }
    }
}

fn main() -> Result<()> {
    let args = cli::get_args();
    let filename = args.filename;
    // Slightly normalize the input path
    // TODO: after every step, slight normalization should occur
    //let filename = filename.components().as_path();
    let abspath = any_path_to_abs(&get_cwd()?, &filename);
    let it = EntryIteratorContext::new(&abspath);
    for entry in it {
        println!("{}", entry);
    }
    Ok(())
}
