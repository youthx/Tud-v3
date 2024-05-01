
#[derive(Debug, PartialEq, Clone)]
pub enum TDType {
    Dynamic64(i64),

    PLUS,
    MINUS,
    ASTERISK,
    SLASH,
    LPAREN,
    RPAREN,

    EOF, 
    WS,
    Empty,
}

#[derive(Debug, PartialEq, Clone)]
pub struct AsciiSpan {
    pub(crate) start: usize,
    pub(crate) end: usize,
    pub(crate) literal: String,
}

impl AsciiSpan {
    pub fn new(start: usize, end: usize, literal: String) -> Self {
        Self {
            start,
            end,
            literal,
        }
    }

    pub fn len(&self) -> usize {
        self.end - self.start
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct TDToken {
    pub(crate) _type: TDType,
    pub(crate) lexeme: AsciiSpan,
}

impl TDToken {
    pub fn new(type_: TDType, lexeme: AsciiSpan) -> Self {
        Self {
            _type: type_,
            lexeme,
        }
    }
}

pub struct TDLexer<'a> {
    source: &'a str,
    current_pos: usize,
}

impl<'a> TDLexer<'a> {
    pub fn new(source: &'a str) -> Self {
        Self {
            source,
            current_pos: 0,
        }
    }

    pub fn next_token(&mut self) -> Option<TDToken> {
        if self.current_pos == self.source.len() {
            let eof_char = '\0';
            self.current_pos += 1;
            return Some(
                TDToken::new(
                    TDType::EOF, 
                    AsciiSpan::new(0, 0, eof_char.to_string()))
                );
        }
        let c = self.current_char();
        return c.map(|c: char| {
            let start = self.current_pos;

            let mut _type = TDType::Empty;
            if Self::is_number(&c) {
                let number: i64 = self.parse_number();
                _type = TDType::Dynamic64(number);
            } else if Self::is_whitespace(&c) {
                self.consume();
                _type = TDType::WS;
            } else {
                _type = self.parse_operator();
            }

            let end = self.current_pos;
            let literal = self.source[start..end].to_string();
            let span = AsciiSpan::new(start, end, literal);
            TDToken::new(_type, span)
        })
        
    }

    fn is_whitespace(c: &char) -> bool {
        c.is_whitespace()
    }
    
    fn parse_operator(&mut self) -> TDType {
        let c = self.consume().unwrap();
        match c {
            '+' => TDType::PLUS,
            '-' => TDType::MINUS,
            '*' => TDType::ASTERISK,
            '/' => TDType::SLASH,
            '(' => TDType::LPAREN,
            ')' => TDType::RPAREN,
            _ => panic!("Unexpected character: {}", c),
        }
    }
    
    fn is_number(c: &char) -> bool {
        c.is_digit(10)
    }

    fn current_char(&self) -> Option<char> {
        self.source.chars().nth(self.current_pos)
    }

    fn peek_char(&self) -> Option<char> {
        self.source.chars().nth(self.current_pos+1)
    }

    fn consume(&mut self) -> Option<char> {
        if self.current_pos >= self.source.len() {
            return None;
        }
        let c = self.current_char();
        self.current_pos += 1;
        
        c
    }

    fn parse_number(&mut self) -> i64 {
        let mut number: i64 = 0;
        while let Some(c) = self.current_char() {
            if c.is_digit(10) {
                self.consume().unwrap();
                number = number * 10 + c.to_digit(10).unwrap() as i64;
            } else {
                break;
            }
        }
        number
    }
}
