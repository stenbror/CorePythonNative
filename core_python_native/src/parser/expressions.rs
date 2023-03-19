use crate::parser::abstract_syntax_tree::AbstractSyntaxNodes;
use crate::parser::python_core_parser::{PythonCoreParser, PythonParser};
use crate::parser::symbols::*;

// Trait: Expression rules ////////////////////////////////////////////////////////////////////////////////////////////

trait Expressions {
    fn parse_atom( &mut self ) -> Result<Box<AbstractSyntaxNodes>, Box<String>>;
    fn parse_atom_expr( &mut self ) -> Result<Box<AbstractSyntaxNodes>, Box<String>>;
    fn parse_power( &mut self ) -> Result<Box<AbstractSyntaxNodes>, Box<String>>;
    fn parse_factor( &mut self ) -> Result<Box<AbstractSyntaxNodes>, Box<String>>;
    fn parse_term( &mut self ) -> Result<Box<AbstractSyntaxNodes>, Box<String>>;
    fn parse_arith_expr( &mut self ) -> Result<Box<AbstractSyntaxNodes>, Box<String>>;
    fn parse_shift_expr( &mut self ) -> Result<Box<AbstractSyntaxNodes>, Box<String>>;
    fn parse_and_expr( &mut self ) -> Result<Box<AbstractSyntaxNodes>, Box<String>>;
    fn parse_xor_expr( &mut self ) -> Result<Box<AbstractSyntaxNodes>, Box<String>>;
    fn parse_or_expr( &mut self ) -> Result<Box<AbstractSyntaxNodes>, Box<String>>;
    fn parse_star_expr( &mut self ) -> Result<Box<AbstractSyntaxNodes>, Box<String>>;
    fn parse_comparison( &mut self ) -> Result<Box<AbstractSyntaxNodes>, Box<String>>;
    fn parse_not_test( &mut self ) -> Result<Box<AbstractSyntaxNodes>, Box<String>>;
    fn parse_and_test( &mut self ) -> Result<Box<AbstractSyntaxNodes>, Box<String>>;
    fn parse_or_test( &mut self ) -> Result<Box<AbstractSyntaxNodes>, Box<String>>;
    fn parse_lambda( &mut self, is_cond: bool) -> Result<Box<AbstractSyntaxNodes>, Box<String>>;
    fn parse_test_no_cond( &mut self ) -> Result<Box<AbstractSyntaxNodes>, Box<String>>;
    fn parse_test( &mut self ) -> Result<Box<AbstractSyntaxNodes>, Box<String>>;
    fn parse_named_expr( &mut self ) -> Result<Box<AbstractSyntaxNodes>, Box<String>>;
    fn parse_testlist_comp( &mut self ) -> Result<Box<AbstractSyntaxNodes>, Box<String>>;
    fn parse_trailer( &mut self ) -> Result<Box<AbstractSyntaxNodes>, Box<String>>;
    fn parse_subscript_list( &mut self ) -> Result<Box<AbstractSyntaxNodes>, Box<String>>;
    fn parse_subscript( &mut self ) -> Result<Box<AbstractSyntaxNodes>, Box<String>>;
    fn parse_testlist( &mut self ) -> Result<Box<AbstractSyntaxNodes>, Box<String>>;
    fn parse_exprlist( &mut self ) -> Result<Box<AbstractSyntaxNodes>, Box<String>>;
    fn parse_dictionary_or_set_maker( &mut self ) -> Result<Box<AbstractSyntaxNodes>, Box<String>>;
    fn parse_comp_iter( &mut self ) -> Result<Box<AbstractSyntaxNodes>, Box<String>>;
    fn parse_sync_comp_for( &mut self ) -> Result<Box<AbstractSyntaxNodes>, Box<String>>;
    fn parse_comp_for( &mut self ) -> Result<Box<AbstractSyntaxNodes>, Box<String>>;
    fn parse_comp_if( &mut self ) -> Result<Box<AbstractSyntaxNodes>, Box<String>>;
    fn parse_var_argslist( &mut self ) -> Result<Box<AbstractSyntaxNodes>, Box<String>>;
    fn parse_vfp_def( &mut self ) -> Result<Box<AbstractSyntaxNodes>, Box<String>>;
    fn parse_yield_expr( &mut self ) -> Result<Box<AbstractSyntaxNodes>, Box<String>>;
    fn parse_testlist_star_expr( &mut self ) -> Result<Box<AbstractSyntaxNodes>, Box<String>>;
    fn parse_arglist( &mut self ) -> Result<Box<AbstractSyntaxNodes>, Box<String>>;
    fn parse_argument( &mut self ) -> Result<Box<AbstractSyntaxNodes>, Box<String>>;
}


