mod ast;
use crate::ast::Ast;

fn main() {
    let src = "(8 + 4) * (9 * 2)";

    let mut ast: Ast = Ast::new();
    let mut parser = ast::parser::TDParser::from_source(src);
    while let Some(stmt) = parser.next_statement() {
        ast.add_stmt(stmt);
    }
    let mut evaluator = ast::preprocessor::TDEvaluator::new();
    ast.visit(&mut evaluator);
    println!("Last Constant: {}", evaluator.last_const.unwrap());
}


