//! Experiment in delta-trace

macro_rules! macro000 {
    {3} => {};
}

fn use_macro000() {
    /*
        cargo +nightly expand --bin expt001_4ac46d_macro_rules

        # out (relevant)

        //! Experiment in delta-trace
        #[macro_use]
        extern crate std;
        #[prelude_import]
        use std::prelude::rust_2024::*;
        fn use_macro000() {}
    */
    macro000!(3);

    /*
            error: no rules expected `4`
          --> src/bin/expt001_4ac46d_macro_rules.rs:20:15
           |
        3  | macro_rules! macro000 {
           | --------------------- when calling this macro
        ...
        20 |     macro000!(4);
           |               ^ no rules expected this token in macro call
           |
        note: while trying to match `3`
          --> src/bin/expt001_4ac46d_macro_rules.rs:4:6
           |
        4  |     {3} => {};
           |      ^
     */

}

macro_rules! macro001 {
    {$($name:expr),+ $(,)?} => {
        $(
            println!("{}", $name);
        )*
    }
}

fn use_macro001() {
    /*
        cargo +nightly expand --bin expt001_4ac46d_macro_rules

        # out (relevant)

        #![feature(prelude_import)]
        //! Experiment in delta-trace
        #[macro_use]
        extern crate std;
        #[prelude_import]
        use std::prelude::rust_2024::*;
        fn use_macro001() {
            {
                ::std::io::_print(format_args!("{0}\n", "aaa"));
            };
            {
                ::std::io::_print(format_args!("{0}\n", "bbb"));
            };
            {
                ::std::io::_print(format_args!("{0}\n", "ccc"));
            };
        }
    */
    macro001! {
        "aaa", "bbb", "ccc",
    }

    // Note use of "$(,)?" for optional trailing comma.
}

macro_rules! macro002 {
    {$($name:ident: $typ:ty),+ $(,)?} => {
        pub struct Macro002<'a> {
            $(
                pub $name: $typ,
            )*
        }
    }
}

fn use_macro002() {
    /*
        cargo +nightly expand --bin expt001_4ac46d_macro_rules

        # out (relevant)

        #![feature(prelude_import)]
        //! Experiment in delta-trace
        #[macro_use]
        extern crate std;
        #[prelude_import]
        use std::prelude::rust_2024::*;
        fn use_macro002() {
            pub struct Macro002<'a> {
                pub _person: &'a str,
                pub _coins: i32,
            }
            let _ = Macro002 { _person: "", _coins: 0 };
        }
    */
    macro002! {
        _person: &'a str,
        _coins: i32,
    }

    let _ = Macro002 { _person: "", _coins: 0 };
}


fn main() {
    use_macro000();
    use_macro001();
    use_macro002();
}
