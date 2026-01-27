pub mod preprocessor {

#[derive(Debug,Clone)]
pub enum ParserError{
Defer_err,
Nullaccess_err,
}

impl std::fmt::Display for ParserError{
fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
 write!(f,"{:?}:{:?} => {:?}\n",line!(),column!(),self)
}
}

impl std::error::Error for ParserError{    
}


pub type ParserReturn<T> = Result<T,ParserError>;
    


}