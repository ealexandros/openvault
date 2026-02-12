pub const ROOT_FOLDER: &str = "/";

pub fn normalize_folder_path(path: &str) -> String {
    let trimmed = path.trim();

    if trimmed.is_empty() || trimmed == ROOT_FOLDER {
        return ROOT_FOLDER.to_string();
    }

    let parts: Vec<&str> = trimmed
        .split('/')
        .filter(|segment| !segment.trim().is_empty())
        .collect();

    if parts.is_empty() {
        return ROOT_FOLDER.to_string();
    }

    format!("/{}", parts.join("/"))
}

pub fn immediate_child_folder(parent: &str, candidate: &str) -> Option<String> {
    let parent = normalize_folder_path(parent);
    let candidate = normalize_folder_path(candidate);

    if parent == candidate {
        return None;
    }

    let prefix = if parent == ROOT_FOLDER {
        ROOT_FOLDER.to_string()
    } else {
        format!("{}/", parent)
    };

    if !candidate.starts_with(&prefix) {
        return None;
    }

    let remainder = candidate.strip_prefix(&prefix)?;
    if remainder.is_empty() {
        return None;
    }

    let next_segment = remainder.split('/').next()?;
    if next_segment.is_empty() {
        return None;
    }

    if parent == ROOT_FOLDER {
        Some(format!("/{}", next_segment))
    } else {
        Some(format!("{}/{}", parent, next_segment))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_input() {
        assert_eq!(normalize_folder_path(""), "/");
        assert_eq!(normalize_folder_path("   "), "/");
    }

    #[test]
    fn test_root_input() {
        assert_eq!(normalize_folder_path("/"), "/");
        assert_eq!(normalize_folder_path(" / "), "/");
    }

    #[test]
    fn test_normal_paths() {
        assert_eq!(normalize_folder_path("foo/bar"), "/foo/bar");
        assert_eq!(normalize_folder_path("/foo/bar/"), "/foo/bar");
        assert_eq!(normalize_folder_path("///foo//bar//"), "/foo/bar");
    }

    #[test]
    fn test_single_segment() {
        assert_eq!(normalize_folder_path("foo"), "/foo");
        assert_eq!(normalize_folder_path("/foo"), "/foo");
    }

    #[test]
    fn test_immediate_child_folder() {
        assert_eq!(
            immediate_child_folder("/", "/work/team"),
            Some("/work".to_string())
        );
        assert_eq!(
            immediate_child_folder("/work", "/work/team"),
            Some("/work/team".to_string())
        );
        assert_eq!(immediate_child_folder("/work", "/personal"), None);
        assert_eq!(immediate_child_folder("/work", "/work"), None);
    }
}
