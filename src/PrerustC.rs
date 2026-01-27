pub mod Preprocess{
use std::fmt::format;

use crate::{util::util::{open_file,get_h},lib::preprocessor::{ParserReturn,ParserError}};

struct Prerustc{
pub tok_c: Vec<String>,
pub tok_h: Vec<String>,
pub ret_tok_c: Option<Vec<String>>,
pub ret_tok_h: Option<Vec<String>>,
}

impl Prerustc{
pub fn new(fname :  &str) -> ParserReturn<Self>{
let tok_c = open_file(fname).unwrap();
let tok_h = open_file(&get_h(fname)).unwrap();
    Ok(Self{tok_c,tok_h,ret_tok_c:None,ret_tok_h:None})
}



//  THere is riuntime modification of iterable when im iterating over it so FIXME:
pub fn process(&mut self) ->  ParserReturn<()>{
let l = self.tok_c.len();
let mut scope = 0;
for i in 0..l{
match &self.tok_c[i][..]{
"defer" => {
    self.eval_Defer(scope,i)?;
},
"@autowired" => {
self.eval_Autowired(i)?;
},
"{" => {
    scope += 1 
},
"}" => {
    scope -=1;
}
"?." => {
  self.eval_nullaccess(i)?;
},
"??=" => {
    self.eval_nullcoalese(i)?;
}
_ => {

/*  Make sure to also deal with those cases where the tokens may be part of another token */

},
}


    }

Ok(())
}

fn eval_nullaccess(&mut self,idx:usize) -> ParserReturn<()>{


Ok(())
}

fn eval_nullcoalese(&mut self,idx:usize) -> ParserReturn<()>{


Ok(())
}

fn eval_Defer(&mut self,scope: i32,i:usize) -> ParserReturn<()>{
    let mut scope = scope;
    let z = scope;
    let l = self.tok_c.len();
    let mut k = 0;
    let mut name = "".to_string();
    if self.tok_c[i+1].ends_with(';'){
        name = self.tok_c[i+1].clone();
        name.pop();        
        k = i + 2;
    }else{
        k = i + 3;
        name = self.tok_c[i+2].clone();
        
    }
    for j in k..l{ 
        if self.tok_c[j] == "{" {
            scope += 1;
        }
                
        if self.tok_c[j] == "}"{
            scope -= 1;
        }
        if scope < z{
            self.tok_c.insert(j, format!("free({}); // <PreRustC: Defer>\n",name));
            break;
        } 
    }
Ok(())
}


fn eval_Autowired(&mut self,i:usize) -> ParserReturn<()> {
let mut var_type = self.tok_c[i+1].trim().to_string();
let mut i = i+2;

'br : while !self.tok_c[i].contains(";"){
        if !self.tok_c[i].contains("*"){
            var_type += &self.tok_c[i][..];
            break;
        }else{
            let mut flag = false;
            for k in self.tok_c[i].chars(){
                match k {
                    '*' => {var_type.push(k);flag = true;},
                    ' ' => var_type.push(k),
                    _ => { if flag {
                         break 'br;
                        }
                    }
                }
            }
        }
    i += 1;
    }

let var_type = var_type.trim().to_string();
let mut malloc_sz = var_type.clone();
malloc_sz.pop(); 

while self.tok_c[i].contains(';') {
    i+=1;
}
if self.tok_c[i].len() == 1{
    i -= 1;
}
let mut var_name = self.get_name(&self.tok_c[i][..]).trim().to_string();
if var_name.ends_with(';') {var_name.pop();}
self.tok_c.insert(i, format!("({var_type} )malloc(sizeof({malloc_sz})); // <PreRustC: @AutoWired> \nif (!{var_name}) {{ free({var_name});exit(-1); }}"));
Ok(())
}



fn get_name(&self, z: &str) -> String {
z.rsplit(|c| c == ' ' || c == '*')
.next().unwrap_or("").to_string()
}





}



}