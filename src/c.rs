use std::usize;

#[allow(unused_imports,non_snake_case,non_camel_case_types,dead_code)]
use crate::Preprocessor_Struct::Prerustc;
use crate::errors::preprocessor::*;
use crate::util::util;

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
let mut iterator = self.tok_c[i+3].clone();
if iterator.ends_with("{"){iterator.pop().unwrap();}

let buff = format!("for (char* iter_var = {iterator};*iter_var;iter_var++){{\nchar {var} = *iter_var;\n");
self.ret_tok_c.insert(i,buff);
Ok(4+ if !self.tok_c[i+3].ends_with("{") { 1 } else { 0 })
} 

pub fn eval_multi_assign(&mut self,i:usize)-> ParserReturn<usize>{
    let mut idx = i;
    let count = util::freq(&self.tok_c[i],b',');
    if count > 1 && (i > 1 && !self.tok_c[i-1].contains("(")) && !self.tok_c[i].contains("("){ 
        let mut buff = "".to_string();
        let mut varibles : Vec<String> = self.tok_c[i].split(",").map(|x| {x.to_string()}).collect();
        
        while !self.tok_c[idx].contains("=") && !self.tok_c[idx].contains(";"){
            let (L,R) = (self.tok_c[idx].find(",").unwrap_or(usize::MAX),self.tok_c[idx].rfind(",").unwrap_or(usize::MAX));
            if L == R && L != usize::MAX {
                if L == 0 {
                    varibles.push(self.tok_c[idx][(L+1)..].to_string());
                }else{
                    varibles.push(self.tok_c[idx][..R].to_string());
                }                
            }
            idx += 1;
        }
        let l = varibles.len();

        if varibles[l-1].ends_with(";") {
            let csv = varibles.into_iter().reduce(|acc, s| acc + "," + &s).unwrap_or_default();
            println!("{}",csv);
            self.ret_tok_c[i-1].push_str(&csv[..]);
            return Ok(idx - i+ 1);
        }
        if varibles[l-1].ends_with("=") {
            varibles[l-1].pop().unwrap();
        }
            idx += 1;

        let mut vals:Vec<String> = self.tok_c[idx].split(",").map(|s| {s.to_string()}).collect();
        
        while !self.tok_c[idx].contains(";") {
            let (L,R) = (self.tok_c[idx].find(",").unwrap_or(usize::MAX),self.tok_c[idx].rfind(",").unwrap_or(usize::MAX));
            if L == R && L != usize::MAX {
                if L == 0 {
                    vals.push(self.tok_c[idx][(L+1)..].to_string());
                }else{
                    vals.push(self.tok_c[idx][..R].to_string());
                }                
            }
            idx += 1;
        }
        let l = vals.len();
        if vals[l-1].ends_with(";") {
            vals[l-1].pop().unwrap();
        }
            idx += 1;

        if vals.len() != varibles.len() {
            return Err(ParserError::Customer_err("Invalid number of LHS & RHS".to_string()));
        }

        for (var,asgn) in varibles.iter().zip(vals){
            buff += &format!(" {var}={asgn},");
        }

        buff.pop();
        buff += ";";
        self.ret_tok_c[i-1].push_str(&buff[..]);

    }else{
        self.ret_tok_c.insert(i, self.tok_c[i].clone());
        return Ok(1);
    }
        

    Ok(idx-i)
    }

}




