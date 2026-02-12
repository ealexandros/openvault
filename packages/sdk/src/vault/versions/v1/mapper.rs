use crate::domain::Vault;
use crate::domain::models::folder::Folder;
use crate::vault::versions::v1::schema::metadata::VaultMeta;
use chrono::Utc;
use uuid::Uuid;

pub struct Mapper;

impl Mapper {
    pub fn to_domain(_meta: VaultMeta) -> Vault {
        // @todo-now: Implement tree reconstruction from flat lists of files and folders
        // We need to build the folder hierarchy starting from root (id=0 usually)

        let root = Folder {
            id: Uuid::new_v4(), // Temporary, should map from ID 0
            name: "/".to_string(),
            parent_id: None,
            children: vec![],
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };

        Vault {
            id: Uuid::new_v4(),
            name: "Vault".to_string(),
            path: Default::default(), // @todo: Needs path passed in or stored in VaultMeta?
            notes: vec![],
            secrets: vec![],
            created_at: Utc::now(),
            updated_at: Utc::now(),
            root,
            state: Default::default(),
        }
    }

    pub fn to_schema(_vault: Vault) -> VaultMeta {
        // @todo-now: Implement flattening of domain tree to schema lists
        todo!()
    }
}
