#[allow(unused_imports,non_snake_case,non_camel_case_types,dead_code)]
use crate::Preprocessor_Struct::Prerustc;
use crate::errors::preprocessor::*;

impl Prerustc{

pub fn eval_cppinit(&mut self,idx: usize) -> ParserReturn<usize>{
let mut f = false;
if !self.tok_c[idx].contains(";") {
f = true;
}
let (lb,rb) = (self.tok_c[idx].rfind("(").unwrap(),self.tok_c[idx].rfind(")").unwrap());
let val = self.tok_c[idx][(lb+1)..rb].to_string();
self.tok_c[idx].drain(lb..(rb+1));
let mut buff = self.tok_c[idx].to_string();
if !f {
    buff.pop();
}
self.ret_tok_c.insert(idx, buff + &format!("={val}")[..] + ";"+"/* PreRustC: CppInit */");
Ok(1 + (f as usize))    
}

pub fn eval_nullaccess(&mut self,idx:usize) -> ParserReturn<usize>{
let mut buff = "(( ".to_string();
let i = idx;
let tidx = self.tok_c[idx].find('=').unwrap_or(self.tok_c[idx].find(' ').unwrap_or(0));
buff += &self.ret_tok_c[idx][..tidx];
let mut f = false;
if self.tok_c[idx].ends_with(';'){
f = true;
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
if f {curr.pop();buff.pop();buff.pop();}
buff.pop();
buff.pop();
buff.pop();

buff += &format!(") ? {} : NULL){} /* PreRustC: Nullaccess */",curr,if f {";"} else {""})[..];
self.ret_tok_c.insert(i, buff);
Ok(1)
}


pub fn eval_nullcoalese(&mut self,idx:usize) -> ParserReturn<usize>{

let mut buff = "".to_string();
let mut i = idx;
while !self.tok_c[i].contains(";"){
    buff += &self.tok_c[i][..];
    i += 1;
}
let var;
buff += &self.tok_c[i][..];
let L = buff.find("??=").unwrap();
if L == 0{
var = self.tok_c[idx - 1].clone();
}else{
    var = buff[0..L].to_string();
}

let R = buff.find(";").unwrap();
let expr = buff[L..R].to_string(); 
// L+3+1 -> 3 is for moving over the ??= and 1 is for starting at the next char
let expr = &expr[4..][..];
buff = buff.replace("??=", &format!("= ( {var} ) ? {var} : {expr}")[..]);
self.ret_tok_c.insert(idx, buff+" /* PreRustC: Null Coalese */");
Ok(2)  
}


// TODO: FIX DEFERFIXME: Also look for early exit via return
// KILLER IDEA we find every return and also the scope end and instead of adding to the array we simply
// modify the token in tok_c to have the defer command run before them
pub fn eval_Defer(&mut self,scope: i32,i:usize) -> ParserReturn<usize>{
    let mut scope = scope;
    let z = scope;
    let l = self.tok_c.len();
    let k;
    let mut name ;
    if self.tok_c[i+1].ends_with(';'){
        name = self.tok_c[i+1].clone();
        name.pop();        
        k = i + 2;
    }else{
        k = i + 3;
        name = self.tok_c[i+2].clone();
        
    }
    for j in k..l{ 
                   
        if let Some(_) = self.tok_c[j].find("return"){
            self.tok_c[j] = format!("free( {} ); /* PreRustC: Defr */\n",name) + &self.tok_c[j];
        }
        if self.tok_c[j] == "{" {
            scope += 1;
        }
        if self.tok_c[j] == "}"{
            scope -= 1;
        }
        if scope < z{   
            self.tok_c[j] = format!("free( {} ); /* PreRustC: Defr */\n",name) + &self.tok_c[j];
            break;
        } 
    }
Ok(2)
}

pub fn eval_Deferc(&mut self,scope: i32,i:usize) -> ParserReturn<usize>{
    let mut scope = scope;
    let z = scope;
    let l = self.tok_c.len();
   let mut idx = i+1; 
   let mut buff = "".to_string();
    while !self.tok_c[idx].contains(";"){
        buff += &self.tok_c[idx][..];
        buff += " ";
        idx += 1;
    }
    buff += &self.tok_c[idx][..];
    for j in i..l{ 
        if let Some(_) = self.tok_c[j].find("return"){
            self.tok_c[j] = format!("{} /* PreRustC: DefrC */\n",buff) + &self.tok_c[j];
        }
        if self.tok_c[j] == "{" {
            scope += 1;
        }           
        if self.tok_c[j] == "}"{
            scope -= 1;
        }
        if scope < z{
            self.tok_c[j] = format!("{} /* PreRustC: DefrC */\n",buff) + &self.tok_c[j];
            break;
        } 
    }
Ok(idx-i+1)
}



// FIXME: Remove the // comment thingy if we use @Autowired
pub fn eval_Autowired(&mut self,start:usize) -> ParserReturn<usize> {

let mut i = start+1;
let mut buff = "".to_string();
while !self.tok_c[i].contains(";"){
buff += &self.tok_c[i][..];
if self.tok_c[i].contains("struct"){buff += " ";}
i += 1;
}
buff += &self.tok_c[i][..];
if !buff.ends_with(";"){
buff.push(';');
}

let ridx = buff.rfind("*").unwrap();
let col_idx = buff.rfind(";").unwrap();
let var_name = buff[(ridx+1)..col_idx].to_string();
let malloc_sz = buff[..ridx].to_string();
let var_type = buff[..(ridx+1)].to_string();
buff.pop();
self.ret_tok_c.insert(i, buff + 
    &format!(" = ({var_type} )malloc(sizeof({malloc_sz})); /* PreRustC: AutoWired */ \nif (!{var_name}){{ \n  free({var_name});\n  exit(-1); \n}}\n")[..]
);
Ok(i-start+1)
}

pub fn eval_foreach(&mut self,i:usize) -> ParserReturn<usize>{
    if i+2 >= self.tok_c.len() || self.tok_c[i+2] != "in" {
        return Err(ParserError::Foreach_err);
    }

let var = self.tok_c[i+1].clone();
let iterator = self.tok_c[i+3].clone();
let buff = format!("for (char* iter_var = {iterator};*iter_var;iter_var++){{\nchar {var} = *iter_var;\n");
self.ret_tok_c.insert(i,buff);
let mut jump = 0;
let idx = i;
while self.tok_c[idx+jump] != "{" || self.tok_c[idx+jump].ends_with("{") {jump +=1;}

Ok(jump+1)
} 


}