// Implemention of Expression rules for PythonCoreParser //////////////////////////////////////////////////////////////


impl Expressions for PythonCoreParser {

    // Rule: Name | Number | String+ | ... | False | None | True | '(' [ yield_Expr | testlist_comp ] ')' | '[' subscript_list ']' | '{' dictionary or set '}'
    fn parse_atom( &mut self ) -> Result<Box<AbstractSyntaxNodes>, Box<String>> {
        let start_pos = self.symbol_position();
        match &*self.symbol.clone() {
            Ok( s ) => {
                let symbol1 = s.clone();
                self.advance();
                match &*symbol1 {
                    Symbols::PyEllipsis( ..) =>
                        Ok( Box::new ( AbstractSyntaxNodes::Ellipsis( start_pos, self.symbol_position() - 1, symbol1.to_owned() ) ) ),
                    Symbols::PyFalse( .. ) =>
                        Ok( Box::new ( AbstractSyntaxNodes::False( start_pos, self.symbol_position() - 1, symbol1.to_owned() ) ) ),
                    Symbols::PyNone( .. ) =>
                        Ok( Box::new ( AbstractSyntaxNodes::None( start_pos, self.symbol_position() - 1, symbol1.to_owned() ) ) ),
                    Symbols::PyTrue( .. ) =>
                        Ok( Box::new ( AbstractSyntaxNodes::True( start_pos, self.symbol_position() - 1, symbol1.to_owned() ) ) ),
                    Symbols::PyName( .. ) =>
                        Ok( Box::new ( AbstractSyntaxNodes::Name( start_pos, self.symbol_position() - 1, symbol1.to_owned() ) ) ),
                    Symbols::PyNumber( ..  ) =>
                        Ok( Box::new ( AbstractSyntaxNodes::Number( start_pos, self.symbol_position() - 1, symbol1.to_owned() ) ) ),
                    Symbols::PyString( .. ) => {
                        let mut lst: Vec<Box<Symbols>> = Vec::new();
                        lst.push( symbol1.to_owned() );

                        while   match &*self.symbol.clone() {
                                    Ok( s ) => {
                                        let symbol2 = (*s).clone();
                                        match *symbol2 {
                                            Symbols::PyString( _ , _ , _ ) => {
                                                self.advance();
                                                lst.push( symbol2.to_owned() );
                                                true
                                            },
                                            _ => false
                                        }
                                    },
                                    _ => false
                                } {}

                        lst.reverse();
                        Ok( Box::new( AbstractSyntaxNodes::String( start_pos, self.symbol_position() - 1, Box::new( lst.to_owned() ) ) ) )
                    },
                    Symbols::PyLeftParen( _ , _  ) => {
                        let mut right : Option<Box<AbstractSyntaxNodes>> = None;

                        match &*self.symbol.clone() {
                            Ok(s) => {
                                match **s {
                                    Symbols::PyYield(..) => {
                                        right = Some( self.parse_yield_expr()? );
                                    },
                                    Symbols::PyRightParen(..) => {},
                                    _ => {
                                        right = Some( self.parse_testlist_comp()? );
                                    },
                                }
                            },
                            _ => return Err( Box::new( format!("SyntaxError: ( {} ) - No Symbols!", self.symbol_position() ).to_string() ) )
                        }

                        match &*self.symbol.clone() {
                            Ok(s2) => {
                                match **s2 {
                                    Symbols::PyRightParen( .. ) => {
                                        let symbol2 = (*s2).clone();
                                        self.advance();
                                        Ok( Box::new(AbstractSyntaxNodes::Tuple( start_pos, self.current_position() - 1, symbol1.to_owned(), right, symbol2.to_owned() )) )
                                    },
                                    _ => Err( Box::new( format!("SyntaxError: ( {} ) - Expecting ')' in Tuple!", self.symbol_position() ).to_string() ) )
                                }
                            },
                            _ => return Err( Box::new( format!("SyntaxError: ( {} ) - No Symbols!", self.symbol_position() ).to_string() ) )
                        }
                    },
                    Symbols::PyLeftBracket( _ , _  ) => {
                        let mut right : Option<Box<AbstractSyntaxNodes>> = None;

                        match &*self.symbol.clone() {
                            Ok(s) => {
                                match **s {
                                    Symbols::PyRightBracket( .. ) => { },
                                    _ => {
                                        right = Some( self.parse_testlist_comp()? );
                                    }
                                }
                            },
                            _ => return Err( Box::new( format!("SyntaxError: ( {} ) - No Symbols!", self.symbol_position() ).to_string() ) )
                        }

                        match &*self.symbol.clone() {
                            Ok(s2) => {
                                match **s2 {
                                    Symbols::PyRightBracket( .. ) => {
                                        let symbol2 = (*s2).clone();
                                        self.advance();
                                        Ok( Box::new(AbstractSyntaxNodes::List(start_pos, self.current_position() - 1, symbol1.to_owned(), right, symbol2.to_owned())) )
                                    },
                                    _ => Err( Box::new( format!("SyntaxError: ( {} ) - Expecting ']' in List!", self.symbol_position() ).to_string() )  )
                                }
                            },
                            _ => return Err( Box::new( format!("SyntaxError: ( {} ) - No Symbols!", self.symbol_position() ).to_string() ))
                        }
                    },
                    Symbols::PyLeftCurly( _ , _  ) => {
                        let mut right : Option<Box<AbstractSyntaxNodes>> = None;

                        match &*self.symbol.clone() {
                            Ok(s) => {
                                match **s {
                                    Symbols::PyRightCurly( .. ) => { },
                                    _ => {
                                        right = Some( self.parse_dictionary_or_set_maker()? );
                                    }
                                }
                            },
                            _ => return Err( Box::new( format!("SyntaxError: ( {} ) - Expecting valid literal!", self.symbol_position() ).to_string() ) )
                        }

                        match &*self.symbol.clone() {
                            Ok(s2) => {
                                match **s2 {
                                    Symbols::PyRightCurly(..) => {
                                        let symbol2 = (*s2).clone();
                                        self.advance();
                                        match right {
                                            Some( ref a ) => {
                                                match &**a {
                                                    AbstractSyntaxNodes::DictionaryContainer( .. ) => {
                                                        Ok( Box::new(AbstractSyntaxNodes::Dictionary(start_pos, self.current_position() - 1, symbol1.to_owned(), right, symbol2.to_owned())))
                                                    },
                                                    AbstractSyntaxNodes::SetContainer( .. ) => {
                                                        Ok( Box::new(AbstractSyntaxNodes::Set(start_pos, self.current_position() - 1, symbol1.to_owned(), right, symbol2.to_owned())))
                                                    },
                                                    _ => Ok( Box::new(AbstractSyntaxNodes::Dictionary(start_pos, self.current_position() - 1, symbol1.to_owned(), None, symbol2.to_owned())))
                                                }
                                            },
                                            None => Err( Box::new( format!("SyntaxError: ( {} ) - Expecting valid literal!", self.symbol_position() ).to_string() ) )
                                        }
                                    },
                                    _ => Err( Box::new( format!("SyntaxError: ( {} ) - Expecting '}}' in dictionary or set!", self.symbol_position() ).to_string() ) )
                                }
                            },
                            _ => return Err( Box::new( format!("SyntaxError: ( {} ) - No Symbols!", self.symbol_position() ).to_string() ) )
                        }

                    },
                    _ => Err( Box::new( format!("SyntaxError: ( {} ) - No Symbols!", self.symbol_position() ).to_string() ) )
                }
            },
            _ =>    Err( Box::new( format!("SyntaxError: ( {} ) - No Symbols!", self.symbol_position() ).to_string() ) )
        }
    }

