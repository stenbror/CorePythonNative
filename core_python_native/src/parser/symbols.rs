
#[derive(Clone, PartialEq, Debug)]
pub enum Symbols
{
    Empty,
    PyFalse             ( u32, u32 ),
    PyNone              ( u32, u32 ),
    PyTrue              ( u32, u32 ),
    PyAnd               ( u32, u32 ),
    PyAs                ( u32, u32 ),
    PyAssert            ( u32, u32 ),
    PyAsync             ( u32, u32 ),
    PyAwait             ( u32, u32 ),
    PyBreak             ( u32, u32 ),
    PyClass             ( u32, u32 ),
    PyContinue          ( u32, u32 ),
    PyDef               ( u32, u32 ),
    PyDel               ( u32, u32 ),
    PyElif              ( u32, u32 ),
    PyElse              ( u32, u32 ),
    PyExcept            ( u32, u32 ),
    PyFinally           ( u32, u32 ),
    PyFor               ( u32, u32 ),
    PyFrom              ( u32, u32 ),
    PyGlobal            ( u32, u32 ),
    PyIf                ( u32, u32 ),
    PyImport            ( u32, u32 ),
    PyIn                ( u32, u32 ),
    PyIs                ( u32, u32 ),
    PyLambda            ( u32, u32 ),
    PyNonlocal          ( u32, u32 ),
    PyNot               ( u32, u32 ),
    PyOr                ( u32, u32 ),
    PyPass              ( u32, u32 ),
    PyRaise             ( u32, u32 ),
    PyReturn            ( u32, u32 ),
    PyTry               ( u32, u32 ),
    PyWhile             ( u32, u32 ),
    PyWith              ( u32, u32 ),
    PyYield             ( u32, u32 ),
    PyPowerAssign       ( u32, u32 ),
    PyPower             ( u32, u32 ),
    PyMulAssign         ( u32, u32 ),
    PyMul               ( u32, u32 ),
    PyFloorDivAssign    ( u32, u32 ),
    PyFloorDiv          ( u32, u32 ),
    PyDivAssign         ( u32, u32 ),
    PyDiv               ( u32, u32 ),
    PyShiftLeftAssign   ( u32, u32 ),
    PyShiftLeft         ( u32, u32 ),
    PyShiftRightAssign  ( u32, u32 ),
    PyShiftRight        ( u32, u32 ),
    PyLess              ( u32, u32 ),
    PyLessEqual         ( u32, u32 ),
    PyEqual             ( u32, u32 ),
    PyGreaterEqual      ( u32, u32 ),
    PyGreater           ( u32, u32 ),
    PyEllipsis          ( u32, u32 ),
    PyNotEqual          ( u32, u32 ),
    PyDot               ( u32, u32 ),
    PyPlusAssign        ( u32, u32 ),
    PyPlus              ( u32, u32 ),
    PyMinusAssign       ( u32, u32 ),
    PyArrow             ( u32, u32 ),
    PyMinus             ( u32, u32 ),
    PyModuloAssign      ( u32, u32 ),
    PyModulo            ( u32, u32 ),
    PyMatricesAssign    ( u32, u32 ),
    PyMatrices          ( u32, u32 ),
    PyColonAssign       ( u32, u32 ),
    PyColon             ( u32, u32 ),
    PyBitwiseAndAssign  ( u32, u32 ),
    PyBitwiseAnd        ( u32, u32 ),
    PyBitwiseXorAssign  ( u32, u32 ),
    PyBitwiseXor        ( u32, u32 ),
    PyBitwiseOrAssign   ( u32, u32 ),
    PyBitwiseOr         ( u32, u32 ),
    PyBitwiseInvert     ( u32, u32 ),
    PySemicolon         ( u32, u32 ),
    PyComma             ( u32, u32 ),
    PyAssign            ( u32, u32 ),
    PyLeftParen         ( u32, u32 ),
    PyLeftBracket       ( u32, u32 ),
    PyLeftCurly         ( u32, u32 ),
    PyRightParen        ( u32, u32 ),
    PyRightBracket      ( u32, u32 ),
    PyRightCurly        ( u32, u32 ),
    PyName              ( u32, u32, Box<String> ),
    PyNumber            ( u32, u32, Box<String> ),
    PyString            ( u32, u32, Box<String> ),
    TypeComment         ( u32, u32, Box<String> ),
    Newline             ( u32, u32 ),
    Indent,
    Dedent,
    EOF                 ( u32 )
}