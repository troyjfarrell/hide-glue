//! A temporary file fixture
use std::env;
use std::path::{Path, PathBuf};

use tempfile::TempDir;

// Thanks Andrew Radev!
// https://andrewra.dev/2019/03/01/testing-in-rust-temporary-files/
/// Create a temporary file in a temporary directory, optionally populating the file with the
/// contents of a file in $CARGO_MANIFEST_DIR/tests/fixtures.
pub struct TemporaryFileFixture {
    path: PathBuf,
    source: PathBuf,
    _tempdir: TempDir,
}

impl TemporaryFileFixture {
    pub fn blank(fixture_filename: &str) -> Self {
        let tempdir =
            tempfile::tempdir().expect("Failed to initialize a temporary directory for a fixture");
        let mut path = PathBuf::from(&tempdir.path());
        path.push(fixture_filename);

        Self {
            path,
            source: PathBuf::new(),
            _tempdir: tempdir,
        }
    }
    pub fn copy(fixture_filename: &str) -> Self {
        let mut fixture = Self::blank(fixture_filename);

        let key = "CARGO_MANIFEST_DIR";
        let root = env::var(key)
            .unwrap_or_else(|_| format!("Failed to get the {} environment variable", key));
        fixture.source.push(root);
        fixture.source.push("tests/fixtures");
        fixture.source.push(fixture_filename);

        std::fs::copy(&fixture.source, &fixture.path)
            .expect("Failed to copy a fixture file to the temporary directory");
        fixture
    }

    pub fn get_path(&self) -> &Path {
        &self.path
    }
}

impl std::ops::Deref for TemporaryFileFixture {
    type Target = Path;

    fn deref(&self) -> &Self::Target {
        self.path.deref()
    }
}
