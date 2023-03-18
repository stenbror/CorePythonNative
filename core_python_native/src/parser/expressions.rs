use crate::parser::abstract_syntax_tree::AbstractSyntaxNodes;
use crate::parser::python_core_parser::PythonCoreParser;

// Trait: Expression rules ////////////////////////////////////////////////////////////////////////////////////////////

trait Expressions {
    fn ParseAtom( &mut self ) -> Result<Box<AbstractSyntaxNodes>, Box<String>>;
    fn ParseAtomExpr( &mut self ) -> Result<Box<AbstractSyntaxNodes>, Box<String>>;
    fn ParsePower( &mut self ) -> Result<Box<AbstractSyntaxNodes>, Box<String>>;
    fn ParseFactor( &mut self ) -> Result<Box<AbstractSyntaxNodes>, Box<String>>;
    fn ParseTerm( &mut self ) -> Result<Box<AbstractSyntaxNodes>, Box<String>>;
    fn ParseArithExpr( &mut self ) -> Result<Box<AbstractSyntaxNodes>, Box<String>>;
    fn ParseShiftExpr( &mut self ) -> Result<Box<AbstractSyntaxNodes>, Box<String>>;
    fn ParseAndExpr( &mut self ) -> Result<Box<AbstractSyntaxNodes>, Box<String>>;
    fn ParseXorExpr( &mut self ) -> Result<Box<AbstractSyntaxNodes>, Box<String>>;
    fn ParseOrExpr( &mut self ) -> Result<Box<AbstractSyntaxNodes>, Box<String>>;
    fn ParseStarExpr( &mut self ) -> Result<Box<AbstractSyntaxNodes>, Box<String>>;
    fn ParseComparison( &mut self ) -> Result<Box<AbstractSyntaxNodes>, Box<String>>;
    fn ParseNotTest( &mut self ) -> Result<Box<AbstractSyntaxNodes>, Box<String>>;
    fn ParseAndTest( &mut self ) -> Result<Box<AbstractSyntaxNodes>, Box<String>>;
    fn ParseOrTest( &mut self ) -> Result<Box<AbstractSyntaxNodes>, Box<String>>;
    fn ParseLambda( &mut self, is_cond: bool) -> Result<Box<AbstractSyntaxNodes>, Box<String>>;
    fn ParseTestNoCond( &mut self ) -> Result<Box<AbstractSyntaxNodes>, Box<String>>;
    fn ParseTest( &mut self ) -> Result<Box<AbstractSyntaxNodes>, Box<String>>;
    fn ParseNamedExpr( &mut self ) -> Result<Box<AbstractSyntaxNodes>, Box<String>>;
    fn ParseTestListComp( &mut self ) -> Result<Box<AbstractSyntaxNodes>, Box<String>>;
    fn ParseTrailer( &mut self ) -> Result<Box<AbstractSyntaxNodes>, Box<String>>;
    fn ParseSubscriptList( &mut self ) -> Result<Box<AbstractSyntaxNodes>, Box<String>>;
    fn ParseSubscript( &mut self ) -> Result<Box<AbstractSyntaxNodes>, Box<String>>;
    fn ParseTestList( &mut self ) -> Result<Box<AbstractSyntaxNodes>, Box<String>>;
    fn ParseExprList( &mut self ) -> Result<Box<AbstractSyntaxNodes>, Box<String>>;
    fn ParseDictionaryOrSetMaker( &mut self ) -> Result<Box<AbstractSyntaxNodes>, Box<String>>;
    fn ParseCompIter( &mut self ) -> Result<Box<AbstractSyntaxNodes>, Box<String>>;
    fn ParseSyncCompFor( &mut self ) -> Result<Box<AbstractSyntaxNodes>, Box<String>>;
    fn ParseCompFor( &mut self ) -> Result<Box<AbstractSyntaxNodes>, Box<String>>;
    fn ParseCompIf( &mut self ) -> Result<Box<AbstractSyntaxNodes>, Box<String>>;
    fn ParseVarArgsList( &mut self ) -> Result<Box<AbstractSyntaxNodes>, Box<String>>;
    fn ParseVFPDef( &mut self ) -> Result<Box<AbstractSyntaxNodes>, Box<String>>;
    fn ParseYieldExpr( &mut self ) -> Result<Box<AbstractSyntaxNodes>, Box<String>>;
    fn ParseTestListStarExpr( &mut self ) -> Result<Box<AbstractSyntaxNodes>, Box<String>>;
    fn ParseArgList( &mut self ) -> Result<Box<AbstractSyntaxNodes>, Box<String>>;
    fn ParseArgument( &mut self ) -> Result<Box<AbstractSyntaxNodes>, Box<String>>;
}


