use crate::parser::abstract_syntax_tree::AbstractSyntaxNodes;
use crate::parser::python_core_parser::PythonCoreParser;

// Trait: Expression rules ////////////////////////////////////////////////////////////////////////////////////////////

trait Expressions {
    fn ParseAtom() -> Result<Box<AbstractSyntaxNodes>, Box<String>>;
    fn ParseAtomExpr() -> Result<Box<AbstractSyntaxNodes>, Box<String>>;
    fn ParsePower() -> Result<Box<AbstractSyntaxNodes>, Box<String>>;
    fn ParseFactor() -> Result<Box<AbstractSyntaxNodes>, Box<String>>;
    fn ParseTerm() -> Result<Box<AbstractSyntaxNodes>, Box<String>>;
    fn ParseArithExpr() -> Result<Box<AbstractSyntaxNodes>, Box<String>>;
    fn ParseShiftExpr() -> Result<Box<AbstractSyntaxNodes>, Box<String>>;
    fn ParseAndExpr() -> Result<Box<AbstractSyntaxNodes>, Box<String>>;
    fn ParseXorExpr() -> Result<Box<AbstractSyntaxNodes>, Box<String>>;
    fn ParseOrExpr() -> Result<Box<AbstractSyntaxNodes>, Box<String>>;
    fn ParseStarExpr() -> Result<Box<AbstractSyntaxNodes>, Box<String>>;
    fn ParseComparison() -> Result<Box<AbstractSyntaxNodes>, Box<String>>;
    fn ParseNotTest() -> Result<Box<AbstractSyntaxNodes>, Box<String>>;
    fn ParseAndTest() -> Result<Box<AbstractSyntaxNodes>, Box<String>>;
    fn ParseOrTest() -> Result<Box<AbstractSyntaxNodes>, Box<String>>;
    fn ParseLambda(isCond: bool) -> Result<Box<AbstractSyntaxNodes>, Box<String>>;
    fn ParseTestNoCond() -> Result<Box<AbstractSyntaxNodes>, Box<String>>;
    fn ParseTest() -> Result<Box<AbstractSyntaxNodes>, Box<String>>;
    fn ParseNamedExpr() -> Result<Box<AbstractSyntaxNodes>, Box<String>>;
    fn ParseTestListComp() -> Result<Box<AbstractSyntaxNodes>, Box<String>>;
    fn ParseTrailer() -> Result<Box<AbstractSyntaxNodes>, Box<String>>;
    fn ParseSubscriptList() -> Result<Box<AbstractSyntaxNodes>, Box<String>>;
    fn ParseSubscript() -> Result<Box<AbstractSyntaxNodes>, Box<String>>;
    fn ParseTestList() -> Result<Box<AbstractSyntaxNodes>, Box<String>>;
    fn ParseExprList() -> Result<Box<AbstractSyntaxNodes>, Box<String>>;
    fn ParseDictionaryOrSetMaker() -> Result<Box<AbstractSyntaxNodes>, Box<String>>;
    fn ParseCompIter() -> Result<Box<AbstractSyntaxNodes>, Box<String>>;
    fn ParseSyncCompFor() -> Result<Box<AbstractSyntaxNodes>, Box<String>>;
    fn ParseCompFor() -> Result<Box<AbstractSyntaxNodes>, Box<String>>;
    fn ParseCompIf() -> Result<Box<AbstractSyntaxNodes>, Box<String>>;
    fn ParseVarArgsList() -> Result<Box<AbstractSyntaxNodes>, Box<String>>;
    fn ParseVFPDef() -> Result<Box<AbstractSyntaxNodes>, Box<String>>;
    fn ParseYieldExpr() -> Result<Box<AbstractSyntaxNodes>, Box<String>>;
    fn ParseTestListStarExpr() -> Result<Box<AbstractSyntaxNodes>, Box<String>>;
    fn ParseArgList() -> Result<Box<AbstractSyntaxNodes>, Box<String>>;
    fn ParseArgument() -> Result<Box<AbstractSyntaxNodes>, Box<String>>;
}


// Implemention of Expression rules for PythonCoreParser //////////////////////////////////////////////////////////////


impl Expressions for PythonCoreParser {

    fn ParseAtom() -> Result<Box<AbstractSyntaxNodes>, Box<String>> {
        Ok(Box::new(AbstractSyntaxNodes::Empty))
    }

    fn ParseAtomExpr() -> Result<Box<AbstractSyntaxNodes>, Box<String>> {
        Ok(Box::new(AbstractSyntaxNodes::Empty))
    }

    fn ParsePower() -> Result<Box<AbstractSyntaxNodes>, Box<String>> {
        Ok(Box::new(AbstractSyntaxNodes::Empty))
    }

    fn ParseFactor() -> Result<Box<AbstractSyntaxNodes>, Box<String>> {
        Ok(Box::new(AbstractSyntaxNodes::Empty))
    }

    fn ParseTerm() -> Result<Box<AbstractSyntaxNodes>, Box<String>> {
        Ok(Box::new(AbstractSyntaxNodes::Empty))
    }

    fn ParseArithExpr() -> Result<Box<AbstractSyntaxNodes>, Box<String>> {
        Ok(Box::new(AbstractSyntaxNodes::Empty))
    }

