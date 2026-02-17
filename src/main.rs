#![allow(non_camel_case_types,non_snake_case,non_upper_case_globals,unused_features,unused_imports,dead_code)]
mod Preprocessor_Struct;
mod PrerustC;
mod c;
mod convert;
mod errors;
mod h;
#[warn(
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_imports,
    dead_code
)]
mod util;
use std::env;
fn main() {
    let arg: Vec<String> = env::args().collect();
    if arg.len() != 2 {
        return;
    }
    let fname = &arg[1][..];
    let mut PREPROC = Preprocessor_Struct::Prerustc::new(fname).unwrap();
    PREPROC.build();
}
