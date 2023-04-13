pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tokens(){
        let mut tokenizer = Tokenizer::new("let a = 10;");
        let mut tokens = Vec::new();

        while let Some(token) = tokenizer.next() {
            tokens.push(token);
        }

        assert_eq!(tokens[0].kind, TokenKind::Identifier(Identifier::new("let")));
        assert_eq!(tokens[1].kind, TokenKind::Identifier(Identifier::new("a")));
        assert_eq!(tokens[2].kind, TokenKind::Punctuator(Punctuator::new("=")));
        assert_eq!(tokens[3].kind, TokenKind::Number(Number::new("10")));
        assert_eq!(tokens[4].kind, TokenKind::Punctuator(Punctuator::new(";")));
    }

    #[test]
    fn number(){
        let mut tokenizer = Tokenizer::new("10");
        let mut tokens = Vec::new();

        while let Some(token) = tokenizer.next() {
            tokens.push(token);
        }

        assert_eq!(tokens[0].kind, TokenKind::Number(Number::new("10")));
    }

    #[test]
    fn string(){
        let mut tokenizer = Tokenizer::new("\"Hello World\"");
        let mut tokens = Vec::new();

        while let Some(token) = tokenizer.next() {
            tokens.push(token);
        }

        assert_eq!(tokens[0].kind, TokenKind::StringLiteral(StringLiteral::new("Hello World")));
    }

    #[test]
    fn identifier(){
        let mut tokenizer = Tokenizer::new("let");
        let mut tokens = Vec::new();

        while let Some(token) = tokenizer.next() {
            tokens.push(token);
        }

        assert_eq!(tokens[0].kind, TokenKind::Identifier(Identifier::new("let")));
    }

    #[test]
    fn operator(){
        let mut tokenizer = Tokenizer::new("+");
        let mut tokens = Vec::new();

        while let Some(token) = tokenizer.next() {
            tokens.push(token);
        }

        assert_eq!(tokens[0].kind, TokenKind::Operator(Operator::new("+")));
    }

    #[test]
    fn punctuator(){
        let mut tokenizer = Tokenizer::new(";");
        let mut tokens = Vec::new();

        while let Some(token) = tokenizer.next() {
            tokens.push(token);
        }

        assert_eq!(tokens[0].kind, TokenKind::Punctuator(Punctuator::new(";")));
    }
}

//create a list of all operators
const OPERATORS: [&str; 16] = ["+", "-", "*", "/", "%", "++", "--", "==", "!=", "<", ">", "<=", ">=", "&&", "||", "!"];

//create a list of all punctuators
const PUNCTUATORS: [&str; 10] = ["(", ")", "{", "}", "[", "]", ",", ";", ".", "="];

#[derive(Debug, PartialEq)]
pub struct Operator(pub String);

impl Operator {
    pub fn new(value: &str) -> Self {
        Operator(value.to_string())
    }
}

#[derive(Debug, PartialEq)]
pub struct Punctuator(pub String);

impl Punctuator {
    pub fn new(value: &str) -> Self {
        Punctuator(value.to_string())
    }
}

#[derive(Debug, PartialEq)]
pub struct Number(pub f64);

impl Number {
    pub fn new(value: &str) -> Self {
        Number(value.parse::<f64>().unwrap())
    }
}

#[derive(Debug, PartialEq)]
pub struct StringLiteral(pub String);

impl StringLiteral {
    pub fn new(value: &str) -> Self {
        StringLiteral(value.to_string())
    }
}

#[derive(Debug, PartialEq)]
pub struct Identifier(pub String);

impl Identifier {
    pub fn new(value: &str) -> Self {
        Identifier(value.to_string())
    }
}

#[derive(Debug, PartialEq)]
pub struct Token {
    pub kind: TokenKind,
    pub line: usize,
    pub column: usize,
}

impl Token {
    pub fn new(kind: TokenKind, line: usize, column: usize) -> Self {
        Token { kind, line, column }
    }
}

#[derive(Debug, PartialEq)]
pub enum TokenKind {
    Operator(Operator),
    Punctuator(Punctuator),
    Number(Number),
    StringLiteral(StringLiteral),
    Identifier(Identifier),
    Error(Error)
}

#[derive(Debug, PartialEq)]
pub struct Tokenizer {
    input: String,
    position: usize,
    line: usize,
    column: usize,
}

impl Tokenizer {
    pub fn new(input: &str) -> Self {
        Tokenizer {
            input: input.to_string(),
            position: 0,
            line: 1,
            column: 1,
        }
    }

