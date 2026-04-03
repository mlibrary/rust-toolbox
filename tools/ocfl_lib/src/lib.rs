// OCFL v1.1 core library API skeleton
// Functional, minimal 'unsafe', anyhow for error handling

use anyhow::{Context, Result};
use std::fs;
use std::fs::create_dir_all;
use std::path::Path;

/// OCFL repository operations
pub trait OcflRepo {
    /// Initialize a new OCFL repository at the given path
    fn init_repo<P: AsRef<Path>>(&self, path: P) -> Result<()>;
    /// Add an object to the repository
    fn add_object<P: AsRef<Path>>(&self, object_id: &str, src_path: P) -> Result<()>;
    /// Get an object from the repository
    fn get_object<P: AsRef<Path>>(&self, object_id: &str, dest_path: P) -> Result<()>;
    /// List all objects in the repository
    fn list_objects(&self) -> Result<Vec<String>>;
}

/// Default OCFL repository implementation
pub struct OcflRepoImpl {
    root: String,
}

impl OcflRepoImpl {
    pub fn new<P: AsRef<Path>>(root: P) -> Self {
        Self { root: root.as_ref().to_string_lossy().to_string() }
    }
}

impl OcflRepo for OcflRepoImpl {
    fn init_repo<P: AsRef<Path>>(&self, path: P) -> Result<()> {
        let repo_path = path.as_ref();
        create_dir_all(repo_path).with_context(|| format!("Failed to create repo directory: {}", repo_path.display()))?;
        let spec_file = repo_path.join("0=ocfl_1.1");
        if !spec_file.exists() {
            fs::write(&spec_file, "OCFL Object Root\nhttps://ocfl.io/1.1/spec/\n").with_context(|| format!("Failed to write OCFL spec file: {}", spec_file.display()))?;
        }
        Ok(())
    }
    fn add_object<P: AsRef<Path>>(&self, object_id: &str, src_path: P) -> Result<()> {
        let repo_path = Path::new(&self.root);
        if !repo_path.exists() {
            anyhow::bail!("OCFL repository does not exist at {}", repo_path.display());
        }
        let spec_file = repo_path.join("0=ocfl_1.1");
        if !spec_file.exists() {
            anyhow::bail!("Not a valid OCFL v1.1 repository: missing {}", spec_file.display());
        }
        let object_root = repo_path.join(object_id);
        if object_root.exists() {
            anyhow::bail!("OCFL object {} already exists", object_id);
        }
        create_dir_all(&object_root).with_context(|| format!("Failed to create object root: {}", object_root.display()))?;
        let object_spec = object_root.join("0=ocfl_object_1.1");
        fs::write(&object_spec, "OCFL Object Root\nhttps://ocfl.io/1.1/spec/\n").with_context(|| format!("Failed to write object spec file: {}", object_spec.display()))?;
        let content_dir = object_root.join("content");
        create_dir_all(&content_dir).with_context(|| format!("Failed to create content dir: {}", content_dir.display()))?;
        let src = src_path.as_ref();
        let file_name = src.file_name().ok_or_else(|| anyhow::anyhow!("Source path has no file name: {}", src.display()))?;
        let dest = content_dir.join(file_name);
        fs::copy(src, &dest).with_context(|| format!("Failed to copy {} to {}", src.display(), dest.display()))?;
        Ok(())
    }
    fn get_object<P: AsRef<Path>>(&self, object_id: &str, dest_path: P) -> Result<()> {
        let repo_path = Path::new(&self.root);
        let object_root = repo_path.join(object_id);
        if !object_root.exists() {
            anyhow::bail!("OCFL object {} does not exist", object_id);
        }
        let object_spec = object_root.join("0=ocfl_object_1.1");
        if !object_spec.exists() {
            anyhow::bail!("Not a valid OCFL object: missing {}", object_spec.display());
        }
        let content_dir = object_root.join("content");
        if !content_dir.exists() {
            anyhow::bail!("OCFL object content directory missing: {}", content_dir.display());
        }
        let dest = dest_path.as_ref();
        if dest.exists() {
            anyhow::bail!("Destination already exists: {}", dest.display());
        }
        // For simplicity, copy all files in content_dir to dest (if dest is a dir, else copy single file)
        if content_dir.is_dir() {
            fs::create_dir_all(dest).with_context(|| format!("Failed to create destination dir: {}", dest.display()))?;
            for entry in fs::read_dir(&content_dir).with_context(|| format!("Failed to read content dir: {}", content_dir.display()))? {
                let entry = entry?;
                let file_type = entry.file_type()?;
                if file_type.is_file() {
                    let file_name = entry.file_name();
                    let src_file = entry.path();
                    let dest_file = dest.join(file_name);
                    fs::copy(&src_file, &dest_file).with_context(|| format!("Failed to copy {} to {}", src_file.display(), dest_file.display()))?;
                }
            }
        } else {
            anyhow::bail!("Content directory is not a directory: {}", content_dir.display());
        }
        Ok(())
    }
    fn list_objects(&self) -> Result<Vec<String>> {
        let repo_path = Path::new(&self.root);
        if !repo_path.exists() {
            anyhow::bail!("OCFL repository does not exist at {}", repo_path.display());
        }
        let spec_file = repo_path.join("0=ocfl_1.1");
        if !spec_file.exists() {
            anyhow::bail!("Not a valid OCFL v1.1 repository: missing {}", spec_file.display());
        }
        let mut objects = Vec::new();
        for entry in fs::read_dir(repo_path).with_context(|| format!("Failed to read repo dir: {}", repo_path.display()))? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                let object_spec = path.join("0=ocfl_object_1.1");
                if object_spec.exists() {
                    if let Some(name) = path.file_name().and_then(|n| n.to_str()) {
                        objects.push(name.to_string());
                    }
                }
            }
        }
        Ok(objects)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ocfl_repo_trait() {
        let repo = OcflRepoImpl::new("/tmp/ocfl");
        assert!(repo.init_repo("/tmp/ocfl").is_ok());
        assert!(repo.add_object("obj1", "/tmp/file1").is_ok());
        assert!(repo.get_object("obj1", "/tmp/out").is_ok());
        assert!(repo.list_objects().is_ok());
    }
}
