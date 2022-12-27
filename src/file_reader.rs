//! A text file reader
use std::fmt;
use std::fs::File;
use std::io::SeekFrom::Start;
use std::io::{Read, Seek};
use std::path::Path;

const FAILED_READ_SENTINEL: usize = 1234567890;
// We need a second one to ensure inequality for the PartialEq trait.
const FAILED_READ_SENTINEL_2: usize = 12345678901;

pub struct TextFileReader<'a> {
    filepath: &'a Path,
    file: File,
    size: u64,
}

/// [`TextFileReader`] simplifies reading a file for assertions.  The struct implements
/// [`PartialEq`] and [`fmt::Debug`] to simplify debugging.
///
/// # Usage
///
/// ```no_run
/// use std::fs::OpenOptions;
/// use std::path::Path;
/// use hide_glue::file_reader::TextFileReader;
///
/// let test_output_path = Path::new("/temporary/file/path");
/// let expected_output_path = Path::new("tests/fixtures/expected.txt");
/// let test_output = OpenOptions::new()
///     .write(true)
///     .create_new(true)
///     .open(test_output_path).unwrap();
///
/// // write some text to the file; don't forget to flush the buffers!
///
/// assert_eq!(
///     TextFileReader::new(test_output_path)
///         .expect("Failed to read test_output_path file"),
///     TextFileReader::new(expected_output_path)
///         .expect("Failed to read expected_output_path file")
/// );
/// ```
impl<'a> TextFileReader<'a> {
    /// Open the file and record its length.
    pub fn new(filepath: &'a Path) -> Result<Self, Box<dyn std::error::Error>> {
        let file = File::open(filepath)?;
        let size = file.metadata()?.len();
        Ok(Self {
            filepath,
            file,
            size,
        })
    }
}

impl fmt::Debug for TextFileReader<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        writeln!(f, "{}", self.filepath.to_string_lossy()).unwrap();
        let mut my_file = &self.file;
        my_file.seek(Start(0)).unwrap();
        let buffer: &mut [u8] = &mut [0; 1024];
        loop {
            // Because the interface does not allow reporting errors, write the error to the debug
            // text output.  This ought to be safe because the debug output of this type should not
            // be mission-critical.  If you are reading this comment and cursing this decision,
            // then you have made a mistake by depending on this output.
            match my_file.read(buffer) {
                Ok(bytes_read) => {
                    if bytes_read > 0 {
                        write!(f, "{}", String::from_utf8_lossy(&buffer[0..bytes_read])).unwrap();
                    } else {
                        break;
                    }
                }
                Err(err) => {
                    write!(
                        f,
                        "Error: failed to read source file {}: {}",
                        self.filepath.to_string_lossy(),
                        err
                    )
                    .unwrap();
                    return Err(fmt::Error);
                }
            }
        }
        Ok(())
    }
}

impl PartialEq for TextFileReader<'_> {
    fn eq(&self, other: &Self) -> bool {
        if self.size != other.size {
            return false;
        }
        let mut my_file = &self.file;
        let mut other_file = &other.file;
        my_file.seek(Start(0)).unwrap();
        other_file.seek(Start(0)).unwrap();
        let my_buffer: &mut [u8] = &mut [0; 1024];
        let other_buffer: &mut [u8] = &mut [0; 1024];
        loop {
            // It would be good if there were a way to report the read failure here without
            // breaking the Trait interface.
            let my_bytes_read = my_file.read(my_buffer).unwrap_or(FAILED_READ_SENTINEL);
            let other_bytes_read = other_file
                .read(other_buffer)
                .unwrap_or(FAILED_READ_SENTINEL_2);
            if my_bytes_read == other_bytes_read {
                if my_buffer != other_buffer {
                    return false;
                }
            } else {
                return false;
            }
            if my_bytes_read == 0 {
                break;
            }
        }
        true
    }
}
