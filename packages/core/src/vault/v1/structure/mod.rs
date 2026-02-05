pub mod header;
pub mod index;

pub use header::*;
pub use index::*;

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct Vault {
    pub header: VaultHeader,
    pub files_index: SectionIndex<IndexEntry>,
    pub notes_index: SectionIndex<NoteEntry>,
    pub logs_index: SectionIndex<LogEntry>,
    pub folder_index: SectionIndex<FolderEntry>,
    pub decoy_vault: Option<Box<Vault>>,
}
