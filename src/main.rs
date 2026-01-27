mod util;
mod lib;
mod PrerustC;
use std::{env, process::exit,fs};

use crate::util::util::open_file;
fn main() {
    let arg: Vec<String> = env::args().collect();
    if arg.len() != 2{
        return;
    }
    let tokens = open_file(&arg[1]).unwrap();
    let l = tokens.len();

    let mut scope = 0;
    let mut ret: Vec<String> = Vec::new();


        


}