    // Rule: [ await? ] atom [ trailer* ]
    fn parse_atom_expr( &mut self ) -> Result<Box<AbstractSyntaxNodes>, Box<String>> {
        let start_pos = self.symbol_position();

        let await_symbol = match *self.symbol.clone() {
                                    Ok(s) => {
                                        let symbol1 = s.clone();
                                        match &*symbol1 {
                                            Symbols::PyAwait( .. ) => {
                                                let _ = &self.advance();
                                                Some( symbol1.to_owned() )
                                            },
                                            _ => None
                                        }
                                    },
                                    _ => None
                                };

        let right = self.parse_atom_expr()?;

        let mut lst : Vec<Box<AbstractSyntaxNodes>> = Vec::new();
        while   match *self.symbol.clone() {
                    Ok(s) => {
                        match *s {
                            Symbols::PyLeftParen( .. ) |
                            Symbols::PyLeftBracket( .. ) |
                            Symbols::PyDot( .. ) => {
                                let trailer = self.parse_trailer()?;
                                lst.push( trailer.to_owned() );
                                true
                            },
                            _ => false
                        }
                    },
                    _ => false
                } {};
        lst.reverse();

        match ( &await_symbol, lst.len() ) {
            ( None, 0 ) => Ok( right ),
            ( _ , _ ) => {
                let trailers = match lst.len() { 0 => None, _ => Some( Box::new( lst ) ) };
                Ok(Box::new(AbstractSyntaxNodes::AtomExpr(start_pos, self.current_position() - 1, await_symbol, right, trailers.to_owned())))
            }
        }
    }

