use std::fmt::format;
#[allow(non_camel_case_types,non_snake_case,non_upper_case_globals,unused_features,unused_imports,dead_code)]
use std::fs;
use std::{io, path::Path};
use std::path::PathBuf;
use crate::util::util::get_h;

pub struct Prerustc{
pub tok_c: Vec<String>,
pub tok_h: Vec<String>,
pub ret_tok_c: Vec<String>,
pub ret_tok_h: Vec<String>,
pub c_fname:String,
}


impl Prerustc {

    pub fn swap(&self) -> io::Result<()>{
        let (c,h,t_c,t_h) = (self.c_fname.clone(),get_h(&self.c_fname),".".to_string()+ &self.c_fname[..],".".to_string()+ &get_h(&self.c_fname)[..]);
        let c_path = PathBuf::from(&self.c_fname);
        let h_path = PathBuf::from(get_h(&self.c_fname));

        let c_hidden = c_path.with_file_name(format!(".{}", c_path.file_name().unwrap().to_string_lossy()));
        let h_hidden = h_path.with_file_name(format!(".{}", h_path.file_name().unwrap().to_string_lossy()));

        self.swap_pair(&c_path, &c_hidden)?;
        self.swap_pair(&h_path, &h_hidden)?;

        Ok(())    
        
    }

    pub fn swap_pair(&self,A: &Path,B:&Path) -> io::Result<()> {
        if !A.exists() || !B.exists() {
            return Err(io::Error::new(
                io::ErrorKind::NotFound,
                format!("Swap files missing: {} <-> {}", A.display(), B.display())
            ));
        }

        let tmp = A.with_file_name(format!(".swap_tmp_{}",A.file_name().unwrap().to_string_lossy()));
        fs::rename(A, &tmp)?;
        fs::rename(B, A)?;
        fs::rename(tmp, B)?;
     Ok(())   
    }




}