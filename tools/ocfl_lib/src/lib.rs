// OCFL v1.1 core library API skeleton
// Functional, minimal 'unsafe', anyhow for error handling

extern crate serde;
extern crate serde_json;
extern crate sha2;
extern crate chrono;

use anyhow::{Context, Result};
use serde::{Serialize, Deserialize};
use std::fs;
use std::fs::create_dir_all;
use std::fs::File;
use std::io::Write;
use std::path::Path;

#[derive(Serialize, Deserialize)]
pub struct Inventory {
    pub id: String,
    pub type_field: String,
    pub digest_algorithm: String,
    pub head: String,
    pub manifest: serde_json::Value,
    pub versions: serde_json::Value,
}

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
    /// Add a new version to an existing object
    fn add_object_version<P: AsRef<Path>>(&self, object_id: &str, src_path: P) -> Result<()>;
    /// Retrieve the inventory.json for an object as serde_json::Value
    fn get_inventory(&self, object_id: &str) -> Result<serde_json::Value>;
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
        let spec_file = repo_path.join("0=ocfl_1.1");
        if spec_file.exists() {
            anyhow::bail!("OCFL repository already exists at {}", repo_path.display());
        }
        create_dir_all(repo_path).with_context(|| format!("Failed to create repo directory: {}", repo_path.display()))?;
        fs::write(&spec_file, "OCFL Object Root\nhttps://ocfl.io/1.1/spec/\n").with_context(|| format!("Failed to write OCFL spec file: {}", spec_file.display()))?;
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
        // --- BEGIN inventory.json creation ---
        let inventory = Inventory {
            id: object_id.to_string(),
            type_field: "https://ocfl.io/1.0/spec/#inventory".to_string(),
            digest_algorithm: "sha512".to_string(),
            head: "v1".to_string(),
            manifest: serde_json::json!({
                format!("{}", sha512_digest(&dest)?): [format!("v1/content/{}", file_name.to_string_lossy())]
            }),
            versions: serde_json::json!({
                "v1": {
                    "created": chrono::Utc::now().to_rfc3339(),
                    "message": "Initial version",
                    "user": { "name": "system" },
                    "state": {
                        format!("{}", sha512_digest(&dest)?): [format!("content/{}", file_name.to_string_lossy())]
                    }
                }
            }),
        };
        let inventory_path = object_root.join("inventory.json");
        let mut file = File::create(&inventory_path).with_context(|| format!("Failed to create inventory.json: {}", inventory_path.display()))?;
        let json = serde_json::to_string_pretty(&inventory)?;
        file.write_all(json.as_bytes()).with_context(|| format!("Failed to write inventory.json: {}", inventory_path.display()))?;
        // --- END inventory.json creation ---
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
        if content_dir.is_dir() {
            let files: Vec<_> = fs::read_dir(&content_dir)
                .with_context(|| format!("Failed to read content dir: {}", content_dir.display()))?
                .filter_map(|e| e.ok())
                .filter(|e| e.file_type().map(|ft| ft.is_file()).unwrap_or(false))
                .collect();
            if files.len() == 1 {
                // If only one file, copy it directly to dest (as a file)
                let src_file = files[0].path();
                fs::copy(&src_file, dest).with_context(|| format!("Failed to copy {} to {}", src_file.display(), dest.display()))?;
            } else {
                // Multiple files: copy all into dest directory
                fs::create_dir_all(dest).with_context(|| format!("Failed to create destination dir: {}", dest.display()))?;
                for entry in files {
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
    fn add_object_version<P: AsRef<Path>>(&self, object_id: &str, src_path: P) -> Result<()> {
        let repo_path = Path::new(&self.root);
        let object_root = repo_path.join(object_id);
        if !object_root.exists() {
            anyhow::bail!("OCFL object {} does not exist", object_id);
        }
        let inventory_path = object_root.join("inventory.json");
        let mut inventory: Inventory = {
            let data = fs::read_to_string(&inventory_path).with_context(|| format!("Failed to read inventory.json: {}", inventory_path.display()))?;
            serde_json::from_str(&data).with_context(|| format!("Invalid inventory.json: {}", inventory_path.display()))?
        };
        // Determine next version
        let mut versions: Vec<String> = inventory.versions.as_object().unwrap().keys().cloned().collect();
        versions.sort();
        let last_version = versions.last().unwrap();
        let next_version = format!("v{}", last_version[1..].parse::<u32>().unwrap() + 1);
        // Create new version dir
        let version_dir = object_root.join(&next_version).join("content");
        create_dir_all(&version_dir).with_context(|| format!("Failed to create version dir: {}", version_dir.display()))?;
        let src = src_path.as_ref();
        let file_name = src.file_name().ok_or_else(|| anyhow::anyhow!("Source path has no file name: {}", src.display()))?;
        let dest = version_dir.join(file_name);
        fs::copy(src, &dest).with_context(|| format!("Failed to copy {} to {}", src.display(), dest.display()))?;
        let digest = sha512_digest(&dest)?;
        // Update manifest
        let mut manifest = inventory.manifest.as_object().cloned().unwrap_or_default();
        manifest.entry(digest.clone()).or_insert_with(|| serde_json::json!([]));
        let arr = manifest.get_mut(&digest).unwrap().as_array_mut().unwrap();
        arr.push(serde_json::json!(format!("{}/content/{}", next_version, file_name.to_string_lossy())));
        // Update versions
        let mut versions_map = inventory.versions.as_object().cloned().unwrap_or_default();
        versions_map.insert(next_version.clone(), serde_json::json!({
            "created": chrono::Utc::now().to_rfc3339(),
            "message": "New version",
            "user": { "name": "system" },
            "state": {
                digest.clone(): [format!("content/{}", file_name.to_string_lossy())]
            }
        }));
        // Update inventory
        inventory.head = next_version.clone();
        inventory.manifest = serde_json::json!(manifest);
        inventory.versions = serde_json::json!(versions_map);
        let mut file = File::create(&inventory_path).with_context(|| format!("Failed to update inventory.json: {}", inventory_path.display()))?;
        let json = serde_json::to_string_pretty(&inventory)?;
        file.write_all(json.as_bytes()).with_context(|| format!("Failed to write inventory.json: {}", inventory_path.display()))?;
        Ok(())
    }
    fn get_inventory(&self, object_id: &str) -> Result<serde_json::Value> {
        let repo_path = Path::new(&self.root);
        let object_root = repo_path.join(object_id);
        let inventory_path = object_root.join("inventory.json");
        let data = fs::read_to_string(&inventory_path).with_context(|| format!("Failed to read inventory.json: {}", inventory_path.display()))?;
        let v: serde_json::Value = serde_json::from_str(&data).with_context(|| format!("Invalid inventory.json: {}", inventory_path.display()))?;
        Ok(v)
    }
}

fn sha512_digest<P: AsRef<Path>>(path: P) -> Result<String> {
    use sha2::{Sha512, Digest};
    use std::fs::File;
    use std::io::{BufReader, Read};
    let mut file = BufReader::new(File::open(path)?);
    let mut hasher = Sha512::new();
    let mut buf = [0u8; 4096];
    loop {
        let n = file.read(&mut buf)?;
        if n == 0 { break; }
        hasher.update(&buf[..n]);
    }
    Ok(format!("{:x}", hasher.finalize()))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_ocfl_repo_trait() {
        let dir = tempfile::tempdir().expect("tempdir");
        let root = dir.path().to_str().unwrap();
        let repo = OcflRepoImpl::new(root);

        assert!(repo.init_repo(root).is_ok());

        let src = dir.path().join("file1.txt");
        fs::write(&src, b"hello").unwrap();
        assert!(repo.add_object("obj1", &src).is_ok());

        let dest = dir.path().join("out");
        assert!(repo.get_object("obj1", &dest).is_ok());

        let objects = repo.list_objects().unwrap();
        assert!(objects.contains(&"obj1".to_string()));
    }
}
