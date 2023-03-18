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
                                        Ok( Box::new(AbstractSyntaxNodes::Tuple( start_pos, self.current_position(), symbol1.to_owned(), right, symbol2.to_owned() )) )
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
                                        Ok( Box::new(AbstractSyntaxNodes::List(start_pos, self.current_position(), symbol1.to_owned(), right, symbol2.to_owned())) )
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
                                                        Ok( Box::new(AbstractSyntaxNodes::Dictionary(start_pos, self.current_position(), symbol1.to_owned(), right, symbol2.to_owned())))
                                                    },
                                                    AbstractSyntaxNodes::SetContainer( .. ) => {
                                                        Ok( Box::new(AbstractSyntaxNodes::Set(start_pos, self.current_position(), symbol1.to_owned(), right, symbol2.to_owned())))
                                                    },
                                                    _ => Ok( Box::new(AbstractSyntaxNodes::Dictionary(start_pos, self.current_position(), symbol1.to_owned(), None, symbol2.to_owned())))
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
        Ok(Box::new(AbstractSyntaxNodes::Empty))
    }

    fn parse_power( &mut self ) -> Result<Box<AbstractSyntaxNodes>, Box<String>> {
        Ok(Box::new(AbstractSyntaxNodes::Empty))
    }

    fn parse_factor( &mut self ) -> Result<Box<AbstractSyntaxNodes>, Box<String>> {
        Ok(Box::new(AbstractSyntaxNodes::Empty))
    }

    fn parse_term( &mut self ) -> Result<Box<AbstractSyntaxNodes>, Box<String>> {
        Ok(Box::new(AbstractSyntaxNodes::Empty))
    }

    fn parse_arith_expr( &mut self ) -> Result<Box<AbstractSyntaxNodes>, Box<String>> {
        Ok(Box::new(AbstractSyntaxNodes::Empty))
    }

    fn parse_shift_expr( &mut self ) -> Result<Box<AbstractSyntaxNodes>, Box<String>> {
        Ok(Box::new(AbstractSyntaxNodes::Empty))
    }

    fn parse_and_expr( &mut self ) -> Result<Box<AbstractSyntaxNodes>, Box<String>> {
        Ok(Box::new(AbstractSyntaxNodes::Empty))
    }

    fn parse_xor_expr( &mut self ) -> Result<Box<AbstractSyntaxNodes>, Box<String>> {
        Ok(Box::new(AbstractSyntaxNodes::Empty))
    }

    fn parse_or_expr( &mut self ) -> Result<Box<AbstractSyntaxNodes>, Box<String>> {
        Ok(Box::new(AbstractSyntaxNodes::Empty))
    }

    fn parse_star_expr( &mut self ) -> Result<Box<AbstractSyntaxNodes>, Box<String>> {
        Ok(Box::new(AbstractSyntaxNodes::Empty))
    }

    fn parse_comparison( &mut self ) -> Result<Box<AbstractSyntaxNodes>, Box<String>> {
        Ok(Box::new(AbstractSyntaxNodes::Empty))
    }

    fn parse_not_test( &mut self ) -> Result<Box<AbstractSyntaxNodes>, Box<String>> {
        Ok(Box::new(AbstractSyntaxNodes::Empty))
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