    pub fn next(&mut self) -> Option<Token> {
        if self.position >= self.input.len() {
            return None;
        }

        let mut token = None;

        while token.is_none() {
            let c = self.input.chars().nth(self.position).unwrap();

            if c.is_whitespace() {
                self.skip_whitespace();
            } 

            else if c.is_numeric() {
                token = Some(self.read_number());
            } 

            else if c == '"' {
                token = Some(self.read_string());
            } 

            else if c.is_alphabetic() {
                token = Some(self.read_identifier());
            } 

            else if c == '/' && self.peek() == Some('/') {
               self.skip_comment();
            }

            else if OPERATORS.contains(&c.to_string().as_str()) {
                token = Some(self.read_operator());
            }

            else if PUNCTUATORS.contains(&c.to_string().as_str()) {
                token = Some(self.read_punctuator());
            }

            else {
                //create an error with an error type
                token = Some(Token::new(TokenKind::Error(Error::new(ErrorType::InvalidToken,"Invalid character")), self.line, self.column));
            }
        }
        token
    }

    //write a peek function that return the next char
    pub fn peek(&mut self) -> Option<char> {
        if self.position >= self.input.len() {
            return None;
        }

        let c = self.input.chars().nth(self.position).unwrap();
        Some(c)
    }

    fn skip_whitespace(&mut self) {
        let c = self.input.chars().nth(self.position).unwrap();

        if c.is_whitespace() {
            self.position += 1;
            self.column += 1;
        }
    }

    pub fn skip_comment(&mut self) {
        self.position += 2;
        self.column += 2;

        while self.position < self.input.len() {
            let c = self.input.chars().nth(self.position).unwrap();

            if c == '\n' {
                self.position += 1;
                self.column = 1;
                self.line += 1;
                break;
            } else {
                self.position += 1;
                self.column += 1;
            }
        }
    }

    fn read_number(&mut self) -> Token {
        let mut value = String::new();

        while self.position < self.input.len() {
            let c = self.input.chars().nth(self.position).unwrap();

            if c.is_numeric() {
                value.push(c);
                self.position += 1;
                self.column += 1;
            } else {
                break;
            }
        }

        Token::new(
            TokenKind::Number(Number::new(&value)),
            self.line,
            self.column,
        )
    }

    fn read_string(&mut self) -> Token {
        //read string and check if string is closed at the end if not create an error
        let mut value = String::new();

        self.position += 1;
        self.column += 1;

        while self.position < self.input.len() {
            let c = self.input.chars().nth(self.position).unwrap();

            if c == '"' {
                self.position += 1;
                self.column += 1;
                break;
            } else {
                value.push(c);
                self.position += 1;
                self.column += 1;
            }
        }

        //check last char if it is a " if not create an error
        if self.input.chars().nth(self.position - 1).unwrap() != '"' {
            return Token::new(TokenKind::Error(Error::new(ErrorType::InvalidToken,"String not closed")), self.line, self.column);
        }

        Token::new(
            TokenKind::StringLiteral(StringLiteral::new(&value)),
            self.line,
            self.column,
        )
    }

    fn read_identifier(&mut self) -> Token {
        let mut value = String::new();

        while self.position < self.input.len() {
            let c = self.input.chars().nth(self.position).unwrap();

            if c.is_alphabetic() {
                value.push(c);
                self.position += 1;
                self.column += 1;
            } else {
                break;
            }
        }

        Token::new(
            TokenKind::Identifier(Identifier::new(&value)),
            self.line,
            self.column,
        )
    }

    fn read_operator(&mut self) -> Token {
        let mut value = String::new();

        while self.position < self.input.len() {
            let c = self.input.chars().nth(self.position).unwrap();

            if OPERATORS.contains(&c.to_string().as_str()) {
                value.push(c);
                self.position += 1;
                self.column += 1;
            } else {
                break;
            }
        }

        Token::new(
            TokenKind::Operator(Operator::new(&value)),
            self.line,
            self.column,
        )
    }

    fn read_punctuator(&mut self) -> Token {
        let mut value = String::new();

        while self.position < self.input.len() {
            let c = self.input.chars().nth(self.position).unwrap();

            if PUNCTUATORS.contains(&c.to_string().as_str()) {
                value.push(c);
                self.position += 1;
                self.column += 1;
            } else {
                break;
            }
        }

        Token::new(
            TokenKind::Punctuator(Punctuator::new(&value)),
            self.line,
            self.column,
        )
    }
}

//create error struct with a type and a message
#[derive(Debug, PartialEq)]
pub enum ErrorType {
    UnexpectedToken,
    InvalidToken,
    UnexpectedEndOfInput,
}

#[derive(Debug, PartialEq)]
pub struct Error {
    error_type: ErrorType,
    message: String,
}

impl Error {
    pub fn new(error_type: ErrorType, message: &str) -> Self {
        Error {
            error_type,
            message: message.to_string(),
        }
    }
}