// Implemention of Expression rules for PythonCoreParser //////////////////////////////////////////////////////////////


impl Expressions for PythonCoreParser {

    fn ParseAtom( &mut self ) -> Result<Box<AbstractSyntaxNodes>, Box<String>> {
        Ok(Box::new(AbstractSyntaxNodes::Empty))
    }

    fn ParseAtomExpr( &mut self ) -> Result<Box<AbstractSyntaxNodes>, Box<String>> {
        Ok(Box::new(AbstractSyntaxNodes::Empty))
    }

    fn ParsePower( &mut self ) -> Result<Box<AbstractSyntaxNodes>, Box<String>> {
        Ok(Box::new(AbstractSyntaxNodes::Empty))
    }

    fn ParseFactor( &mut self ) -> Result<Box<AbstractSyntaxNodes>, Box<String>> {
        Ok(Box::new(AbstractSyntaxNodes::Empty))
    }

    fn ParseTerm( &mut self ) -> Result<Box<AbstractSyntaxNodes>, Box<String>> {
        Ok(Box::new(AbstractSyntaxNodes::Empty))
    }

    fn ParseArithExpr( &mut self ) -> Result<Box<AbstractSyntaxNodes>, Box<String>> {
        Ok(Box::new(AbstractSyntaxNodes::Empty))
    }

    fn ParseShiftExpr( &mut self ) -> Result<Box<AbstractSyntaxNodes>, Box<String>> {
        Ok(Box::new(AbstractSyntaxNodes::Empty))
    }

    fn ParseAndExpr( &mut self ) -> Result<Box<AbstractSyntaxNodes>, Box<String>> {
        Ok(Box::new(AbstractSyntaxNodes::Empty))
    }

    fn ParseXorExpr( &mut self ) -> Result<Box<AbstractSyntaxNodes>, Box<String>> {
        Ok(Box::new(AbstractSyntaxNodes::Empty))
    }

    fn ParseOrExpr( &mut self ) -> Result<Box<AbstractSyntaxNodes>, Box<String>> {
        Ok(Box::new(AbstractSyntaxNodes::Empty))
    }

    fn ParseStarExpr( &mut self ) -> Result<Box<AbstractSyntaxNodes>, Box<String>> {
        Ok(Box::new(AbstractSyntaxNodes::Empty))
    }

    fn ParseComparison( &mut self ) -> Result<Box<AbstractSyntaxNodes>, Box<String>> {
        Ok(Box::new(AbstractSyntaxNodes::Empty))
    }

    fn ParseNotTest( &mut self ) -> Result<Box<AbstractSyntaxNodes>, Box<String>> {
        Ok(Box::new(AbstractSyntaxNodes::Empty))
    }

    fn ParseAndTest( &mut self ) -> Result<Box<AbstractSyntaxNodes>, Box<String>> {
        Ok(Box::new(AbstractSyntaxNodes::Empty))
    }

    fn ParseOrTest( &mut self ) -> Result<Box<AbstractSyntaxNodes>, Box<String>> {
        Ok(Box::new(AbstractSyntaxNodes::Empty))
    }

    fn ParseLambda( &mut self, is_cond: bool) -> Result<Box<AbstractSyntaxNodes>, Box<String>> {
        Ok(Box::new(AbstractSyntaxNodes::Empty))
    }

