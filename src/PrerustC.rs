pub mod Preprocess{
use std::fmt::format;

use crate::{util::util::{open_file,get_h},lib::preprocessor::{ParserReturn,ParserError}};

struct Prerustc{
pub tok_c: Vec<String>,
pub tok_h: Vec<String>,
pub ret_tok_c: Vec<String>,
pub ret_tok_h: Vec<String>,
}

impl Prerustc{
pub fn new(fname :  &str) -> ParserReturn<Self>{
let tok_c = open_file(fname).unwrap();
let tok_h = open_file(&get_h(fname)).unwrap();
let (l_c,l_h) = (tok_c.len(),tok_h.len()));
    Ok(Self{tok_c,tok_h,ret_tok_c:vec!["".to_string();l_c],ret_tok_h:vec!["".to_string();l_h]})
}



//  There is riuntime modification of iterable when im iterating over it best modify the code to work with ret_arr FIXME:
// There may also be a  chance that the tokens we searching for are inside a token FIXME: 
// Also accomodate the shift FIXME:
pub fn process(&mut self) ->  ParserReturn<()>{
let l = self.tok_c.len();
let mut scope = 0;
let mut i = 0;
while i < l{
match &self.tok_c[i][..]{
"defer" => {
    self.ret_tok_c.push(self.tok_c[i].clone());
    self.eval_Defer(scope,i)?;
},
"@autowired" => {
self.ret_tok_c.push(self.tok_c[i].clone());
self.eval_Autowired(i)?;
},
"{" => {
    self.ret_tok_c.push(self.tok_c[i].clone());
    scope += 1 
},
"}" => {
    self.ret_tok_c.push(self.tok_c[i].clone());
    scope -=1;
}
"?." => {
    self.ret_tok_c.push(self.tok_c[i].clone());
    let ret = self.eval_nullaccess(i)?;
},
"??=" => {
    self.ret_tok_c.push(self.tok_c[i].clone());
    self.eval_nullcoalese(i)?;
}
_ => {

    self.ret_tok_c.push(self.tok_c[i].clone());
/*  Make sure to also deal with those cases where the tokens may be part of another token */

},
}

i += 1;
}

Ok(())
}
// Might require a TT macro or regex
fn eval_nullaccess(&mut self,idx:usize) -> ParserReturn<()>{


Ok(())
}

// We can return shift so that we can accordingly modify the iteration;
fn eval_nullcoalese(&mut self,idx:usize) -> ParserReturn<usize>{
let mut buff = "".to_string();
let mut i = idx - 1;
while !self.tok_c[i].contains(";"){
    i += 1;
    buff += &self.tok_c[i][..];
}

let L = buff.find("??=").unwrap();
let R = buff.find(";").unwrap();
let expr = buff[(L+1)..R].to_string();
buff = buff.replace("??=", &format!("=(!{expr})?NULL:")[..]);
self.ret_tok_c.insert(idx, buff);
Ok()  //TODO:
}

fn eval_Defer(&mut self,scope: i32,i:usize) -> ParserReturn<usize>{
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
            self.ret_tok_c.insert(j, format!("free({}); // <PreRustC: Defer>\n",name));
            break;
        } 
    }
Ok(0)
}


/// Try doing a ctrl+c + ctrl+v method like example struct sample ** txt; 
/// we can replace (;,"with our gibberish") and to get name and type it be easy by simply find rindex of * and hence splicing
/// Be on lookpout for the buffer since here structsample is inavlid so make sure if struct we add " "
/// 

fn eval_Autowired(&mut self,start:usize) -> ParserReturn<usize> {
let mut var_type = self.tok_c[start+1].trim().to_string();
if &var_type[..] == "struct"{
    var_type += " ";
}
let mut i = start+1;

'br : while !self.tok_c[i].contains(";"){
        i += 1;
        if !self.tok_c[i].contains("*"){
            var_type += " ";
            var_type += &self.tok_c[i][..];
        }else{
            for k in self.tok_c[i].chars(){
                match k {
                    '*' => {var_type.push(k);},
                    ' ' => var_type.push(k),
                    _ => {
                         break 'br;
                    }
                }
            }
        }
    }
//  i is pointing to ; or var;

let var_type = var_type.trim().to_string();
let mut malloc_sz = var_type.clone();
malloc_sz.pop(); // Since size has one less '*'
while self.tok_c[i].contains(';') {
    i+=1;
}

let mut var_name;
if self.tok_c[i].len() == 1 && self.tok_c[i] == ";" {
    i -= 1;
}
    var_name = self.get_name(&self.tok_c[i][..]).trim().to_string();
    if var_name.ends_with(';') {var_name = var_name.replace(";","");}
    if var_name.starts_with("**") {var_name = var_name.replace("**","");}
    if var_name.starts_with("*") {var_name = var_name.replace("*","");}

/*   Accordingly modify tokens in Tok_c */

if self.tok_c[i] == ";"{
self.tok_c.remove(i);
}
else if self.tok_c[i].ends_with(";"){
    self.tok_c[i].pop();
}

self.tok_c.insert(i, format!("=({var_type} )malloc(sizeof({malloc_sz})); // <PreRustC: @AutoWired> \nif (!{var_name}) {{ free({var_name});exit(-1); }}"));
Ok(())
}



fn get_name(&self, z: &str) -> String {
z.rsplit(|c| c == ' ' || c == '*')
.next().unwrap_or("").to_string()
}





}



}