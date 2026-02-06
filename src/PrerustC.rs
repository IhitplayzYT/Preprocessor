#[allow(non_camel_case_types,non_snake_case,non_upper_case_globals,unused_features,unused_imports,dead_code)]
pub mod Preprocess{
use crate::{h::MAP, errors::preprocessor::{ParserError, ParserReturn}, util::util::{get_h, open_file}};
use crate::{c::*,h,Preprocessor_Struct::Prerustc};


impl Prerustc{
pub fn new(fname :  &str) -> ParserReturn<Self>{
let tok_c = open_file(fname).unwrap();
let tok_h = open_file(&get_h(fname)).unwrap();
let (l_c,l_h) = (tok_c.len(),tok_h.len());
    Ok(Self{tok_c,tok_h,ret_tok_c:vec!["".to_string();l_c],ret_tok_h:vec!["".to_string();l_h]})
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


// NOTE  TILL HERE NO ERROR SO BASICALLY .h file PARSING IS DONE :)

while i < l {

match &self.tok_c[i][..]{
"defer" => {println!("defer");i += self.eval_Defer(scope,i)?;},
"deferc" => {println!("deferc");i += self.eval_Deferc(scope,i)?;},
"@Autowired" => {i += self.eval_Autowired(i)?;},
"{" => {
    println!("{{");
    self.ret_tok_c.insert(i,self.tok_c[i].clone());
    scope += 1;
    i += 1;
},
"}" => {
    println!("}}");
    self.ret_tok_c.insert(i,self.tok_c[i].clone());
    scope -=1;
    i +=1;
}

"?." => {println!("?.");i += self.eval_nullaccess(i)?;},
"??=" => {println!("??=");i += self.eval_nullcoalese(i)?;}
_ => {
    if self.tok_c[i].contains("deferc"){
        println!("defc");
        i += self.eval_Deferc(scope, i)?;
    }
    else if self.tok_c[i].contains("defer"){
        println!("def");
        i += self.eval_Defer(scope, i)?;
    }else if self.tok_c[i].contains("<") && self.tok_c[i].contains(">"){
        i += self.populate_generics(i)?;
    }
    else if self.tok_c[i].contains("@Autowired"){
        println!("auto");
        i += self.eval_Autowired(i)?;
    }else if self.tok_c[i].contains("?."){
        println!(".");
        i += self.eval_nullaccess(i)?;
    }else if self.tok_c[i].contains("??="){
        println!("??");
        i += self.eval_nullcoalese(i)?;
    }else if self.tok_c[i].contains("(") && self.tok_c[i].contains(")"){
        i += self.eval_cppinit(i)?;
    }else{
        println!("xxx");
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
