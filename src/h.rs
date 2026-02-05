#[allow(non_camel_case_types,non_snake_case,dead_code)]

use std::collections::HashMap;
use crate::Preprocessor_Struct::Prerustc;
use crate::lib::preprocessor::*;
use std::sync::RwLock;
use lazy_static::lazy_static;

lazy_static! {
    #[derive(Debug)]
    pub static ref MAP: RwLock<HashMap<String, String>> =
        RwLock::new(HashMap::new());
}

fn set_kv(k: &str, v: &str) {
    let mut map = MAP.write().unwrap();
    map.insert(k.to_string(), v.to_string());
}

fn get_value(k: &str) -> Option<String> {
    let map = MAP.read().unwrap();
    map.get(k).cloned()
}


impl Prerustc{


    /*

        typedef struct Teacher<T> {
        T pid;
        char * name;
        } Teacher;


     */

    // Make compatible with typedef struct 
    pub fn def_generic(&mut self,idx: usize) -> ParserReturn<usize> { 
        let mut i = idx;
        let mut typdef_flg = false;
        let mut template = "".to_string();
        let upper = self.tok_h[i].find("<").unwrap();
        let mut name = self.tok_h[i][..upper].to_string();
        if i > 2 {
            i -= 2;
            if self.tok_h[i].contains("typedef"){
                template += "typedef struct";
                typdef_flg = true;
                self.ret_tok_h.remove(i);
                self.ret_tok_h.remove(i);
            }else{
                name = "struct ".to_string() + &name[..];
            }
        }
        i = idx;
        let mut scope = if self.tok_h[idx].ends_with("{") { 1 }  else { 0 }; 

        while scope != 0 || !self.tok_h[i].contains(";"){
            if self.tok_h[i].contains("{") {
                scope += 1;
            }
            if self.tok_h[i].contains("}") {
                scope -= 1;
            }
            template += " ";
            template += &self.tok_h[i][..];
            i += 1;
        }
        template += &self.tok_h[i][..];
        if typdef_flg {
            if self.tok_h[i].len() == 1 {
                name = self.tok_h[i-1].to_string();
            }else{
                name = self.tok_h[i][..self.tok_h[i].find(";").unwrap()].to_string();
            }
        }

        set_kv(&name[..], &template[..]);
        Ok(i - idx + 1)
    }

    pub fn populate_generics(&mut self,i:usize) -> ParserReturn<usize>{
        let mut name = self.tok_c[i][..self.tok_c[i].find("<").unwrap()].to_string();
        let mut template = get_value(&name[..]).unwrap();
        if template.is_empty() {
            name = "struct ".to_string() + &name[..];
            template = get_value(&name[..]).unwrap();
            if template.is_empty() {
                return Ok(1);
            }
        }

        let type_var = &self.tok_c[i][(self.tok_c[i].find("<").unwrap()+1)..self.tok_c[i].find(">").unwrap()];

        template = template.replace("<T>",&(name+ type_var)[..]);
        template = template.replace("T",type_var);
        self.ret_tok_h.push(template);        
        Ok(1)
    }


}