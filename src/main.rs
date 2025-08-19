#![allow(unused_imports)]
#![allow(dead_code)]

#[cfg(feature = "nontracked")]
pub mod repro;

pub mod repro_tracked;

fn main() {
    #[cfg(feature = "nontracked")]
    repro_issues::prop_trait_on_vect1::main();

    #[cfg(feature = "scratch")]
    println!("regex: `{}`", format!(r##""(.*?)"\s*"##));

    #[cfg(feature = "repro000")]
    repro_tracked::repro000_parquet_read_write::main();

    #[cfg(feature = "repro001")]
    repro_tracked::repro001_parquet_derive_example::main();

    #[cfg(feature = "repro002")]
    repro_tracked::repro002_parquet_derive_read_example::main();

    #[cfg(feature = "repro003")]
    repro_tracked::repro003_ron_read_write::main();

    #[cfg(feature = "repro004")]
    repro_tracked::repro004_regex_tester::main();

    #[cfg(feature = "repro005")]
    repro_tracked::repro005_cust_chunk_no_type::main();

    #[cfg(feature = "repro006")]
    repro_tracked::repro006_sluggify_and_github_slugger::main();
}