    // Rule: atom_expr [ '**' factor ]
    fn parse_power( &mut self ) -> Result<Box<AbstractSyntaxNodes>, Box<String>> {
        let start_pos = self.symbol_position();
        let left_node = self.parse_atom_expr()?;
        match &*self.symbol.clone() {
            Ok(s) => {
                match **s {
                    Symbols::PyPower( .. ) => {
                        let symbol = (**s).clone();
                        let _ = self.advance();
                        let right_node = self.parse_factor()?;
                        Ok( Box::new(AbstractSyntaxNodes::Power( start_pos, self.current_position() - 1, left_node, s.to_owned(), right_node)) )
                    },
                    _ => Ok( left_node )
                }
            },
            _ => Err( Box::new( format!("SyntaxError: ( {} ) - No Symbols!", self.symbol_position() ).to_string() ) )
        }
    }

    // Rule: ( '+' | '-' | '~' ) factor | power
    fn parse_factor( &mut self ) -> Result<Box<AbstractSyntaxNodes>, Box<String>> {
        let start_pos = self.symbol_position();
        match &*self.symbol {
            Ok(s) => {
                match &**s {
                    Symbols::PyPlus( .. ) => {
                        let symbol = (*s).clone();
                        let _ = self.advance();
                        let right_node = self.parse_factor()?;
                        Ok( Box::new(AbstractSyntaxNodes::UnaryPlus(start_pos, self.current_position() - 1, symbol.to_owned(), right_node)) )
                    },
                    Symbols::PyMinus( .. ) => {
                        let symbol = (*s).clone();
                        let _ = self.advance();
                        let right_node = self.parse_factor()?;
                        Ok( Box::new(AbstractSyntaxNodes::UnaryMinus(start_pos, self.current_position() - 1, symbol.to_owned(), right_node)) )
                    },
                    Symbols::PyBitwiseInvert( .. ) => {
                        let symbol = (*s).clone();
                        let _ = self.advance();
                        let right_node = self.parse_factor()?;
                        Ok( Box::new(AbstractSyntaxNodes::BitwiseInvert(start_pos, self.current_position() - 1, symbol.to_owned(), right_node)) )
                    },
                    _ => Ok ( self. parse_power()? )
                }
            },
            _ => Err( Box::new( format!("SyntaxError: ( {} ) - No Symbols!", self.symbol_position() ).to_string() )  )
        }
    }

    // Rule: factor ( ( '*'  | '/' | '//' | '%' | '@' ) factor )*
    fn parse_term( &mut self ) -> Result<Box<AbstractSyntaxNodes>, Box<String>> {
        let start_pos = self.symbol_position();
        let mut left_node = self.parse_factor()?;
        while   match &*self.symbol.clone() {
                    Ok(symbol_x) => {
                        let symbol = (*symbol_x).clone();
                        match &*symbol {
                            Symbols::PyMul( .. ) => {
                                let _ = self.advance();
                                let right_node = self.parse_factor()?;
                                left_node = Box::new(AbstractSyntaxNodes::Mul(start_pos, self.current_position() - 1,left_node, symbol.to_owned(), right_node));
                                true
                            },
                            Symbols::PyDiv( .. ) => {
                                let _ = self.advance();
                                let right_node = self.parse_factor()?;
                                left_node = Box::new(AbstractSyntaxNodes::Div(start_pos, self.current_position() - 1,left_node, symbol.to_owned(), right_node));
                                true
                            },
                            Symbols::PyFloorDiv( .. ) => {
                                let _ = self.advance();
                                let right_node = self.parse_factor()?;
                                left_node = Box::new(AbstractSyntaxNodes::FloorDiv(start_pos, self.current_position() - 1,left_node, symbol.to_owned(), right_node));
                                true
                            },
                            Symbols::PyModulo( .. ) => {
                                let _ = self.advance();
                                let right_node = self.parse_factor()?;
                                left_node = Box::new(AbstractSyntaxNodes::Modulo(start_pos, self.current_position() - 1, left_node, symbol.to_owned(), right_node));
                                true
                            },
                            Symbols::PyMatrices( .. ) => {
                                let _ = self.advance();
                                let right_node = self.parse_factor()?;
                                left_node = Box::new(AbstractSyntaxNodes::Matrices(start_pos, self.current_position() - 1,left_node, symbol.to_owned(), right_node));
                                true
                            },
                            _ => false
                        }
                    },
                    _ => return Err( Box::new( format!("SyntaxError: ( {} ) - No Symbols!", self.symbol_position() ).to_string() ) )
        } {};

        Ok( left_node )
    }

