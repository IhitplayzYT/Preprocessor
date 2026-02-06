#[warn(non_camel_case_types,non_snake_case,non_upper_case_globals,unused_imports,dead_code)]
mod util;
mod errors;
mod c;
mod h;
mod Preprocessor_Struct;
mod PrerustC;
use std::env;
fn main() {
    let arg: Vec<String> = env::args().collect();
    if arg.len() != 2{
        return;
    }
    let fname = &arg[1][..];
    let mut PREPROC = Preprocessor_Struct::Prerustc::new(fname).unwrap();
    PREPROC.process().unwrap();
    println!("{:?}",PREPROC.ret_tok_c);
}
