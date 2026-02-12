use serde::{Deserialize, Serialize};
use std::collections::BTreeSet;
use uuid::Uuid;

use crate::domain::folders::{ROOT_FOLDER, normalize_folder_path};
use crate::domain::secrets::login::LoginEntry;
use crate::domain::secrets::totp::TOTP;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct LoginEntryView {
    pub id: Uuid,
    pub folder: String,
    pub name: String,
    pub username: String,
    pub website: String,
    pub comments: String,
    pub totp: Option<TOTP>,
}

impl From<&LoginEntry> for LoginEntryView {
    fn from(entry: &LoginEntry) -> Self {
        Self {
            id: entry.id,
            folder: entry.folder.clone(),
            name: entry.name.clone(),
            username: entry.username.clone(),
            website: entry.website.clone(),
            comments: entry.comments.clone(),
            totp: entry.totp.clone(),
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct FolderBreadcrumb {
    pub name: String,
    pub path: String,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct FolderItem {
    pub name: String,
    pub path: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct FolderListing {
    pub current_folder: String,
    pub breadcrumbs: Vec<FolderBreadcrumb>,
    pub subfolders: Vec<FolderItem>,
    pub entries: Vec<LoginEntryView>,
}

pub fn build_folder_listing(
    current_folder: &str,
    mut entries: Vec<LoginEntryView>,
    subfolder_paths: BTreeSet<String>,
) -> FolderListing {
    let current_folder = normalize_folder_path(current_folder);
    let breadcrumbs = build_breadcrumbs(&current_folder);

    entries.sort_by(|a, b| {
        a.name
            .cmp(&b.name)
            .then_with(|| a.username.cmp(&b.username))
    });

    let subfolders = subfolder_paths
        .into_iter()
        .map(|path| FolderItem {
            name: path.rsplit('/').next().unwrap_or_default().to_string(),
            path,
        })
        .collect();

    FolderListing {
        current_folder,
        breadcrumbs,
        subfolders,
        entries,
    }
}

fn build_breadcrumbs(current_folder: &str) -> Vec<FolderBreadcrumb> {
    let mut breadcrumbs = vec![FolderBreadcrumb {
        name: ROOT_FOLDER.to_string(),
        path: ROOT_FOLDER.to_string(),
    }];

    if current_folder == ROOT_FOLDER {
        return breadcrumbs;
    }

    let mut path = String::new();
    for segment in current_folder.trim_start_matches('/').split('/') {
        if segment.is_empty() {
            continue;
        }

        path.push('/');
        path.push_str(segment);
        breadcrumbs.push(FolderBreadcrumb {
            name: segment.to_string(),
            path: path.clone(),
        });
    }

    breadcrumbs
}
