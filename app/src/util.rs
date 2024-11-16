use std::path::{Path, PathBuf};

pub fn any_path_to_abs(parent: &Path, maybe_relative: &Path) -> PathBuf {
    if maybe_relative.is_absolute() {
        maybe_relative.to_path_buf()
    } else {
        parent.join(maybe_relative)
    }
}
