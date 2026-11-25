use base64::{Engine as _, engine::general_purpose::URL_SAFE_NO_PAD};
use sha2::{Digest, Sha256};
use std::io::{Cursor, Write};
use std::path::{Path, PathBuf};
use zip::ZipWriter;
use zip::write::SimpleFileOptions;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Package {
    pub name: String,
    pub version: String,
    pub requires_python: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GeneratedWheel {
    pub name: String,
    pub filename: String,
    pub version: String,
    pub requires_python: String,
    pub sha256_hash: String,
    pub file_path: String,
    pub path: PathBuf,
}

pub fn generate_wheel(package: &Package) -> zip::result::ZipResult<Vec<u8>> {
    let dist_info = format!("{}-{}.dist-info", package.name, package.version);

    let mut buffer = Cursor::new(Vec::new());
    let mut zip = ZipWriter::new(&mut buffer);
    let options = SimpleFileOptions::default();

    let mut records = Vec::new();

    // Helper closure to write a file, compute its SHA-256 + size, and log it for RECORD
    let mut add_file = |path: &str, content: &[u8]| -> std::io::Result<()> {
        zip.start_file(path, options)?;
        zip.write_all(content)?;

        let hash = Sha256::digest(content);
        let b64_hash = URL_SAFE_NO_PAD.encode(hash);
        records.push(format!("{},sha256={},{}", path, b64_hash, content.len()));
        Ok(())
    };

    // Package source code
    add_file(
        &format!("{}/__init__.py", package.name),
        b"def hello():\n    return 'Hello from Rust-generated wheel!'\n",
    )?;

    // METADATA
    let metadata = format!(
        "Metadata-Version: 2.1\nName: {}\nVersion: {}\nRequires-Python: >={}\n",
        package.name, package.version, package.requires_python
    );
    add_file(&format!("{}/METADATA", dist_info), metadata.as_bytes())?;

    // WHEEL specification
    let wheel_spec = "Wheel-Version: 1.0\nGenerator: custom-rust-builder\nRoot-Is-Purelib: true\nTag: py3-none-any\n";
    add_file(&format!("{}/WHEEL", dist_info), wheel_spec.as_bytes())?;

    // RECORD (lists all files; RECORD itself has an empty hash and size)
    let record_path = format!("{}/RECORD", dist_info);
    records.push(format!("{},,", record_path));
    let record_content = records.join("\n") + "\n";

    zip.start_file(&record_path, options)?;
    zip.write_all(record_content.as_bytes())?;

    zip.finish()?;
    Ok(buffer.into_inner())
}

pub fn generate_wheel_file(
    output_dir: impl AsRef<Path>,
    package: Package,
) -> std::io::Result<GeneratedWheel> {
    let wheel_bytes =
        generate_wheel(&package).map_err(|error| std::io::Error::other(error.to_string()))?;
    let wheel_name = package.name.replace('-', "_");
    let filename = format!("{}-{}-py3-none-any.whl", wheel_name, package.version);
    let normalized_name = package
        .name
        .chars()
        .map(|character| character.to_ascii_lowercase())
        .collect::<String>()
        .replace('_', "-");
    let file_path = format!("{normalized_name}/{filename}");
    let path = output_dir.as_ref().join(&file_path);

    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent)?;
    }
    std::fs::write(&path, &wheel_bytes)?;

    let sha256_hash = hex::encode(Sha256::digest(wheel_bytes));
    Ok(GeneratedWheel {
        name: package.name,
        filename,
        version: package.version,
        requires_python: package.requires_python,
        sha256_hash,
        file_path,
        path,
    })
}

pub fn generate_fake_wheels(
    output_dir: impl AsRef<Path>,
    count: usize,
) -> std::io::Result<Vec<GeneratedWheel>> {
    let mut wheels = Vec::with_capacity(count);

    for index in 1..=count {
        let package = Package {
            name: petname::petname(rand::random_range(1..=3), "-")
                .expect("should create a petname"),
            version: format!("0.{}.0", index),
            requires_python: "3.10".to_owned(),
        };
        wheels.push(generate_wheel_file(output_dir.as_ref(), package)?);
    }

    Ok(wheels)
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn test_generating_basic_wheel() {
        let result = generate_wheel(&Package {
            name: "lorem".to_owned(),
            version: "0.1.0".to_owned(),
            requires_python: "3.10".to_owned(),
        });
        assert!(result.unwrap().len() > 0);
    }

    #[test]
    fn test_generating_fake_wheels() {
        let directory = tempdir().unwrap();
        let wheels = generate_fake_wheels(directory.path(), 10).unwrap();

        assert_eq!(wheels.len(), 10);
        for wheel in wheels {
            assert!(wheel.path.is_file());
            assert!(wheel.file_path.ends_with(".whl"));
            assert!((1..=3).contains(&wheel.name.split('-').count()));
        }
    }
}
