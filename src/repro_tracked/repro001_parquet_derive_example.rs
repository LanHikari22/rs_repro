///
/// This is a reproduction of an example from https://docs.rs/parquet_derive/30.0.1/parquet_derive/derive.ParquetRecordWriter.html
/// It was also referenced in stackoverflow: https://stackoverflow.com/questions/75124404/creating-datafusions-dataframe-from-vecstruct-in-rust/75125062#75125062
/// For development context, see https://github.com/dism-exe/dism-exe-notes/blob/main/lan/llm/weekly/Wk%2025%20003%20Rust%20Parquet%20serialize%20and%20deserialize.md
///
/// As of 2025-06-21, it is marked "not tested". So here, we test it.
///
/// To run this example:
/// ```sh
/// git clone https://github.com/LanHikari22/rs_repro.git && cd rs_repro && cargo run --features "repro001"
/// ```
/// 

/// see mod fix_2025_06_21 for commentary on the fix. Below is code as it exists in the example via [#7733](<https://github.com/apache/arrow-rs/pull/7733>)
use parquet::file::properties::WriterProperties;
use parquet::file::writer::SerializedFileWriter;
use parquet::record::RecordWriter;
use parquet_derive::ParquetRecordWriter;
use std::fs::File;
use std::sync::Arc;

// For reader
use parquet::file::reader::{FileReader, SerializedFileReader};
use parquet::record::RecordReader;
use parquet_derive::ParquetRecordReader;

#[derive(Debug, ParquetRecordWriter, ParquetRecordReader)]
struct ACompleteRecord {
    pub a_bool: bool,
    pub a_string: String,
}

fn write_some_records() {
    let samples = vec![
        ACompleteRecord {
            a_bool: true,
            a_string: "I'm true".into(),
        },
        ACompleteRecord {
            a_bool: false,
            a_string: "I'm false".into(),
        },
    ];

    let schema = samples.as_slice().schema().unwrap();

    let props = Arc::new(WriterProperties::builder().build());

    let file = File::create("example.parquet").unwrap();

    let mut writer = SerializedFileWriter::new(file, schema, props).unwrap();

    let mut row_group = writer.next_row_group().unwrap();

    samples
        .as_slice()
        .write_to_row_group(&mut row_group)
        .unwrap();

    row_group.close().unwrap();

    writer.close().unwrap();
}

fn read_some_records() -> Vec<ACompleteRecord> {
    let mut samples: Vec<ACompleteRecord> = Vec::new();
    let file = File::open("example.parquet").unwrap();

    let reader = SerializedFileReader::new(file).unwrap();
    let mut row_group = reader.get_row_group(0).unwrap();
    samples.read_from_row_group(&mut *row_group, 2).unwrap();

    samples
}

pub fn main() {
    write_some_records();

    let records = read_some_records();

    std::fs::remove_file("example.parquet").unwrap();

    assert_eq!(
        format!("{:?}", records),
        "[ACompleteRecord { a_bool: true, a_string: \"I'm true\" }, ACompleteRecord { a_bool: false, a_string: \"I'm false\" }]"
    );
}

mod fix_2025_06_21 {
    use parquet::file::properties::WriterProperties;
    use parquet::file::writer::SerializedFileWriter;

    // Extra imports that had to be added:
    use parquet::record::RecordWriter;
    use parquet_derive::ParquetRecordWriter;
    use std::fs::File;

    use std::sync::Arc;

    #[derive(ParquetRecordWriter)]
    struct ACompleteRecord<'a> {
        pub a_bool: bool,
        pub a_str: &'a str,
    }

    pub fn write_some_records() {
        let samples = vec![
            ACompleteRecord {
                a_bool: true,
                a_str: "I'm true",
            },
            ACompleteRecord {
                a_bool: false,
                a_str: "I'm false",
            },
        ];

        let schema = samples.as_slice().schema();

        let props = Arc::new(WriterProperties::builder().build());

        // Added
        let file = File::create("example.parquet").unwrap();

        // Added:
        let schema = schema.unwrap();

        let mut writer = SerializedFileWriter::new(file, schema, props).unwrap();

        let mut row_group = writer.next_row_group().unwrap();

        samples
            .as_slice()
            .write_to_row_group(&mut row_group)
            .unwrap();

        row_group.close().unwrap();
        // writer.close_row_group(row_group).unwrap(); // this doesn't work

        writer.close().unwrap();
    }

    // Added:
    pub fn main() {
        write_some_records();
    }

    /*
    - [OK] (Error1):
    error[E0423]: expected value, found macro `file`
    --> src/repro_tracked/repro001_parquet_derive_example.rs:35:45
    |
    35 |  let mut writer = SerializedFileWriter::new(file, schema, props).unwrap();
    |                                             ^^^^ not a value


    - [OK] (Error2):
    error[E0308]: mismatched types
    --> src/repro_tracked/repro001_parquet_derive_example.rs:35:51
        |
    35  |  let mut writer = SerializedFileWriter::new(file, schema, props).unwrap();
        |                   -------------------------       ^^^^^^ expected `Arc<Type>`, found `Result<Arc<Type>, ParquetError>`
        |                   |
        |                   arguments to this function are incorrect
        |
        = note: expected struct `Arc<_>`
                    found enum `Result<Arc<_>, ParquetError>`
    note: associated function defined here
    --> /home/lan/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parquet-55.1.0/src/file/writer.rs:181:12
        |
    181 |     pub fn new(buf: W, schema: TypePtr, properties: WriterPropertiesPtr) -> Result<Self> {
        |            ^^^
    help: consider using `Result::expect` to unwrap the `Result<Arc<parquet::schema::types::Type>, ParquetError>` value, panicking if the value is a `Result::Err`
        |
    35  |  let mut writer = SerializedFileWriter::new(file, schema.expect("REASON"), props).unwrap();
        |                                                         +++++++++++++++++


    - [OK] (Error3):
    error[E0599]: no method named `close_row_group` found for struct `SerializedFileWriter` in the current scope
    --> src/repro_tracked/repro001_parquet_derive_example.rs:51:12
        |
    51  |     writer.close_row_group(row_group).unwrap();
        |            ^^^^^^^^^^^^^^^
        |
    help: there is a method `flushed_row_groups` with a similar name, but with different arguments
    --> /home/lan/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parquet-55.1.0/src/file/writer.rs:284:5
        |
    284 |     pub fn flushed_row_groups(&self) -> &[RowGroupMetaData] {
        |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

    */
}