    fn ParseShiftExpr() -> Result<Box<AbstractSyntaxNodes>, Box<String>> {
        Ok(Box::new(AbstractSyntaxNodes::Empty))
    }

    fn ParseAndExpr() -> Result<Box<AbstractSyntaxNodes>, Box<String>> {
        Ok(Box::new(AbstractSyntaxNodes::Empty))
    }

    fn ParseXorExpr() -> Result<Box<AbstractSyntaxNodes>, Box<String>> {
        Ok(Box::new(AbstractSyntaxNodes::Empty))
    }

    fn ParseOrExpr() -> Result<Box<AbstractSyntaxNodes>, Box<String>> {
        Ok(Box::new(AbstractSyntaxNodes::Empty))
    }

    fn ParseStarExpr() -> Result<Box<AbstractSyntaxNodes>, Box<String>> {
        Ok(Box::new(AbstractSyntaxNodes::Empty))
    }

    fn ParseComparison() -> Result<Box<AbstractSyntaxNodes>, Box<String>> {
        Ok(Box::new(AbstractSyntaxNodes::Empty))
    }

    fn ParseNotTest() -> Result<Box<AbstractSyntaxNodes>, Box<String>> {
        Ok(Box::new(AbstractSyntaxNodes::Empty))
    }

    fn ParseAndTest() -> Result<Box<AbstractSyntaxNodes>, Box<String>> {
        Ok(Box::new(AbstractSyntaxNodes::Empty))
    }

    fn ParseOrTest() -> Result<Box<AbstractSyntaxNodes>, Box<String>> {
        Ok(Box::new(AbstractSyntaxNodes::Empty))
    }

    fn ParseLambda(isCond: bool) -> Result<Box<AbstractSyntaxNodes>, Box<String>> {
        Ok(Box::new(AbstractSyntaxNodes::Empty))
    }

    fn ParseTestNoCond() -> Result<Box<AbstractSyntaxNodes>, Box<String>> {
        Ok(Box::new(AbstractSyntaxNodes::Empty))
    }

    fn ParseTest() -> Result<Box<AbstractSyntaxNodes>, Box<String>> {
        Ok(Box::new(AbstractSyntaxNodes::Empty))
    }

    fn ParseNamedExpr() -> Result<Box<AbstractSyntaxNodes>, Box<String>> {
        Ok(Box::new(AbstractSyntaxNodes::Empty))
    }

    fn ParseTestListComp() -> Result<Box<AbstractSyntaxNodes>, Box<String>> {
        Ok(Box::new(AbstractSyntaxNodes::Empty))
    }

    fn ParseTrailer() -> Result<Box<AbstractSyntaxNodes>, Box<String>> {
        Ok(Box::new(AbstractSyntaxNodes::Empty))
    }

    fn ParseSubscriptList() -> Result<Box<AbstractSyntaxNodes>, Box<String>> {
        Ok(Box::new(AbstractSyntaxNodes::Empty))
    }

    fn ParseSubscript() -> Result<Box<AbstractSyntaxNodes>, Box<String>> {
        Ok(Box::new(AbstractSyntaxNodes::Empty))
    }

    fn ParseTestList() -> Result<Box<AbstractSyntaxNodes>, Box<String>> {
        Ok(Box::new(AbstractSyntaxNodes::Empty))
    }

    fn ParseExprList() -> Result<Box<AbstractSyntaxNodes>, Box<String>> {
        Ok(Box::new(AbstractSyntaxNodes::Empty))
    }

    fn ParseDictionaryOrSetMaker() -> Result<Box<AbstractSyntaxNodes>, Box<String>> {
        Ok(Box::new(AbstractSyntaxNodes::Empty))
    }

    fn ParseCompIter() -> Result<Box<AbstractSyntaxNodes>, Box<String>> {
        Ok(Box::new(AbstractSyntaxNodes::Empty))
    }

    fn ParseSyncCompFor() -> Result<Box<AbstractSyntaxNodes>, Box<String>> {
        Ok(Box::new(AbstractSyntaxNodes::Empty))
    }

    fn ParseCompFor() -> Result<Box<AbstractSyntaxNodes>, Box<String>> {
        Ok(Box::new(AbstractSyntaxNodes::Empty))
    }

    fn ParseCompIf() -> Result<Box<AbstractSyntaxNodes>, Box<String>> {
        Ok(Box::new(AbstractSyntaxNodes::Empty))
    }

    fn ParseVarArgsList() -> Result<Box<AbstractSyntaxNodes>, Box<String>> {
        Ok(Box::new(AbstractSyntaxNodes::Empty))
    }

    fn ParseVFPDef() -> Result<Box<AbstractSyntaxNodes>, Box<String>> {
        Ok(Box::new(AbstractSyntaxNodes::Empty))
    }

    fn ParseYieldExpr() -> Result<Box<AbstractSyntaxNodes>, Box<String>> {
        Ok(Box::new(AbstractSyntaxNodes::Empty))
    }

    fn ParseTestListStarExpr() -> Result<Box<AbstractSyntaxNodes>, Box<String>> {
        Ok(Box::new(AbstractSyntaxNodes::Empty))
    }

    fn ParseArgList() -> Result<Box<AbstractSyntaxNodes>, Box<String>> {
        Ok(Box::new(AbstractSyntaxNodes::Empty))
    }

    fn ParseArgument() -> Result<Box<AbstractSyntaxNodes>, Box<String>> {
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
