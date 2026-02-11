use crate::Preprocessor_Struct::Prerustc;
use crate::util::util::get_h;
use std::{env, fs::{self},path::Path};
#[allow(unused_imports, non_snake_case, non_camel_case_types,dead_code)]
impl Prerustc {
    pub fn build(&mut self) -> bool {
        self.process().unwrap();
        let (c_txt, h_txt) = self.fmt_Tok();
        self.create(c_txt, h_txt).unwrap();
        true
    }

    pub fn fmt_Tok(&self) -> (String, String) {
        let mut c_buff = "".to_string();
        for i in &self.ret_tok_c {
            if i == "" {
                continue;
            }
            
            let ed = i.bytes().last().unwrap();
            c_buff += i;
            match ed {
                b';' | b'{' |  b'}' | b'/' | b'>' | b'\'' | b'\"'=> {
                    c_buff += "\n";
                }
                _ => {
                    c_buff += " ";
                }
            }
        }

        let mut h_buff = "".to_string();
        for i in &self.ret_tok_h {
            if i == "" {
                continue;
            }
            let ed = i.bytes().last().unwrap();
            h_buff += i;
            match ed {
                b';' | b'{' | b'\'' | b'\"' |  b'}' | b'>' | b'/' => {
                    h_buff += "\n";
                }
                _ => {
                    h_buff += " ";
                }
            }
        }
        (c_buff, h_buff)
    }


    // TODO: Decide if we neede . temp files for storring old processed code ??
    // Args -> sample.c

    //Output -> sample.c sample.h .sample.c .sample.h

    fn create(&self,text_c: String,text_h: String) -> std::io::Result<bool> {
        let cfname = self.c_fname.clone();
        let tempcfname = ".".to_string() + &cfname[..];
        let temphfname = get_h(&tempcfname[..]);
        let hfname = get_h(&cfname[..]);

        let cwd = env::current_dir()?;        

        let (c_path,h_path,ct_path,ht_path) = (
        cwd.join(Path::new(&cfname)),
        cwd.join(Path::new(&hfname)),
        cwd.join(Path::new(&tempcfname)),
        cwd.join(Path::new(&temphfname))
        );

        if let Ok(x) = fs::exists(&c_path){
        if !x{
        fs::File::create_new(&c_path)?;
        }
        }
        if let Ok(x) = fs::exists(&h_path){
        if !x{
        fs::File::create_new(&h_path)?;
        }
        }
        if let Ok(x) = fs::exists(&ct_path){
        if !x{
        fs::File::create_new(&ct_path)?;
        }
        }
        if let Ok(x) = fs::exists(&ht_path){
        if !x{
        fs::File::create_new(&ht_path)?;
        }
        }

        let buff: String = fs::read_to_string(&c_path)?;
        fs::write(ct_path, buff)?;
        let buff: String = fs::read_to_string(&h_path)?;
        fs::write(ht_path, buff)?;       
        fs::write(c_path,text_c)?;
        fs::write(h_path,text_h)?;

        Ok(true)

    }


}
