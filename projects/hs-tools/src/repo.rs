use std::path::{Path, PathBuf};

/// Walk upward from `start` until a Hankel Searcher repository root is found.
pub fn find_repo_root(start: &Path) -> Option<PathBuf> {
    let mut current = start.canonicalize().ok()?;
    while current.parent().is_some() {
        for target in hs_checkpoint::CHECKPOINT_TARGETS {
            let marker = current.join("projects").join("targets").join(target).join("checkpoint.json");
            if marker.is_file() {
                return Some(current);
            }
        }
        current = current.parent()?.to_path_buf();
    }
    None
}

/// Ensure `checkpoint` resolves inside `repo_root`.
pub fn ensure_checkpoint_in_repo(repo_root: &Path, checkpoint: &Path) -> Result<PathBuf, String> {
    let repo = repo_root.canonicalize().map_err(|e| e.to_string())?;
    let resolved = if checkpoint.is_absolute() {
        checkpoint.to_path_buf()
    } else {
        repo.join(checkpoint)
    };
    let resolved = resolved.canonicalize().map_err(|e| e.to_string())?;
    if !resolved.starts_with(&repo) {
        return Err("checkpoint path escapes repository root".into());
    }
    Ok(resolved)
}

pub fn resolve_checkpoint(repo_root: &Path, target: &str, explicit: Option<PathBuf>) -> Result<PathBuf, String> {
    let path = match explicit {
        Some(path) => ensure_checkpoint_in_repo(repo_root, &path)?,
        None => {
            let default = hs_checkpoint::default_checkpoint_path(target).map_err(|e| e.to_string())?;
            ensure_checkpoint_in_repo(repo_root, &default)?
        }
    };
    Ok(path)
}