    fn ParseTestNoCond( &mut self ) -> Result<Box<AbstractSyntaxNodes>, Box<String>> {
        Ok(Box::new(AbstractSyntaxNodes::Empty))
    }

    fn ParseTest( &mut self ) -> Result<Box<AbstractSyntaxNodes>, Box<String>> {
        Ok(Box::new(AbstractSyntaxNodes::Empty))
    }

    fn ParseNamedExpr( &mut self ) -> Result<Box<AbstractSyntaxNodes>, Box<String>> {
        Ok(Box::new(AbstractSyntaxNodes::Empty))
    }

    fn ParseTestListComp( &mut self ) -> Result<Box<AbstractSyntaxNodes>, Box<String>> {
        Ok(Box::new(AbstractSyntaxNodes::Empty))
    }

    fn ParseTrailer( &mut self ) -> Result<Box<AbstractSyntaxNodes>, Box<String>> {
        Ok(Box::new(AbstractSyntaxNodes::Empty))
    }

    fn ParseSubscriptList( &mut self ) -> Result<Box<AbstractSyntaxNodes>, Box<String>> {
        Ok(Box::new(AbstractSyntaxNodes::Empty))
    }

    fn ParseSubscript( &mut self ) -> Result<Box<AbstractSyntaxNodes>, Box<String>> {
        Ok(Box::new(AbstractSyntaxNodes::Empty))
    }

    fn ParseTestList( &mut self ) -> Result<Box<AbstractSyntaxNodes>, Box<String>> {
        Ok(Box::new(AbstractSyntaxNodes::Empty))
    }

    fn ParseExprList( &mut self ) -> Result<Box<AbstractSyntaxNodes>, Box<String>> {
        Ok(Box::new(AbstractSyntaxNodes::Empty))
    }

    fn ParseDictionaryOrSetMaker( &mut self ) -> Result<Box<AbstractSyntaxNodes>, Box<String>> {
        Ok(Box::new(AbstractSyntaxNodes::Empty))
    }

    fn ParseCompIter( &mut self ) -> Result<Box<AbstractSyntaxNodes>, Box<String>> {
        Ok(Box::new(AbstractSyntaxNodes::Empty))
    }

    fn ParseSyncCompFor( &mut self ) -> Result<Box<AbstractSyntaxNodes>, Box<String>> {
        Ok(Box::new(AbstractSyntaxNodes::Empty))
    }

    fn ParseCompFor( &mut self ) -> Result<Box<AbstractSyntaxNodes>, Box<String>> {
        Ok(Box::new(AbstractSyntaxNodes::Empty))
    }

    fn ParseCompIf( &mut self ) -> Result<Box<AbstractSyntaxNodes>, Box<String>> {
        Ok(Box::new(AbstractSyntaxNodes::Empty))
    }

    fn ParseVarArgsList( &mut self ) -> Result<Box<AbstractSyntaxNodes>, Box<String>> {
        Ok(Box::new(AbstractSyntaxNodes::Empty))
    }

    fn ParseVFPDef( &mut self ) -> Result<Box<AbstractSyntaxNodes>, Box<String>> {
        Ok(Box::new(AbstractSyntaxNodes::Empty))
    }

    fn ParseYieldExpr( &mut self ) -> Result<Box<AbstractSyntaxNodes>, Box<String>> {
        Ok(Box::new(AbstractSyntaxNodes::Empty))
    }

    fn ParseTestListStarExpr( &mut self ) -> Result<Box<AbstractSyntaxNodes>, Box<String>> {
        Ok(Box::new(AbstractSyntaxNodes::Empty))
    }

    fn ParseArgList( &mut self ) -> Result<Box<AbstractSyntaxNodes>, Box<String>> {
        Ok(Box::new(AbstractSyntaxNodes::Empty))
    }

    fn ParseArgument( &mut self ) -> Result<Box<AbstractSyntaxNodes>, Box<String>> {
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
