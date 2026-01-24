mod util;
mod lib;
use std::{env, process::exit,fs};
use util;

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

    for i in 0..l{
        let t = tokens[i].as_str();
        match t{
        "defer" => {
            let z = scope;
            for j in (i+2)..l{
                if tokens[i] == "{" {
                    scope += 1;
                }
                
                if tokens[i] == "}"{
                    scope -= 1;
                }
                if scope < z{
                    /* We found the end of out scope now we can go back by 1 index and place the free*/
                    break;
                } 
                    
            }


        },
        "@autowired" => {

        },
        "{" => {
            scope += 1 
        },
        "}" => {
            scope -=1;
        }
        "?." => {

        }
         _ => {},
        }


    }
        


}
