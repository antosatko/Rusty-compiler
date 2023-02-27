/*pub mod analyzer {
    use std::collections::HashMap;

    use crate::ast_parser::ast_parser::*;
    use crate::lexer::tokenizer::Tokens;
    pub fn analyze(mut tokens: Vec<Tokens>, mut lines: Vec<(usize, usize)>, ast: Tree) {
    }
    fn analyze_struct(
        tokens: &Vec<Tokens>,
        idx: &mut usize,
        this: &Head,
    ) -> Result<HashMap<String, Branch>, AnalyzeErr> {
        let mut result = HashMap::new();
        /*for (i, param) in this.parameters.iter().enumerate() {
            
            result.insert(k, v);
        }*/
        Ok(result)
        //Err(AnalyzeErr::Placeholder)
    }
    fn analyze_scope(){

    }
    struct Branch {
        name: String,
        nodes: Vec<ParamType>,
    }
    enum ParamType {
        Array(Vec<BranchParam>),
        Primitive(BranchParam)
    }
    enum BranchParam {
        Primitive(Tokens),
        Object(Branch),
    }
    enum AnalyzeErr {
        Placeholder,
        /// expected 0, found 1
        Expected(Tokens, Tokens)
    }
}
*/