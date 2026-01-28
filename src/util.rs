#[warn(non_camel_case_types,non_snake_case,non_upper_case_globals,unused_imports)]
pub mod util {
use std::cmp::{min,max};
use std::{fs, io};
    #[macro_export]
    macro_rules! min{
        ($x:expr) => {$x};
        ($x:expr,$($y:expr),+) => {min($x,min!($($y),+))};
    }
    #[macro_export]
    macro_rules! max{
        ($x:expr) => {$x};
        ($x:expr,$($y:expr),+) => {max($x,max!($($y),+))};
    }

pub fn open_file(path: &str) -> Result<Vec<String>,io::Error> {
let text = fs::read_to_string(path)?;
let words = text
.split_ascii_whitespace() 
.map(|s| s.to_owned())
.collect();
return Ok(words);
}

pub fn get_h(name : &str) -> String {
let mut ret = name.to_string();
ret.pop();
ret.push('h');
ret
}





}