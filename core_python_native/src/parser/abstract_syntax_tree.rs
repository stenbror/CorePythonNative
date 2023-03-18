
use crate::parser::symbols::*;

#[derive(Clone, PartialEq, Debug)]
pub enum AbstractSyntaxNodes {
    Empty,
    False                   ( u32, u32, Box<Symbols> ),
    None                    ( u32, u32, Box<Symbols> ),
    True                    ( u32, u32, Box<Symbols> ),
    Ellipsis                ( u32, u32, Box<Symbols> ),
    Name                    ( u32, u32, Box<Symbols> ),
    Number                  ( u32, u32, Box<Symbols> ),
    String                  ( u32, u32, Box<[ Box<Symbols> ]>),
    AtomExpr                ( u32, u32, Option<Box<Symbols>>, Box<AbstractSyntaxNodes>, Option<Box<AbstractSyntaxNodes>> ),
    Power                   ( u32, u32, Box<AbstractSyntaxNodes>, Box<Symbols>, Box<AbstractSyntaxNodes> ),
    UnaryPlus               ( u32, u32, Box<Symbols>, Box<AbstractSyntaxNodes> ),
    UnaryMinus              ( u32, u32, Box<Symbols>, Box<AbstractSyntaxNodes> ),
    BitwiseInvert           ( u32, u32, Box<Symbols>, Box<AbstractSyntaxNodes> ),
    Mul                     ( u32, u32, Box<AbstractSyntaxNodes>, Box<Symbols>, Box<AbstractSyntaxNodes> ),
    Div                     ( u32, u32, Box<AbstractSyntaxNodes>, Box<Symbols>, Box<AbstractSyntaxNodes> ),
    FloorDiv                ( u32, u32, Box<AbstractSyntaxNodes>, Box<Symbols>, Box<AbstractSyntaxNodes> ),
    Modulo                  ( u32, u32, Box<AbstractSyntaxNodes>, Box<Symbols>, Box<AbstractSyntaxNodes> ),
    Matrices                ( u32, u32, Box<AbstractSyntaxNodes>, Box<Symbols>, Box<AbstractSyntaxNodes> ),
    Plus                    ( u32, u32, Box<AbstractSyntaxNodes>, Box<Symbols>, Box<AbstractSyntaxNodes> ),
    Minus                   ( u32, u32, Box<AbstractSyntaxNodes>, Box<Symbols>, Box<AbstractSyntaxNodes> ),
    ShiftLeft               ( u32, u32, Box<AbstractSyntaxNodes>, Box<Symbols>, Box<AbstractSyntaxNodes> ),
    ShiftRight              ( u32, u32, Box<AbstractSyntaxNodes>, Box<Symbols>, Box<AbstractSyntaxNodes> ),
    BitwiseAnd              ( u32, u32, Box<AbstractSyntaxNodes>, Box<Symbols>, Box<AbstractSyntaxNodes> ),
    BitwiseXor              ( u32, u32, Box<AbstractSyntaxNodes>, Box<Symbols>, Box<AbstractSyntaxNodes> ),
    BitwiseOr               ( u32, u32, Box<AbstractSyntaxNodes>, Box<Symbols>, Box<AbstractSyntaxNodes> ),
    StarExpr                ( u32, u32, Box<Symbols>, Box<AbstractSyntaxNodes> ),
    Less                    ( u32, u32, Box<AbstractSyntaxNodes>, Box<Symbols>, Box<AbstractSyntaxNodes> ),
    LessEqual               ( u32, u32, Box<AbstractSyntaxNodes>, Box<Symbols>, Box<AbstractSyntaxNodes> ),
    Equal                   ( u32, u32, Box<AbstractSyntaxNodes>, Box<Symbols>, Box<AbstractSyntaxNodes> ),
    GreaterEqual            ( u32, u32, Box<AbstractSyntaxNodes>, Box<Symbols>, Box<AbstractSyntaxNodes> ),
    Greater                 ( u32, u32, Box<AbstractSyntaxNodes>, Box<Symbols>, Box<AbstractSyntaxNodes> ),
    NotEqual                ( u32, u32, Box<AbstractSyntaxNodes>, Box<Symbols>, Box<AbstractSyntaxNodes> ),
    In                      ( u32, u32, Box<AbstractSyntaxNodes>, Box<Symbols>, Box<AbstractSyntaxNodes> ),
    Is                      ( u32, u32, Box<AbstractSyntaxNodes>, Box<Symbols>, Box<AbstractSyntaxNodes> ),
    NotIn                   ( u32, u32, Box<AbstractSyntaxNodes>, Box<Symbols>, Box<AbstractSyntaxNodes> ),
    IsNot                   ( u32, u32, Box<AbstractSyntaxNodes>, Box<Symbols>, Box<AbstractSyntaxNodes> ),
    NotTest                 ( u32, u32, Box<Symbols>, Box<AbstractSyntaxNodes> ),
    AndTest                 ( u32, u32, Box<AbstractSyntaxNodes>, Box<Symbols>, Box<AbstractSyntaxNodes> ),
    OrTest                  ( u32, u32, Box<AbstractSyntaxNodes>, Box<Symbols>, Box<AbstractSyntaxNodes> ),
    Lambda                  ( u32, u32, Box<Symbols>, Option<Box<AbstractSyntaxNodes>>, Box<Symbols>, Box<AbstractSyntaxNodes> ),
    Test                    ( u32, u32, Box<AbstractSyntaxNodes>, Box<Symbols>, Box<AbstractSyntaxNodes>, Box<Symbols>, Box<AbstractSyntaxNodes> ),
    NamedExpr               ( u32, u32, Box<AbstractSyntaxNodes>, Box<Symbols>, Box<AbstractSyntaxNodes> ),
    YieldExpr               ( u32, u32, Box<Symbols>, Box<AbstractSyntaxNodes> ),
    YieldFrom               ( u32, u32, Box<Symbols>, Box<Symbols>, Box<AbstractSyntaxNodes>),
    TestListStarExpr        ( u32, u32, Box<[Box<AbstractSyntaxNodes>]>, Box<[Box<Symbols>]> ),
    VarArgsList             ( u32, u32, Option<(Box<Symbols>, Box<AbstractSyntaxNodes>)>, Option<(Box<Symbols>, Box<AbstractSyntaxNodes>)>, Box<[Box<AbstractSyntaxNodes>]>, Box<[Box<Symbols>]>  ),
    VFPDefAssign            ( u32, u32, Box<AbstractSyntaxNodes>, Box<Symbols>, Box<AbstractSyntaxNodes> ),
    TestList                ( u32, u32, Box<[Box<AbstractSyntaxNodes>]>, Box<[Box<Symbols>]> ),
    ExprList                ( u32, u32, Box<[Box<AbstractSyntaxNodes>]>, Box<[Box<Symbols>]> ),
    SubscriptList           ( u32, u32, Box<[Box<AbstractSyntaxNodes>]>, Box<[Box<Symbols>]> ),
    Subscript               ( u32, u32, Option<Box<AbstractSyntaxNodes>>, Option<Box<Symbols>>, Option<Box<AbstractSyntaxNodes>>, Option<Box<Symbols>>, Option<Box<AbstractSyntaxNodes>>  ),
    CompSyncFor             ( u32, u32, Box<Symbols>, Box<AbstractSyntaxNodes>, Box<Symbols>, Box<AbstractSyntaxNodes>, Box<AbstractSyntaxNodes> ),
    CompFor                 ( u32, u32, Box<Symbols>, Box<AbstractSyntaxNodes> ),
    CompIf                  ( u32, u32, Box<Symbols>, Box<AbstractSyntaxNodes>, Box<AbstractSyntaxNodes> ),
    DictionaryContainer     ( u32, u32, Box<[Box<AbstractSyntaxNodes>]>, Box<[Box<Symbols>]> ),
    SetContainer            ( u32, u32, Box<[Box<AbstractSyntaxNodes>]>, Box<[Box<Symbols>]> ),
    DotName                 ( u32, u32, Box<Symbols>, Box<Symbols> ),
    CallExpression          ( u32, u32, Box<Symbols>, Option<Box<AbstractSyntaxNodes>>, Box<Symbols>),
    IndexExpression         ( u32, u32, Box<Symbols>, Option<Box<AbstractSyntaxNodes>>, Box<Symbols>),
    Tuple                   ( u32, u32, Box<Symbols>, Option<Box<AbstractSyntaxNodes>>, Box<Symbols> ),
    List                    ( u32, u32, Box<Symbols>, Option<Box<AbstractSyntaxNodes>>, Box<Symbols> ),
    PowerKey                ( u32, u32, Box<Symbols>, Box<AbstractSyntaxNodes> ),
    DictionaryEntry         ( u32, u32, Box<AbstractSyntaxNodes>, Box<Symbols>, Box<AbstractSyntaxNodes> ),
    Dictionary              ( u32, u32, Box<Symbols>, Option<Box<AbstractSyntaxNodes>>, Box<Symbols> ),
    Set                     ( u32, u32, Box<Symbols>, Option<Box<AbstractSyntaxNodes>>, Box<Symbols> ),
    ArgumentList            ( u32, u32, Box<[Box<AbstractSyntaxNodes>]>, Box<[Box<Symbols>]> ),
    Argument                ( u32, u32, Option<Box<AbstractSyntaxNodes>>, Option<Box<Symbols>>, Box<AbstractSyntaxNodes> )
}