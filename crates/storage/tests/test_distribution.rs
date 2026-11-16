use laks_storage::pkginfo::{Distribution, DistributionType, FileFormatVersion};

#[test]
fn test_parse_wheel() {
    let dist = Distribution::new("tests/fixtures/build-0.4.0-py2.py3-none-any.whl").unwrap();
    assert_eq!(dist.r#type(), DistributionType::Wheel);
    let metadata = dist.metadata();
    assert_eq!(metadata.metadata_version, FileFormatVersion::V2_1);
    assert_eq!(metadata.name, "build");
    assert!(metadata.home_page.is_none());
    assert!(metadata.download_url.is_none());
    assert_eq!(dist.python_version(), "py2.py3");
}

#[test]
fn test_parse_wheel_with_vendored_pkgs() {
    let dist = Distribution::new("tests/fixtures/py-1.11.0-py2.py3-none-any.whl").unwrap();
    assert_eq!(dist.r#type(), DistributionType::Wheel);
    let metadata = dist.metadata();
    assert_eq!(metadata.metadata_version, FileFormatVersion::V2_1);
    assert_eq!(metadata.name, "py");
    assert_eq!(dist.python_version(), "py2.py3");
}

#[test]
fn test_parse_egg() {
    let dist = Distribution::new("tests/fixtures/build-0.4.0-py3.9.egg").unwrap();
    assert_eq!(dist.r#type(), DistributionType::Egg);
    let metadata = dist.metadata();
    assert_eq!(metadata.metadata_version, FileFormatVersion::V2_1);
    assert_eq!(metadata.name, "build");
    assert!(metadata.home_page.is_none());
    assert!(metadata.download_url.is_none());
    assert_eq!(dist.python_version(), "py3.9");
}

#[test]
fn test_parse_sdist_zip() {
    let dist = Distribution::new("tests/fixtures/build-0.4.0.zip").unwrap();
    assert_eq!(dist.r#type(), DistributionType::SDist);
    let metadata = dist.metadata();
    assert_eq!(metadata.metadata_version, FileFormatVersion::V2_1);
    assert_eq!(metadata.name, "build");
    assert!(metadata.home_page.is_none());
    assert!(metadata.download_url.is_none());
    assert_eq!(dist.python_version(), "source");
}

#[test]
fn test_parse_sdist_tar_gz() {
    let dist = Distribution::new("tests/fixtures/build-0.4.0.tar.gz").unwrap();
    assert_eq!(dist.r#type(), DistributionType::SDist);
    let metadata = dist.metadata();
    assert_eq!(metadata.metadata_version, FileFormatVersion::V2_1);
    assert_eq!(metadata.name, "build");
    assert_eq!(metadata.author.as_deref(), Some("Filipe Laíns"));
    assert!(metadata.home_page.is_none());
    assert!(metadata.download_url.is_none());
    assert_eq!(dist.python_version(), "source");
}
