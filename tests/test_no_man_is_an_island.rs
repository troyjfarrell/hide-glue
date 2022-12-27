use std::env;
use std::fs::{File, OpenOptions};
use std::io::{BufReader, BufWriter, Write};
use std::path::PathBuf;

use hide_glue::file_reader::TextFileReader;
use hide_glue::fixture::TemporaryFileFixture;

#[test]
fn no_man_is_an_island() {
    let no_man = TemporaryFileFixture::copy("no-man-is-an-island.txt");
    let no_test = TemporaryFileFixture::blank("no-test-is-an-island.txt");
    {
        use std::io::BufRead;

        let source_file = File::open(no_man.get_path()).unwrap();
        let reader = BufReader::new(&source_file);
        let destination_file = OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(no_test.get_path())
            .unwrap();
        let mut writer = BufWriter::new(&destination_file);
        for line in reader.lines() {
            let mut changed_line = String::from(line.unwrap());
            if changed_line.contains("Man") {
                changed_line = changed_line.replace("Man", "Test");
            }
            if changed_line.contains("man") && !changed_line.contains("manor") {
                changed_line = changed_line.replace("man", "test");
            }
            if changed_line.ends_with("Donne") {
                changed_line.push_str(" (sort of)");
            }
            let mut write_count = writer.write(changed_line.as_bytes()).unwrap();
            if write_count < changed_line.len() {
                panic!("Failed to write line to destination file");
            }
            write_count = writer.write(&[0x0a]).unwrap();
            if write_count != 1 {
                panic!("Failed to write new line to destination file");
            }
        }
        writer.flush().unwrap();
    }

    let key = "CARGO_MANIFEST_DIR";
    let cargo_manifest_dir =
        env::var(key).unwrap_or_else(|_| format!("Failed to get the {} environment variable", key));
    let expected_path: PathBuf = [
        cargo_manifest_dir,
        "tests/fixtures/no-test-is-an-island.txt".to_string(),
    ]
    .iter()
    .collect();
    assert_eq!(
        TextFileReader::new(&no_test).expect("Failed to read no_test file"),
        TextFileReader::new(&expected_path).expect("Failed to read expect_path file")
    )
}
