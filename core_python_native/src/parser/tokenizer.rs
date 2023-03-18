use crate::parser::python_core_parser::PythonCoreParser;
use crate::parser::symbols::*;



// Trait: Tokenizer methods for parser ////////////////////////////////////////////////////////////////////////////////

trait Tokenizer {
    fn is_reserved_keywords( start_pos: u32, text: &str ) -> Option<Symbols>;
    fn is_operator_or_delimiter( ch1: char, ch2: char, ch3: char, start_pos: u32  ) -> ( Option<Symbols>, u8 );
}


// Implementation of tokenizer for parser /////////////////////////////////////////////////////////////////////////////

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

    fn is_operator_or_delimiter( ch1: char, ch2: char, ch3: char, start_pos: u32  ) -> ( Option<Symbols>, u8 ) {
        match ( ch1, ch2, ch3 ) {
            ( '*', '*', '=' )   =>  ( Some( Symbols::PyPowerAssign      (start_pos, start_pos + 2 ) ), 3 ),
            ( '*', '*', _   )   =>  ( Some( Symbols::PyPower            (start_pos, start_pos + 1 ) ), 2 ),
            ( '*', '=', _   )   =>  ( Some( Symbols::PyMulAssign        (start_pos, start_pos + 1 ) ), 2 ),
            ( '*', _ , _    )   =>  ( Some( Symbols::PyMul              (start_pos, start_pos ) ), 1 ),
            ( '/', '/', '=' )   =>  ( Some( Symbols::PyFloorDivAssign   (start_pos, start_pos + 2 ) ), 3 ),
            ( '/', '/', _   )   =>  ( Some( Symbols::PyFloorDiv         (start_pos, start_pos + 1 ) ), 2 ),
            ( '/', '=', _   )   =>  ( Some( Symbols::PyDivAssign        (start_pos, start_pos + 1 ) ), 2 ),
            ( '/', _ , _    )   =>  ( Some( Symbols::PyDiv              (start_pos, start_pos ) ), 1 ),
            ( '<', '<', '=' )   =>  ( Some( Symbols::PyShiftLeftAssign  (start_pos, start_pos + 2 ) ), 3 ),
            ( '<', '<', _   )   =>  ( Some( Symbols::PyShiftLeft        (start_pos, start_pos + 1 ) ), 2 ),
            ( '<', '=', _   )   =>  ( Some( Symbols::PyLessEqual        (start_pos, start_pos + 1 ) ), 2 ),
            ( '<', _ , _    )   =>  ( Some( Symbols::PyLess             (start_pos, start_pos) ), 1 ),
            ( '>', '>', '=' )   =>  ( Some( Symbols::PyShiftRightAssign (start_pos, start_pos + 2 ) ), 3 ),
            ( '>', '>', _   )   =>  ( Some( Symbols::PyShiftRight       (start_pos, start_pos + 1 ) ), 2 ),
            ( '>', '=', _   )   =>  ( Some( Symbols::PyGreaterEqual     (start_pos, start_pos + 1 ) ), 2 ),
            ( '>', _ , _    )   =>  ( Some( Symbols::PyGreater          (start_pos, start_pos ) ), 1 ),
            ( '.', '.', '.' )   =>  ( Some( Symbols::PyEllipsis         (start_pos, start_pos + 2 ) ), 3 ),
            ( '.', _ , _    )   =>  ( Some( Symbols::PyDot              (start_pos, start_pos ) ), 1 ),
            ( '+', '=', _   )   =>  ( Some( Symbols::PyPlusAssign       (start_pos, start_pos + 1 ) ), 2 ),
            ( '+', _ , _    )   =>  ( Some( Symbols::PyPlus             (start_pos, start_pos ) ), 1 ),
            ( '-', '=', _   )   =>  ( Some( Symbols::PyMinusAssign      (start_pos, start_pos + 1 ) ), 2 ),
            ( '-', '>', _   )   =>  ( Some( Symbols::PyArrow            (start_pos, start_pos + 1 ) ), 2 ),
            ( '-', _ , _    )   =>  ( Some( Symbols::PyMinus            (start_pos, start_pos ) ), 1 ),
            ( '%', '=', _   )   =>  ( Some( Symbols::PyModuloAssign     (start_pos, start_pos + 1 ) ), 2 ),
            ( '%', _ , _    )   =>  ( Some( Symbols::PyModulo           (start_pos, start_pos ) ), 1 ),
            ( '@', '=', _   )   =>  ( Some( Symbols::PyMatricesAssign   (start_pos, start_pos + 1 ) ), 2 ),
            ( '@', _ , _    )   =>  ( Some( Symbols::PyMatrices         (start_pos, start_pos ) ), 1 ),
            ( ':', '=', _   )   =>  ( Some( Symbols::PyColonAssign      (start_pos, start_pos + 1 ) ), 2 ),
            ( ':', _ , _    )   =>  ( Some( Symbols::PyColon            (start_pos, start_pos ) ), 1 ),
            ( '&', '=', _   )   =>  ( Some( Symbols::PyBitwiseAndAssign (start_pos, start_pos + 1 ) ), 2 ),
            ( '&', _ , _    )   =>  ( Some( Symbols::PyBitwiseAnd       (start_pos, start_pos ) ), 1 ),
            ( '|', '=', _   )   =>  ( Some( Symbols::PyBitwiseOrAssign  (start_pos, start_pos + 1 ) ), 2 ),
            ( '|', _ , _    )   =>  ( Some( Symbols::PyBitwiseOr        (start_pos, start_pos ) ), 1 ),
            ( '^', '=', _   )   =>  ( Some( Symbols::PyBitwiseXorAssign (start_pos, start_pos + 1 ) ), 2 ),
            ( '^', _ , _    )   =>  ( Some( Symbols::PyBitwiseXor       (start_pos, start_pos ) ), 1 ),
            ( '=', '=', _   )   =>  ( Some( Symbols::PyEqual            (start_pos, start_pos + 1 ) ), 2 ),
            ( '=', _ , _    )   =>  ( Some( Symbols::PyAssign           (start_pos, start_pos ) ), 1 ),
            ( '!', '=', _   )   =>  ( Some( Symbols::PyNotEqual         (start_pos, start_pos+ 1 ) ), 2 ),
            ( '~', _ , _    )   =>  ( Some( Symbols::PyBitwiseInvert    (start_pos, start_pos ) ), 1 ),
            ( ';', _ , _    )   =>  ( Some( Symbols::PySemicolon        (start_pos, start_pos ) ), 1 ),
            ( ',', _ , _    )   =>  ( Some( Symbols::PyComma            (start_pos, start_pos ) ), 1 ),
            ( '{', _ , _    )   =>  ( Some( Symbols::PyLeftCurly        (start_pos, start_pos ) ), 1 ),
            ( '[', _ , _    )   =>  ( Some( Symbols::PyLeftBracket      (start_pos, start_pos ) ), 1 ),
            ( '(', _ , _    )   =>  ( Some( Symbols::PyLeftParen        (start_pos, start_pos ) ), 1 ),
            ( '}', _ , _    )   =>  ( Some( Symbols::PyRightCurly       (start_pos, start_pos ) ), 1 ),
            ( ']', _ , _    )   =>  ( Some( Symbols::PyRightBracket     (start_pos, start_pos ) ), 1 ),
            ( ')', _ , _    )   =>  ( Some( Symbols::PyRightParen       (start_pos, start_pos ) ), 1 ),
            _   =>  ( None, 0 )
        }
    }
}