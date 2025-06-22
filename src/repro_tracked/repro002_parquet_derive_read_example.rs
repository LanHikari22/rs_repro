///
/// This serves to test the example [ParquestRecordReader](<https://docs.rs/parquet_derive/latest/parquet_derive/derive.ParquetRecordReader.html>)
/// For development context, see https://github.com/dism-exe/dism-exe-notes/blob/main/lan/llm/weekly/Wk%2025%20003%20Rust%20Parquet%20serialize%20and%20deserialize.md

use parquet::record::RecordReader;
use parquet::file::{serialized_reader::SerializedFileReader, reader::FileReader};
use parquet_derive::{ParquetRecordReader};
use std::fs::File;

// Added Debug to show the results
#[derive(Debug, ParquetRecordReader)]
struct ACompleteRecord {
    pub a_bool: bool,
    pub a_str: String, // rename to a_string -> a_str for parity with repro001.
}

pub fn read_some_records() -> Vec<ACompleteRecord> {
  let mut samples: Vec<ACompleteRecord> = Vec::new();
  let file = File::open("example.parquet").unwrap(); // some_file.parquet -> example.parquet to match repro001.

  let reader = SerializedFileReader::new(file).unwrap();
  let mut row_group = reader.get_row_group(0).unwrap();
  samples.read_from_row_group(&mut *row_group, 2).unwrap(); // Seems we need to know the number of records beforehand...
  samples
}

pub fn main() {
    println!("{:?}", read_some_records());
}