use crate::ast::lexer;

pub struct TDParser {
    tokens: Vec<lexer::TDToken>,
    pub current: usize,
}

impl TDParser {
    pub fn new() -> Self {
        Self {
            tokens: Vec::new(),
            current: 0,
        }
    }

    pub fn from_source(buffer: &str) -> Self {
        let mut lexer = lexer::TDLexer::new(buffer);
        let mut tokens = Vec::new();
        while let Some(token) = lexer.next_token() {
            if token._type == lexer::TDType::WS {
                continue;
            }

            tokens.push(token.clone());
        }
        Self { tokens, current: 0 }
    }

    fn peek(&self, offset: isize) -> Option<&lexer::TDToken> {
        let cur: isize = self.current as isize;
        self.tokens.get((cur + offset) as usize)
    }

    fn current(&self) -> Option<&lexer::TDToken> {
        self.peek(0)
    }

    fn consume(&mut self) -> Option<&lexer::TDToken> {
        self.current += 1;
        let token: &lexer::TDToken = self.peek(-1)?;

        return Some(token);
    }

    pub fn next_statement(&mut self) -> Option<super::AstStmt> {
        self.parse_statement()
    }

    fn parse_statement(&mut self) -> Option<super::AstStmt> {
        let token: &lexer::TDToken = self.current()?;
        if token._type == lexer::TDType::EOF {
            return None;
        }
        let expr = self.parse_expression()?;
        Some(super::AstStmt::expr(expr))
    }

    fn parse_expression(&mut self) -> Option<super::AstExpr> {
        self.parse_binary_op(0)
    }

    fn parse_binary_op(&mut self, precedence: u8) -> Option<super::AstExpr> {
        let mut left = self.parse_primary()?;

        while let Some(operator) = self.parse_operator() {
            self.consume();
            let operator_prec = operator.precedence();
            if operator_prec <= precedence {
                break;
            }
            let right = self.parse_binary_op(operator_prec)?;
            left = super::AstExpr::binary(operator, left, right);
        }
        return Some(left);
    }

    fn parse_operator(&mut self) -> Option<super::AstBinaryOperator> {
        let token: &lexer::TDToken = self.current()?;
        let type_ = match token._type {
            lexer::TDType::PLUS => super::AstBinaryOperationType::Add,
            lexer::TDType::MINUS => super::AstBinaryOperationType::Sub,
            lexer::TDType::ASTERISK => super::AstBinaryOperationType::Mul,
            lexer::TDType::SLASH => super::AstBinaryOperationType::Div,
            _ => return None,
        };

        Some(super::AstBinaryOperator {
            token: token.clone(),
            _type: type_,
        })
    }

    fn parse_primary(&mut self) -> Option<super::AstExpr> {
        let token: &lexer::TDToken = self.consume()?;
        return match token._type {
            lexer::TDType::Dynamic64(number) => Some(super::AstExpr::constant_64(number)),
            lexer::TDType::LPAREN => {
                let expr = self.parse_expression()?;
                self.consume_token(lexer::TDType::RPAREN);
                Some(super::AstExpr::parenthesized(expr))
            }
            _ => None,
        };
    }

    fn current_precedence(&self) -> Option<u8> {
        unimplemented!()
    }

    fn consume_token(&mut self, _type: lexer::TDType) -> Option<&lexer::TDToken> {
        let tok = self.consume()?;
        if tok._type != _type {
            panic!("Invalid type");
        }
        Some(tok)
    }
}
