#[allow(non_camel_case_types,non_snake_case,non_upper_case_globals,unused_features,unused_imports)]
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
while i < l {

match &self.tok_c[i][..]{
"defer" => {println!("defer");i += self.eval_Defer(scope,i)?;},
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
    if self.tok_c[i].contains("defer"){
        println!("def");
        i += self.eval_Defer(scope, i)?;
    }else if self.tok_c[i].contains("@Autowired"){
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


fn eval_cppinit(&mut self,idx: usize) -> ParserReturn<usize>{
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
self.ret_tok_c.insert(idx, buff + &format!("={val}")[..] + ";");
Ok(1 + (f as usize))    
}

// Might require a TT macro or regex


// Potential replace with x?.y?.z?.a   with x && x->y && x->y->z && x->y->z->a ? x->y->z->a:NULL


fn eval_nullaccess(&mut self,idx:usize) -> ParserReturn<usize>{
let mut buff = "".to_string();
let mut i = idx;
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

buff += &format!(" ? {} : NULL; // <PreRustC: Nullaccess>",curr)[..];
self.ret_tok_c.insert(i, buff);
Ok(i + (!f as usize) + 1 - idx)
}

// We can return shift so that we can accordingly modify the iteration;
fn eval_nullcoalese(&mut self,idx:usize) -> ParserReturn<usize>{
let mut buff = "".to_string();
let mut i = idx;
while !self.tok_c[i].contains(";"){
    buff += &self.tok_c[i][..];
    i += 1;
}
buff += &self.tok_c[i][..];
let L = buff.find("??=").unwrap();
let R = buff.find(";").unwrap();
let expr = buff[L..R].to_string(); 
// L+3+1 -> 3 is for moving over the ??= and 1 is for starting at the next char
let expr = &expr[4..][..];
buff = buff.replace("??=", &format!("=(!({expr})) ? NULL : ")[..]);
self.ret_tok_c.insert(idx, buff+" // <PreRustC: Null Coalese>\n");
Ok(i-idx+1)  
}

fn eval_Defer(&mut self,scope: i32,i:usize) -> ParserReturn<usize>{
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
Ok(1)
}

// FIXME: Remove the // comment thingy if we use @Autowired
fn eval_Autowired(&mut self,start:usize) -> ParserReturn<usize> {

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
    &format!(" = ({var_type} )malloc(sizeof({malloc_sz})); // <PreRustC: @AutoWired> \nif (!{var_name}){{ \n  free({var_name});\n  exit(-1); \n}}\n")[..]
);
Ok(i-start+1)
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
