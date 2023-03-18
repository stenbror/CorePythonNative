use crate::parser::python_core_parser::PythonCoreParser;
use crate::parser::symbols::*;

trait Tokenizer {
    fn is_reserved_keywords(start_pos: u32, text: &str) -> Option<Symbols>;
}

impl Tokenizer for PythonCoreParser {

    fn is_reserved_keywords(start_pos: u32, text: &str) -> Option<Symbols> {
        match text {
            "False"         => Some( Symbols::PyFalse       ( start_pos, start_pos + 4 ) ),
            "None"          => Some( Symbols::PyNone        ( start_pos, start_pos + 3 ) ),
            "True"          => Some( Symbols::PyFalse       ( start_pos, start_pos + 3 ) ),
            "and"           => Some( Symbols::PyAnd         ( start_pos, start_pos + 2 ) ),
            "as"            => Some( Symbols::PyAs          ( start_pos, start_pos + 1 ) ),
            "assert"        => Some( Symbols::PyAssert      ( start_pos, start_pos + 5 ) ),
            "async"         => Some( Symbols::PyAsync       ( start_pos, start_pos + 4 ) ),
            "await"         => Some( Symbols::PyAwait       ( start_pos, start_pos + 4 ) ),
            "break"         => Some( Symbols::PyBreak       ( start_pos, start_pos + 4 ) ),
            "class"         => Some( Symbols::PyClass       ( start_pos, start_pos + 4 ) ),
            "continue"      => Some( Symbols::PyContinue    ( start_pos, start_pos + 7 ) ),
            "def"           => Some( Symbols::PyDef         ( start_pos, start_pos + 2 ) ),
            "del"           => Some( Symbols::PyDel         ( start_pos, start_pos + 2 ) ),
            "elif"          => Some( Symbols::PyElif        ( start_pos, start_pos + 3 ) ),
            "else"          => Some( Symbols::PyElse        ( start_pos, start_pos + 3 ) ),
            "except"        => Some( Symbols::PyExcept      ( start_pos, start_pos + 5 ) ),
            "finally"        => Some( Symbols::PyFinally     ( start_pos, start_pos + 5 ) ),
            "for"           => Some( Symbols::PyFor         ( start_pos, start_pos + 2 ) ),
            "from"          => Some( Symbols::PyFrom        ( start_pos, start_pos + 3 ) ),
            "global"        => Some( Symbols::PyGlobal      ( start_pos, start_pos + 5 ) ),
            "if"            => Some( Symbols::PyIf          ( start_pos, start_pos + 1 ) ),
            "import"        => Some( Symbols::PyImport      ( start_pos, start_pos + 5 ) ),
            "in"            => Some( Symbols::PyIn          ( start_pos, start_pos + 1 ) ),
            "is"            => Some( Symbols::PyIs          ( start_pos, start_pos + 1 ) ),
            "lambda"        => Some( Symbols::PyLambda      ( start_pos, start_pos + 5 ) ),
            "nonlocal"      => Some( Symbols::PyNonlocal    ( start_pos, start_pos + 7 ) ),
            "not"           => Some( Symbols::PyNot         ( start_pos, start_pos + 2 ) ),
            "or"            => Some( Symbols::PyOr          ( start_pos, start_pos + 1 ) ),
            "pass"          => Some( Symbols::PyPass        ( start_pos, start_pos + 3 ) ),
            "raise"         => Some( Symbols::PyRaise       ( start_pos, start_pos + 4 ) ),
            "return"        => Some( Symbols::PyReturn      ( start_pos, start_pos + 5 ) ),
            "try"           => Some( Symbols::PyTry         ( start_pos, start_pos + 2 ) ),
            "while"         => Some( Symbols::PyWhile       ( start_pos, start_pos + 4 ) ),
            "with"          => Some( Symbols::PyWith        ( start_pos, start_pos + 3 ) ),
            "yield"         => Some( Symbols::PyDef         ( start_pos, start_pos + 4 ) ),
            _               => None
        }
    }
}