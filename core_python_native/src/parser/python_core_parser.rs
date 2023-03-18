
use crate::parser::symbols::*;


pub struct PythonCoreParser {
    pub(crate) symbol: Box<Result<Box<Symbols>, Box<String>>>

}


pub trait PythonParser {
    fn new() -> Self;
    fn advance( &mut self ) -> ();
    fn symbol_position( &mut self ) -> u32;
    fn current_position( &mut self) -> u32;
}


impl PythonParser for PythonCoreParser {

    fn new() -> Self {
        PythonCoreParser {
            symbol: Box::new( Err( Box::new("Token not advanced yet! ".to_string() ) ) ),
        }
    }

    fn advance(&mut self) -> () {
        self.symbol = Box::new( Ok( Box::new( Symbols::PyEllipsis( 0, 2 ) ) ) )
    }

    fn symbol_position( &mut self ) -> u32 {
        0
    }

    fn current_position( &mut self) -> u32 {
        0
    }
}