use std::path::{Path, PathBuf};

use walkdir::WalkDir;

use crate::errors::{Error, Result};

pub const EXCLUDED_FILES: [&str; 1] = [".DS_Store"];

#[derive(Debug)]
pub struct ScannedFolder {
    pub name: String,
    pub path: PathBuf,
    pub files: Vec<PathBuf>,
    pub children: Vec<ScannedFolder>,
}

impl ScannedFolder {
    pub fn new(name: String, path: PathBuf) -> Self {
        Self {
            name,
            path,
            files: Vec::new(),
            children: Vec::new(),
        }
    }
}

pub fn scan_directory(root: &Path) -> Result<ScannedFolder> {
    if !root.is_dir() {
        return Err(Error::InvalidPath);
    }

    let root_name = root
        .file_name()
        .map(|s| s.to_string_lossy().into_owned())
        .ok_or(Error::InvalidPath)?;

    let mut root_node = ScannedFolder {
        name: root_name,
        path: root.to_path_buf(),
        files: Vec::new(),
        children: Vec::new(),
    };

    scan_directory_tree(&mut root_node)?;

    Ok(root_node)
}

fn scan_directory_tree(node: &mut ScannedFolder) -> Result<()> {
    for entry in WalkDir::new(&node.path)
        .min_depth(1)
        .max_depth(1)
        .sort_by_file_name()
    {
        let entry = entry?;
        let path = entry.path();

        let Some(name) = path.file_name().map(|n| n.to_string_lossy()) else {
            continue;
        };

        if EXCLUDED_FILES.contains(&name.as_ref()) {
            continue;
        }

        if !entry.file_type().is_dir() {
            node.files.push(path.to_path_buf());
            continue;
        }

        let mut child_node = ScannedFolder::new(name.into_owned(), path.to_path_buf());

        scan_directory_tree(&mut child_node)?;

        node.children.push(child_node);
    }

    Ok(())
}
