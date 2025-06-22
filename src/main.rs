#![allow(unused_imports)]
#![allow(dead_code)]

#[cfg(feature = "nontracked")]
pub mod repro;

pub mod repro_tracked;

#[cfg(feature = "nontracked")]
fn main() {
    repro_issues::prop_trait_on_vect1::main();
}

#[cfg(feature = "repro000")]
fn main() {
    repro_tracked::repro000_parquet_read_write::main()
}

#[cfg(feature = "repro001")]
fn main() {
    repro_tracked::repro001_parquet_derive_example::main()
}

#[cfg(feature = "repro002")]
fn main() {
    repro_tracked::repro002_parquet_derive_read_example::main()
}


#[cfg(feature = "repro003")]
fn main() {
    repro_tracked::repro003_ron_read_write::main()
}
