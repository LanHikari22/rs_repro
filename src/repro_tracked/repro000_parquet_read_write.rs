/// For basic reading/writing with parquet_derive.
/// For development context, see https://github.com/dism-exe/dism-exe-notes/blob/main/lan/llm/weekly/Wk%2025%20003%20Rust%20Parquet%20serialize%20and%20deserialize.md
/// 

use std::fs::File;
use std::sync::Arc;

use parquet::file::reader::FileReader;
use parquet::{self, file::reader::SerializedFileReader};
use parquet::file::properties::WriterProperties;
use parquet::file::writer::SerializedFileWriter;
use parquet::record::{RecordReader, RecordWriter};
use parquet_derive::{ParquetRecordReader, ParquetRecordWriter};

// #[derive(Debug, ParquetRecordWriter, ParquetRecordReader)]
// struct MetaInformation {
//     created_at: String,
//     author: String,
// }

// #[derive(Debug, ParquetRecordWriter, ParquetRecordReader)]
// enum UserRole {
//     User,
//     Admin,
// }

#[derive(Debug, ParquetRecordWriter, ParquetRecordReader)]
struct User {
    name: String,
    email: String,
    comment: String,
    // role: UserRole,
}

/// similar to repro001
fn write_some_records() {
    let users = vec![
        User {
            name: "Alice".into(),
            email: "alice@example.com".into(),
            comment: "New\nLine, and \"quotes\"".into(),
        },
        User {
            name: "Bob".into(),
            email: "bob@example.com".into(),
            comment: "Tabs\ttoo".into(),
        },
    ];

    let schema = users.as_slice().schema().unwrap();
    let props = Arc::new(WriterProperties::builder().build());
    let file = File::create("example.parquet").unwrap();
    let mut writer = SerializedFileWriter::new(file, schema, props).unwrap();
    let mut row_group = writer.next_row_group().unwrap();

    users
        .as_slice()
        .write_to_row_group(&mut row_group)
        .unwrap();

    row_group.close().unwrap();
    writer.close().unwrap();
}

/// similar to repro002
fn read_some_records() -> Vec<User> {
  let mut users: Vec<User> = Vec::new();
  let file = File::open("example.parquet").unwrap();

  let reader = SerializedFileReader::new(file).unwrap();
  let mut row_group = reader.get_row_group(0).unwrap();
  users.read_from_row_group(&mut *row_group, 2).unwrap();
  users
}

pub fn main() {
    write_some_records();
    let users = read_some_records();

    println!("users: {:?}", users);
}