    // Rule:  term ( ( '+'  | '-' ) term )*
    fn parse_arith_expr( &mut self ) -> Result<Box<AbstractSyntaxNodes>, Box<String>> {
        let start_pos = self.symbol_position();
        let mut left_node = self.parse_term()?;
        while   match &*self.symbol {
                    Ok(symbol_x) => {
                        let symbol = (*symbol_x).clone();
                        match &*symbol {
                            Symbols::PyPlus( .. ) => {
                                let _ = self.advance();
                                let right_node = self.parse_term()?;
                                left_node = Box::new(AbstractSyntaxNodes::Plus(start_pos, self.current_position() - 1, left_node,symbol.to_owned(), right_node));
                                true
                            },
                            Symbols::PyMinus( .. ) => {
                                let _ = self.advance();
                                let right_node = self.parse_term()?;
                                left_node = Box::new(AbstractSyntaxNodes::Minus(start_pos, self.current_position() - 1, left_node,symbol.to_owned(), right_node));
                                true
                            },
                            _ => false
                        }
                    },
                    _ => return Err( Box::new( format!("SyntaxError: ( {} ) - No Symbols!", self.symbol_position() ).to_string() ) )
                } {};

        Ok( left_node )
    }

    // Rule: arith ( ( ( '<<'  | '>>' )  ) arith )*
    fn parse_shift_expr( &mut self ) -> Result<Box<AbstractSyntaxNodes>, Box<String>> {
        let start_pos = self.symbol_position();
        let mut left_node = self.parse_arith_expr()?;
        while   match &*self.symbol {
                    Ok(symbol_x) => {
                        let symbol = (*symbol_x).clone();
                        match &*symbol {
                            Symbols::PyShiftLeft( .. ) => {
                                let _ = self.advance();
                                let right_node = self.parse_arith_expr()?;
                                left_node = Box::new(AbstractSyntaxNodes::ShiftLeft(start_pos, self.current_position() - 1, left_node, symbol.to_owned(), right_node));
                                true
                            },
                            Symbols::PyShiftRight( .. ) => {
                                let _ = self.advance();
                                let right_node = self.parse_arith_expr()?;
                                left_node = Box::new(AbstractSyntaxNodes::ShiftRight(start_pos, self.current_position() - 1, left_node,symbol.to_owned(), right_node));
                                true
                            },
                            _ => false
                        }
                    },
                    _ => return Err( Box::new( format!("SyntaxError: ( {} ) - No Symbols!", self.symbol_position() ).to_string() ) )
                } {};

        Ok( left_node )
    }

    // Rule: shift_expr ( '&' shift_expr )*
    fn parse_and_expr( &mut self ) -> Result<Box<AbstractSyntaxNodes>, Box<String>> {
        let start_pos = self.symbol_position();
        let mut left_node = self.parse_shift_expr()?;
        while   match &*self.symbol {
            Ok(symbol_x) => {
                let symbol = (*symbol_x).clone();
                match &*symbol {
                    Symbols::PyBitwiseAnd( .. ) => {
                        let _ = self.advance();
                        let right_node = self.parse_shift_expr()?;
                        left_node = Box::new(AbstractSyntaxNodes::BitwiseAnd(start_pos, self.current_position() - 1, left_node, symbol.to_owned(), right_node));
                        true
                    },
                    _ => false
                }
            },
            _ => return Err( Box::new( format!("SyntaxError: ( {} ) - No Symbols!", self.symbol_position() ).to_string() ) )
        } {};

        Ok( left_node )
    }

