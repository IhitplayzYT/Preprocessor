#[warn(non_camel_case_types,non_snake_case,non_upper_case_globals)]
pub mod Preprocess{
use crate::{util::util::{open_file,get_h},lib::preprocessor::{ParserReturn,ParserError}};

pub struct Prerustc{
pub tok_c: Vec<String>,
pub tok_h: Vec<String>,
pub ret_tok_c: Vec<String>,
pub ret_tok_h: Vec<String>,
}

impl Prerustc{
pub fn new(fname :  &str) -> ParserReturn<Self>{
let tok_c = open_file(fname).unwrap();
let tok_h = open_file(&get_h(fname)).unwrap();
let (l_c,l_h) = (tok_c.len(),tok_h.len());
    Ok(Self{tok_c,tok_h,ret_tok_c:vec!["".to_string();l_c],ret_tok_h:vec!["".to_string();l_h]})
}


// FIXME: Check if in all the fxn we simply assigning to the idx = "" instead of pushing
pub fn process(&mut self) ->  ParserReturn<()>{
let l = self.tok_c.len();
let mut scope = 0;
let mut i = 0;
while i < l{
self.print();
match &self.tok_c[i][..]{
"defer" => {
    i += self.eval_Defer(scope,i)?;
},
"@Autowired" => {
    i += self.eval_Autowired(i)?;
},
"{" => {
    self.ret_tok_c.insert(i,self.tok_c[i].clone());
    scope += 1 
},
"}" => {
    self.ret_tok_c.insert(i,self.tok_c[i].clone());
    scope -=1;
}
"?." => {
    i += self.eval_nullaccess(i)?;
},
"??=" => {
    i += self.eval_nullcoalese(i)?;
}
_ => {
    if self.tok_c[i].contains("defer"){
        i += self.eval_Defer(scope, i)?;
    }else if self.tok_c[i].contains("@Autowired"){
        i += self.eval_Autowired(i)?;
    }else if self.tok_c[i].contains("?."){
        i += self.eval_nullaccess(i)?;
    }else if self.tok_c[i].contains("??="){
        i += self.eval_nullcoalese(i)?;
    }else{
        i += 1;
        self.ret_tok_c.insert(i,self.tok_c[i].clone());
    }

/*  Make sure to also deal with those cases where the tokens may be part of another token */

},
}

}

Ok(())
}
// Might require a TT macro or regex


// Potential replace with x?.y?.z?.a   with x && x->y && x->y->z && x->y->z->a ? x->y->z->a:NULL

fn eval_nullaccess(&mut self,idx:usize) -> ParserReturn<usize>{
let mut buff = "".to_string();
let mut tidx = self.tok_c[idx].find("?.").unwrap();
let mut i = idx;
while tidx >= 0 && tidx != usize::MAX && self.tok_c[tidx] != " " || self.tok_c[tidx] != "=" {
    tidx -= 1;
}
tidx += 1;
if !self.tok_c[idx].ends_with(";"){
    self.tok_c[idx] += ";";
    i+=1;
}
let mut curr = "".to_string();
let inter:Vec<&str> = self.tok_c[idx][tidx..].split("?.").collect();
for z in inter{
    curr += z;
    buff.push_str(&curr[..]);
    buff.push_str(" && ");
    curr += "->"
}
curr.pop();
curr.pop();
buff.pop();
buff.pop();
buff.pop();
buff += &format!("? {} : NULL; // <PreRustC: Nullaccess>",buff)[..];
self.ret_tok_c.insert(i, buff);
Ok(i)
}

// We can return shift so that we can accordingly modify the iteration;
fn eval_nullcoalese(&mut self,idx:usize) -> ParserReturn<usize>{
let mut buff = "".to_string();
let mut i = idx;
while !self.tok_c[i].contains(";"){
    i += 1;
    buff += &self.tok_c[i][..];
}

let L = buff.find("??=").unwrap();
let R = buff.find(";").unwrap();
let expr = buff[(L+4)..R].to_string(); // L+3+1 -> 3 is for moving over the ??= and 1 is for starting at the next char
buff = buff.replace("??=", &format!("=(!{expr})?NULL:")[..]);
self.ret_tok_c.insert(idx, buff);
Ok(i)  
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


fn eval_Autowired(&mut self,start:usize) -> ParserReturn<usize> {
let mut i = start;
let mut buff = "".to_string();
while !self.tok_c[i].contains(";"){
if self.tok_c[i].contains("struct"){
buff += " ";
}
i += 1;
}
if !buff.ends_with(";"){
buff.push(';');
}
let ridx = buff.rfind("*").unwrap();
let var_name = buff[(ridx+1)..].to_string();
let malloc_sz = buff[..ridx].to_string();
let var_type = buff[..(ridx+1)].to_string();
self.ret_tok_c.insert(i,
format!(" = ({var_type} )malloc(sizeof({malloc_sz})); // <PreRustC: @AutoWired> 
\nif (!{var_name})\n
\t{{ free({var_name});exit(-1); }}"
));
buff.pop();
self.ret_tok_c.insert(i, buff);
    Ok(i)
}

fn get_name(&self, z: &str) -> String {
z.rsplit(|c| c == ' ' || c == '*')
.next().unwrap_or("").to_string()
}

pub fn print(&self) {

for i in &self.ret_tok_c {
if i.contains(";"){
    println!(" {i} ");
}
    print!(" {i} ");
}
}






}



}
