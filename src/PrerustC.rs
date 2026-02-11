#[allow(non_camel_case_types,non_snake_case,non_upper_case_globals,unused_features,unused_imports,dead_code)]
pub mod Preprocess{
use crate::{h::MAP, errors::preprocessor::{ParserError, ParserReturn}, util::util::{get_h, open_file}};
use crate::{c::*,h,Preprocessor_Struct::Prerustc};


impl Prerustc{
pub fn new(fname :  &str) -> ParserReturn<Self>{
let tok_c = open_file(fname).unwrap();
let tok_h = open_file(&get_h(fname)).unwrap();
let (l_c,l_h) = (tok_c.len(),tok_h.len());
    Ok(Self{tok_c,tok_h,ret_tok_c:vec!["".to_string();l_c],ret_tok_h:vec!["".to_string();l_h],c_fname:fname.to_string()})
}


pub fn process(&mut self) ->  ParserReturn<()>{
let l = self.tok_h.len();
let mut i = 0;

while i < l {
if self.tok_h[i].contains("<") && self.tok_h[i].contains(">") && !self.tok_h[i].starts_with("<"){
    i += self.def_generic(i)?;
}else{
self.ret_tok_h.insert(i, self.tok_h[i].clone());
i += 1;
}
}

let l = self.tok_c.len();
let mut scope = 0;
let mut i = 0;



while i < self.tok_c.len() {
match &self.tok_c[i][..]{
"deferc" => {i += self.eval_Deferc(scope,i)?;},
"defer" => {i += self.eval_Defer(scope,i)?;},
"@Autowired" => {i += self.eval_Autowired(i)?;},
"{" => {
    self.ret_tok_c.insert(i,self.tok_c[i].clone());
    scope += 1;
    i += 1;
},
"}" => {
    self.ret_tok_c.insert(i,self.tok_c[i].clone());
    scope -=1;
    i +=1;
}

"?." => {i += self.eval_nullaccess(i)?;},
"??=" => {i += self.eval_nullcoalese(i)?;}
_ => {
    if self.tok_c[i].contains("deferc"){
        i += self.eval_Deferc(scope, i)?;
    }
    else if self.tok_c[i].contains("defer"){
        i += self.eval_Defer(scope, i)?;
    }else if !self.tok_c[i].starts_with("<") && (self.tok_c[i].contains("<") && self.tok_c[i].contains(">")){
        i += self.populate_generics(i)?;
    }
    else if self.tok_c[i].contains("@Autowired"){
        i += self.eval_Autowired(i)?;
    }else if self.tok_c[i].contains("?."){
        i += self.eval_nullaccess(i)?;
    }else if self.tok_c[i].contains("??="){
        i += self.eval_nullcoalese(i)?;
    }else{
        self.ret_tok_c.insert(i,self.tok_c[i].clone());
        i += 1;
    }
},
}

}

Ok(())
}







fn get_name(&self, z: &str) -> String {
z.rsplit(|c| c == ' ' || c == '*')
.next().unwrap_or("").to_string()
}

pub fn print(&self) {

for i in &self.ret_tok_c {
if i.is_empty(){continue;}
if i.contains(";"){
    println!(" {i} ");
}
    print!(" {i} ");
}
}






}



}