    // Rule: and_expr ( '^' and_expr )*
    fn parse_xor_expr( &mut self ) -> Result<Box<AbstractSyntaxNodes>, Box<String>> {
        let start_pos = self.symbol_position();
        let mut left_node = self.parse_and_expr()?;
        while   match &*self.symbol {
                    Ok(symbol_x) => {
                        let symbol = (*symbol_x).clone();
                        match &*symbol {
                            Symbols::PyBitwiseXor( .. ) => {
                                let _ = self.advance();
                                let right_node = self.parse_and_expr()?;
                                left_node = Box::new(AbstractSyntaxNodes::BitwiseXor(start_pos, self.current_position() - 1, left_node,symbol.to_owned(), right_node));
                                true
                            },
                            _ => false
                        }
                    },
                    _ => return Err( Box::new( format!("SyntaxError: ( {} ) - No Symbols!", self.symbol_position() ).to_string() ) )
                } {};

        Ok( left_node )
    }

    // Rule: xor_expr ( '|' xor_expr )*
    fn parse_or_expr( &mut self ) -> Result<Box<AbstractSyntaxNodes>, Box<String>> {
        let start_pos = self.symbol_position();
        let mut left_node = self.parse_xor_expr()?;
        while   match &*self.symbol {
                    Ok(symbol_x) => {
                        let symbol = (*symbol_x).clone();
                        match &*symbol {
                            Symbols::PyBitwiseOr( .. ) => {
                                let _ = self.advance();
                                let right_node = self.parse_xor_expr()?;
                                left_node = Box::new(AbstractSyntaxNodes::BitwiseOr(start_pos, self.current_position() - 1, left_node,symbol.to_owned(), right_node));
                                true
                            },
                            _ => false
                        }
                    },
                    _ => return Err( Box::new( format!("SyntaxError: ( {} ) - No Symbols!", self.symbol_position() ).to_string() ) )
                } {};

        Ok( left_node )
    }

    // Rule: '*' or_expr
    fn parse_star_expr( &mut self ) -> Result<Box<AbstractSyntaxNodes>, Box<String>> {
        let start_pos = self.symbol_position();
        match &*self.symbol {
            Ok(s) => {
                match &**s {
                    Symbols::PyMul(..) => {
                        let symbol = (*s).clone();
                        let _ = self.advance();
                        let right_node = self.parse_or_expr()?;
                        Ok(Box::new(AbstractSyntaxNodes::StarExpr(start_pos, self.current_position() - 1, symbol.to_owned(), right_node)))
                    },
                    _ => Err( Box::new( format!("SyntaxError: ( {} ) - Expecting '*' in star expression!", self.symbol_position() ).to_string() ) )
                }
            },
            _ => Err( Box::new( format!("SyntaxError: ( {} ) - No Symbols!", self.symbol_position() ).to_string() ) )
        }
    }

