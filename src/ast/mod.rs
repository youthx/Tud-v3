pub mod lexer;
pub mod parser;
pub mod preprocessor;

pub struct Ast {
    pub statements: Vec<AstStmt>,
}

impl Ast {
    pub fn new() -> Self {
        Self {
            statements: Vec::new(),
        }
    }

    pub fn add_stmt(&mut self, stmt: AstStmt) {
        self.statements.push(stmt);
    }

    pub fn visit(&self, visitor: &mut dyn AstVisitor) {
        for stmt in &self.statements {
            visitor.visit_stmt(stmt);
        }
    }

    pub fn visualize(&self) -> () {
        let mut printer = AstPrinter { indent: 0 };
        self.visit(&mut printer);
    }
}

pub enum AstStmtType {
    Expr(AstExpr),
}

pub struct AstStmt {
    pub _type: AstStmtType,
}

impl AstStmt {
    pub fn new(type_: AstStmtType) -> Self {
        Self { _type: type_ }
    }

    pub fn expr(expr: AstExpr) -> Self {
        AstStmt::new(AstStmtType::Expr(expr))
    }
}

pub enum AstExprType {
    Constant64(AstConstant),
    Binary(AstBinaryOperation),
    Parenthesized(AstParenthesized),
}

pub struct AstExpr {
    pub _type: AstExprType,
}

pub struct AstBinaryOperation {
    pub(crate) left: Box<AstExpr>,
    pub(crate) right: Box<AstExpr>,
    pub(crate) operator: AstBinaryOperator,
}

pub enum AstBinaryOperationType {
    Add,
    Sub,
    Mul,
    Div,
}

pub struct AstBinaryOperator {
    pub(crate) _type: AstBinaryOperationType,
    pub(crate) token: lexer::TDToken,
}

impl AstBinaryOperator {
    pub fn precedence(&self) -> u8 {
        match self._type {
            AstBinaryOperationType::Add => 1,
            AstBinaryOperationType::Sub => 1,
            AstBinaryOperationType::Mul => 2,
            AstBinaryOperationType::Div => 2,
        }
    }
}

pub struct AstParenthesized {
    expr: Box<AstExpr>,
}

pub struct AstConstant {
    number: i64,
}

impl AstExpr {
    pub fn new(type_: AstExprType) -> Self {
        AstExpr { _type: type_ }
    }

    pub fn constant_64(n: i64) -> Self {
        AstExpr::new(AstExprType::Constant64(AstConstant { number: n }))
    }

    pub fn binary(operator: AstBinaryOperator, left: AstExpr, right: AstExpr) -> Self {
        AstExpr::new(AstExprType::Binary(AstBinaryOperation {
            operator,
            left: Box::new(left),
            right: Box::new(right),
        }))
    }

    pub fn parenthesized(expr: AstExpr) -> Self {
        AstExpr::new(AstExprType::Parenthesized(AstParenthesized {
            expr: Box::new(expr),
        }))
    }
}

pub trait AstVisitor {
    fn do_visit_stmt(&mut self, stmt: &AstStmt) {
        match &stmt._type {
            AstStmtType::Expr(expr) => {
                self.visit_expr(expr);
            }
        }
    }

    fn do_visit_expr(&mut self, expr: &AstExpr) {
        match &expr._type {
            AstExprType::Constant64(number) => {
                self.visit_constant(number);
            }
            AstExprType::Binary(binary) => {
                self.visit_binary_op(binary);
            },
            AstExprType::Parenthesized(paren) => {
                self.visit_parenthesized(paren);
            }
        }
    }

    fn visit_stmt(&mut self, ast: &AstStmt) {
        self.do_visit_stmt(ast)
    }

    fn visit_expr(&mut self, ast: &AstExpr) {
        self.do_visit_expr(ast)
    }

    fn visit_parenthesized(&mut self, ast: &AstParenthesized) {
        self.visit_expr(&ast.expr);
    }
    
    fn visit_binary_op(&mut self, ast: &AstBinaryOperation) {
        self.visit_expr(&ast.left);
        self.visit_expr(&ast.right);
    }
    fn visit_constant(&mut self, ast: &AstConstant);
}

pub struct AstPrinter {
    indent: usize,
}

impl AstVisitor for AstPrinter {
    fn visit_constant(&mut self, ast: &AstConstant) {
        self.print_with_indent(&format!("const [{}]", ast.number));
    }

    fn visit_stmt(&mut self, ast: &AstStmt) {
        self.print_with_indent("stmt:");
        self.indent += Self::INDENT;
        AstVisitor::do_visit_stmt(self, ast);
        self.indent -= Self::INDENT;
    }

    fn visit_expr(&mut self, ast: &AstExpr) {
        self.print_with_indent("expr:");
        self.indent += Self::INDENT;
        AstVisitor::do_visit_expr(self, ast);
        self.indent -= Self::INDENT;
    }

    fn visit_binary_op(&mut self, ast: &AstBinaryOperation) {
        self.print_with_indent(&format!("binop [{}]:", ast.operator.token.lexeme.literal));
        self.indent += Self::INDENT;
        self.visit_expr(&ast.left);
        self.visit_expr(&ast.right);
        self.indent -= Self::INDENT;
    }

    fn visit_parenthesized(&mut self, ast: &AstParenthesized) {
        self.print_with_indent(&format!("group:"));
        self.indent += Self::INDENT;
        self.visit_expr(&ast.expr);
        self.indent -= Self::INDENT;
    }
}

impl AstPrinter {
    const INDENT: usize = 2;

    fn print_with_indent(&mut self, text: &str) {
        println!("{}{}", " ".repeat(self.indent), text);
    }
}
