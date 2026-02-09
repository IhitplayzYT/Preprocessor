#[allow(unused_imports, non_snake_case, non_camel_case_types, dead_code)]
use crate::Preprocessor_Struct::Prerustc;
use std::{env, fs::{self, File},path::Path};
impl Prerustc {
    fn build(&self) -> bool {
        let (c_txt, h_txt) = self.fmt_Tok();
        true
    }

    fn fmt_Tok(&self) -> (String, String) {
        let mut c_buff = "".to_string();
        for i in &self.ret_tok_c {
            if i == "" {
                continue;
            }
            let ed = i.bytes().last().unwrap();
            match ed {
                b';' | b'}' => {
                    c_buff += "\n";
                }
                _ => {
                    c_buff += " ";
                }
            }
            c_buff += i;
        }
        let mut h_buff = "".to_string();
        for i in &self.ret_tok_h {
            if i == "" {
                continue;
            }
            let ed = i.bytes().last().unwrap();
            match ed {
                b';' | b'}' => {
                    h_buff += "\n";
                }
                _ => {
                    h_buff += " ";
                }
            }
            h_buff += i;
        }
        (c_buff, h_buff)
    }


    // TODO: Decide if we neede . temp files for storring old processed code ??
    fn create(&self, tempfname: String, text: String) -> std::io::Result<bool,> {
        let dir = env::current_dir()?;
        let dir = dir.join(Path::new(&tempfname[..]));
        let file;
        if let Ok(x) = fs::exists(dir.as_path()){
            if !x{ 
                fs::File::create_new(dir.as_path())?;
            }
        }
        file = fs::File::open(dir)?;

        Ok(true)

    }

// FIXME: If we decided no then we dont need this    
    fn swap(&self, f1: String, f2: String) -> bool {

        true
    }
}
