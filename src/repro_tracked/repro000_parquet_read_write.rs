#[cfg(feature = "repro000")]
pub mod repro000 {
    use std::fs::File;
    use std::sync::Arc;

    use parquet;
    use parquet::file::properties::WriterProperties;
    use parquet::file::writer::SerializedFileWriter;
    use parquet::record::RecordWriter;
    use parquet_derive::{ParquetRecordReader, ParquetRecordWriter};


    #[derive(Debug, ParquetRecordWriter, ParquetRecordReader)]
    struct User {
        name: String,
        email: String,
        comment: String,
    }

    pub fn main() -> Result<(), Box<dyn std::error::Error>> {
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


        // Write to Parquet
        let file = File::create("users.parquet")?;
        users.write_parquet(file)?;

        // Read from Parquet
        let file = File::open("users.parquet")?;
        let loaded: Vec<User> = Vec::read_parquet(file)?;

        println!("{:#?}", loaded);

        Ok(())
    }
}