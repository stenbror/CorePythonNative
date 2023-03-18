
use crate::parser::symbols::*;

pub struct PythonCoreParser {
    pub(crate) symbol: Box<Result<Box<Symbols>, Box<String>>>
}


pub trait PythonParser {
    fn new() -> Self;
    fn advance(&mut self) -> ();
}


impl PythonParser for PythonCoreParser {

    fn new() -> Self {
        PythonCoreParser {
            symbol: Box::new( Err( Box::new("Token not advanced yet! ".to_string() ) ) ),
        }
    }

    fn advance(&mut self) -> () {

    }
}