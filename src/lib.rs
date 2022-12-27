//! Hide glue, like testing, holds beautiful things together.
//!
//! This library contains some helpers for writing tests with files.
//!
//! The [`file_reader`] module contains a [`TextFileReader`][`file_reader::TextFileReader`], which
//! simplifies tests which need to read and compare text files.  The [`fixture`] module contains a
//! [`TemporaryFileFixture`][`fixture::TemporaryFileFixture`], which will create a temporary file
//! in a temporary directory, optionally copying an existing file to the temporary file.
pub mod file_reader;
pub mod fixture;