    // Rule: or_expr ( ( '<' | '<=' | '==' | '<>' | '!=' | '>' | '>=' | 'in' | 'is' 'not' | 'is' | 'not' 'in' ) or_expr )*
    fn parse_comparison( &mut self ) -> Result<Box<AbstractSyntaxNodes>, Box<String>> {
        let start_pos = self.symbol_position();
        let mut left_node = self.parse_or_expr()?;
        while   match &*self.symbol {
                    Ok(symbol_x) => {
                        let symbol = (*symbol_x).clone();
                        match &*symbol {
                            Symbols::PyLess(..) => {
                                let _ = self.advance();
                                let right_node = self.parse_or_expr()?;
                                left_node = Box::new(AbstractSyntaxNodes::Less(start_pos, self.current_position() - 1, left_node, symbol.to_owned(), right_node));
                                true
                            },
                            Symbols::PyLessEqual(..) => {
                                let _ = self.advance();
                                let right_node = self.parse_or_expr()?;
                                left_node = Box::new(AbstractSyntaxNodes::LessEqual(start_pos, self.current_position() - 1, left_node, symbol.to_owned(), right_node));
                                true
                            },
                            Symbols::PyEqual(..) => {
                                let _ = self.advance();
                                let right_node = self.parse_or_expr()?;
                                left_node = Box::new(AbstractSyntaxNodes::Equal(start_pos, self.current_position() - 1, left_node, symbol.to_owned(), right_node));
                                true
                            },
                            Symbols::PyGreaterEqual(..) => {
                                let _ = self.advance();
                                let right_node = self.parse_or_expr()?;
                                left_node = Box::new(AbstractSyntaxNodes::GreaterEqual(start_pos, self.current_position() - 1, left_node, symbol.to_owned(), right_node));
                                true
                            },
                            Symbols::PyGreater(..) => {
                                let _ = self.advance();
                                let right_node = self.parse_or_expr()?;
                                left_node = Box::new(AbstractSyntaxNodes::Greater(start_pos, self.current_position() - 1, left_node, symbol.to_owned(), right_node));
                                true
                            },
                            Symbols::PyNotEqual(..) => {
                                let _ = self.advance();
                                let right_node = self.parse_or_expr()?;
                                left_node = Box::new(AbstractSyntaxNodes::NotEqual(start_pos, self.current_position() - 1, left_node, symbol.to_owned(), right_node));
                                true
                            },
                            Symbols::PyIn(..) => {
                                let _ = self.advance();
                                let right_node = self.parse_or_expr()?;
                                left_node = Box::new(AbstractSyntaxNodes::In(start_pos, self.current_position() - 1, left_node,  symbol.to_owned(), right_node));
                                true
                            },
                            Symbols::PyIs(..) => {
                                let _ = self.advance();
                                match &*self.symbol {
                                    Ok(symbol_x) => {
                                        let symbol2 = (*symbol_x).clone();
                                        match &*symbol2 {
                                            Symbols::PyNot(..) => {
                                                let _ = self.advance();
                                                let right_node = self.parse_or_expr()?;
                                                left_node = Box::new(AbstractSyntaxNodes::IsNot(start_pos, self.current_position() - 1, left_node, symbol.to_owned(),  symbol2.to_owned(), right_node));
                                            },
                                            _ => {
                                                let right_node = self.parse_or_expr()?;
                                                left_node = Box::new(AbstractSyntaxNodes::Is(start_pos, self.current_position() - 1, left_node, symbol.to_owned(), right_node));
                                            }
                                        }
                                    },
                                    _ => {
                                        return Err( Box::new( format!("SyntaxError: ( {} ) - No Symbols!", self.symbol_position() ).to_string() ) )
                                    }
                                }
                                true
                            },
                            Symbols::PyNot(..) => {
                                let _ = self.advance();
                                match &*self.symbol {
                                    Ok(symbol_x) => {
                                        let symbol2 = (*symbol_x).clone();
                                        match &*symbol2 {
                                            Symbols::PyIn(..) => {
                                                let _ = self.advance();
                                                let right_node = self.parse_or_expr()?;
                                                left_node = Box::new(AbstractSyntaxNodes::NotIn(start_pos, self.current_position() - 1, left_node,  symbol.to_owned(),  symbol2.to_owned(), right_node));
                                            },
                                            _ => {
                                                return Err( Box::new( format!("SyntaxError: ( {} ) - Expecting in in not in expression!", self.symbol_position() ).to_string() ) )
                                            }
                                        }
                                    },
                                    _ => {
                                        return Err(Box::new( format!("SyntaxError: ( {} ) - No Symbols!", self.symbol_position() ).to_string() ) )
                                    }
                                }
                                true
                            },
                            _ => false
                        }
                    },
                    _ => return Err( Box::new( format!("SyntaxError: ( {} ) - No Symbols!", self.symbol_position() ).to_string() ) )
                } {};

        Ok( left_node )
    }

    // Rule: 'not' not_test
    fn parse_not_test( &mut self ) -> Result<Box<AbstractSyntaxNodes>, Box<String>> {
        let start_pos = self.symbol_position();
        match &*self.symbol {
            Ok(s) => {
                match &**s {
                    Symbols::PyNot( .. ) => {
                        let symbol = (*s).clone();
                        let _ = self.advance();
                        let right_node = self.parse_not_test()?;
                        Ok( Box::new(AbstractSyntaxNodes::NotTest(start_pos, self.current_position() - 1, symbol.to_owned(), right_node)) )
                    },
                    _ => self.parse_comparison()
                }
            },
            _ => Err( Box::new( format!("SyntaxError: ( {} ) - No Symbols!", self.symbol_position() ).to_string() ) )
        }
    }

    fn parse_and_test( &mut self ) -> Result<Box<AbstractSyntaxNodes>, Box<String>> {
        Ok(Box::new(AbstractSyntaxNodes::Empty))
    }

    fn parse_or_test( &mut self ) -> Result<Box<AbstractSyntaxNodes>, Box<String>> {
        Ok(Box::new(AbstractSyntaxNodes::Empty))
    }

