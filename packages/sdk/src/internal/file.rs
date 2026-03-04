use std::path::{Path, PathBuf};

use crate::errors::{Error, Result};

pub(crate) fn resolve_export_file_destination(
    destination: &Path,
    file_name: &str,
) -> Result<PathBuf> {
    resolve_export_path(destination, file_name, true)
}

pub(crate) fn resolve_export_folder_destination(
    destination: &Path,
    folder_name: &str,
) -> Result<PathBuf> {
    resolve_export_path(destination, folder_name, false)
}

pub(crate) fn resolve_export_root_destination(destination: &Path) -> Result<PathBuf> {
    if destination.exists() && destination.is_dir() {
        return Ok(destination.to_path_buf());
    }
    find_available_path(destination, false)
}

fn resolve_export_path(destination: &Path, name: &str, is_file: bool) -> Result<PathBuf> {
    let target = if destination.is_dir() {
        destination.join(name)
    } else {
        destination.to_path_buf()
    };

    find_available_path(&target, is_file)
}

pub(crate) fn find_available_path(path: &Path, is_file: bool) -> Result<PathBuf> {
    if !path.exists() {
        return Ok(path.to_path_buf());
    }

    let parent = path.parent().ok_or(Error::InvalidPath)?;
    let name = path
        .file_name()
        .and_then(|n| n.to_str())
        .ok_or(Error::InvalidPath)?;

    for i in 1.. {
        let candidate_name = if is_file {
            add_suffix_to_file(name, i)
        } else {
            format!("{name} ({i})")
        };

        let candidate = parent.join(candidate_name);

        if !candidate.exists() {
            return Ok(candidate);
        }

        if i == u32::MAX {
            return Err(Error::ItemAlreadyExists(name.to_string()));
        }
    }

    unreachable!()
}

fn add_suffix_to_file(file_name: &str, n: u32) -> String {
    let path = Path::new(file_name);
    let stem = path
        .file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or(file_name);
    let extension = path.extension().and_then(|e| e.to_str());

    match extension {
        Some(ext) if !ext.is_empty() => format!("{stem} ({n}).{ext}"),
        _ => format!("{stem} ({n})"),
    }
}
