
pub struct TDEvaluator {
    pub last_const: Option<i64>
}

impl TDEvaluator {
    pub fn new() -> Self {
        Self {last_const: None}
    }
}

impl super::AstVisitor for TDEvaluator {

    fn visit_constant(&mut self, ast: &super::AstConstant) {
        self.last_const = Some(ast.number);
    }

    fn visit_binary_op(&mut self, ast: &super::AstBinaryOperation) {
        self.visit_expr(&ast.left);
        let left = self.last_const.unwrap();
        
        self.visit_expr(&ast.right);
        let right = self.last_const.unwrap();
        
        self.last_const = Some(match ast.operator._type {
            super::AstBinaryOperationType::Add => left + right,
            super::AstBinaryOperationType::Sub => left - right,
            super::AstBinaryOperationType::Mul => left * right,
            super::AstBinaryOperationType::Div => left / right,
        })

    }
}