    fn parse_lambda( &mut self, is_cond: bool) -> Result<Box<AbstractSyntaxNodes>, Box<String>> {
        Ok(Box::new(AbstractSyntaxNodes::Empty))
    }

    fn parse_test_no_cond( &mut self ) -> Result<Box<AbstractSyntaxNodes>, Box<String>> {
        Ok(Box::new(AbstractSyntaxNodes::Empty))
    }

    fn parse_test( &mut self ) -> Result<Box<AbstractSyntaxNodes>, Box<String>> {
        Ok(Box::new(AbstractSyntaxNodes::Empty))
    }

    fn parse_named_expr( &mut self ) -> Result<Box<AbstractSyntaxNodes>, Box<String>> {
        Ok(Box::new(AbstractSyntaxNodes::Empty))
    }

    fn parse_testlist_comp( &mut self ) -> Result<Box<AbstractSyntaxNodes>, Box<String>> {
        Ok(Box::new(AbstractSyntaxNodes::Empty))
    }

    fn parse_trailer( &mut self ) -> Result<Box<AbstractSyntaxNodes>, Box<String>> {
        Ok(Box::new(AbstractSyntaxNodes::Empty))
    }

    fn parse_subscript_list( &mut self ) -> Result<Box<AbstractSyntaxNodes>, Box<String>> {
        Ok(Box::new(AbstractSyntaxNodes::Empty))
    }

    fn parse_subscript( &mut self ) -> Result<Box<AbstractSyntaxNodes>, Box<String>> {
        Ok(Box::new(AbstractSyntaxNodes::Empty))
    }

    fn parse_testlist( &mut self ) -> Result<Box<AbstractSyntaxNodes>, Box<String>> {
        Ok(Box::new(AbstractSyntaxNodes::Empty))
    }

    fn parse_exprlist( &mut self ) -> Result<Box<AbstractSyntaxNodes>, Box<String>> {
        Ok(Box::new(AbstractSyntaxNodes::Empty))
    }

    fn parse_dictionary_or_set_maker( &mut self ) -> Result<Box<AbstractSyntaxNodes>, Box<String>> {
        Ok(Box::new(AbstractSyntaxNodes::Empty))
    }

    fn parse_comp_iter( &mut self ) -> Result<Box<AbstractSyntaxNodes>, Box<String>> {
        Ok(Box::new(AbstractSyntaxNodes::Empty))
    }

    fn parse_sync_comp_for( &mut self ) -> Result<Box<AbstractSyntaxNodes>, Box<String>> {
        Ok(Box::new(AbstractSyntaxNodes::Empty))
    }

    fn parse_comp_for( &mut self ) -> Result<Box<AbstractSyntaxNodes>, Box<String>> {
        Ok(Box::new(AbstractSyntaxNodes::Empty))
    }

    fn parse_comp_if( &mut self ) -> Result<Box<AbstractSyntaxNodes>, Box<String>> {
        Ok(Box::new(AbstractSyntaxNodes::Empty))
    }

    fn parse_var_argslist( &mut self ) -> Result<Box<AbstractSyntaxNodes>, Box<String>> {
        Ok(Box::new(AbstractSyntaxNodes::Empty))
    }

    fn parse_vfp_def( &mut self ) -> Result<Box<AbstractSyntaxNodes>, Box<String>> {
        Ok(Box::new(AbstractSyntaxNodes::Empty))
    }

    fn parse_yield_expr( &mut self ) -> Result<Box<AbstractSyntaxNodes>, Box<String>> {
        Ok(Box::new(AbstractSyntaxNodes::Empty))
    }

    fn parse_testlist_star_expr( &mut self ) -> Result<Box<AbstractSyntaxNodes>, Box<String>> {
        Ok(Box::new(AbstractSyntaxNodes::Empty))
    }

    fn parse_arglist( &mut self ) -> Result<Box<AbstractSyntaxNodes>, Box<String>> {
        Ok(Box::new(AbstractSyntaxNodes::Empty))
    }

    fn parse_argument( &mut self ) -> Result<Box<AbstractSyntaxNodes>, Box<String>> {
        Ok(Box::new(AbstractSyntaxNodes::Empty))
    }
}


// Expressions:  UnitTests ////////////////////////////////////////////////////////////////////////////////////////////

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dummy_test() {
        assert!(true)
    }

}

// END ////////////////////////////////////////////////////////////////////////////////////////////////////////////////
