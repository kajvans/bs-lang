//import fs
//ignore unused imports for now
#[allow(unused_imports)]
use std::fs;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tokens() {
        //write a really long string
        let mut tokenizer =
            Tokenizer::new("let x: float = 10 + 20.1; let y: int = 30 + 40; let z: float = x + y; \n let x: string = \"hello world\"; let r: bool = true;");
        let mut tokens = Vec::new();

        while let Some(token) = tokenizer.next() {
            tokens.push(token);
        }

        //write to file for debugging
        fs::write("tokens.txt", format!("{:#?}", tokens)).expect("Unable to write file");

        //check all the tokens
        assert_eq!(tokens[0].kind, TokenKind::Keyword(Keyword::new("let")));
        assert_eq!(
            tokens[1].kind,
            TokenKind::Identifier(Identifier::new("x", Type::new("float")))
        );
        assert_eq!(tokens[2].kind, TokenKind::Punctuator(Punctuator::new("=")));
        assert_eq!(tokens[3].kind, TokenKind::IntLiteral(IntLiteral::new("10")));
        assert_eq!(tokens[4].kind, TokenKind::Operator(Operator::new("+")));
        assert_eq!(
            tokens[5].kind,
            TokenKind::FloatLitteral(FloatLitteral::new("20.1"))
        );
        assert_eq!(tokens[6].kind, TokenKind::Punctuator(Punctuator::new(";")));
        assert_eq!(tokens[7].kind, TokenKind::Keyword(Keyword::new("let")));
        assert_eq!(
            tokens[8].kind,
            TokenKind::Identifier(Identifier::new("y", Type::new("int")))
        );
        assert_eq!(tokens[9].kind, TokenKind::Punctuator(Punctuator::new("=")));
        assert_eq!(tokens[10].kind, TokenKind::IntLiteral(IntLiteral::new("30")));
        assert_eq!(tokens[11].kind, TokenKind::Operator(Operator::new("+")));
        assert_eq!(
            tokens[12].kind,
            TokenKind::IntLiteral(IntLiteral::new("40"))
        );
        assert_eq!(tokens[13].kind, TokenKind::Punctuator(Punctuator::new(";")));
        assert_eq!(tokens[14].kind, TokenKind::Keyword(Keyword::new("let")));
        assert_eq!(
            tokens[15].kind,
            TokenKind::Identifier(Identifier::new("z", Type::new("float")))
        );
        assert_eq!(tokens[16].kind, TokenKind::Punctuator(Punctuator::new("=")));
        assert_eq!(
            tokens[17].kind,
            TokenKind::Identifier(Identifier::new("x", Type::new("")))
        );
        assert_eq!(tokens[18].kind, TokenKind::Operator(Operator::new("+")));
        assert_eq!(
            tokens[19].kind,
            TokenKind::Identifier(Identifier::new("y", Type::new("")))
        );
        assert_eq!(tokens[20].kind, TokenKind::Punctuator(Punctuator::new(";")));
        assert_eq!(tokens[21].kind, TokenKind::Keyword(Keyword::new("let")));
        assert_eq!(
            tokens[22].kind,
            TokenKind::Identifier(Identifier::new("x", Type::new("string")))
        );
        assert_eq!(tokens[23].kind, TokenKind::Punctuator(Punctuator::new("=")));
        assert_eq!(
            tokens[24].kind,
            TokenKind::StringLiteral(StringLiteral::new("hello world"))
        );
        assert_eq!(tokens[25].kind, TokenKind::Punctuator(Punctuator::new(";")));
        assert_eq!(tokens[26].kind, TokenKind::Keyword(Keyword::new("let")));
        assert_eq!(
            tokens[27].kind,
            TokenKind::Identifier(Identifier::new("r", Type::new("bool")))
        );
        assert_eq!(tokens[28].kind, TokenKind::Punctuator(Punctuator::new("=")));
        assert_eq!(
            tokens[29].kind,
            TokenKind::BoolLiteral(BoolLiteral::new("true"))
        );
    }

    #[test]
    fn int() {
        let mut tokenizer = Tokenizer::new("10");
        let mut tokens = Vec::new();

        while let Some(token) = tokenizer.next() {
            tokens.push(token);
        }

        assert_eq!(tokens[0].kind, TokenKind::IntLiteral(IntLiteral::new("10")));
    }

    #[test]
    fn float() {
        let mut tokenizer = Tokenizer::new("10.1");
        let mut tokens = Vec::new();

        while let Some(token) = tokenizer.next() {
            tokens.push(token);
        }

        assert_eq!(
            tokens[0].kind,
            TokenKind::FloatLitteral(FloatLitteral::new("10.1"))
        );
    }

    #[test]
    fn string() {
        let mut tokenizer = Tokenizer::new("\"Hello World\"");
        let mut tokens = Vec::new();

        while let Some(token) = tokenizer.next() {
            tokens.push(token);
        }

        assert_eq!(
            tokens[0].kind,
            TokenKind::StringLiteral(StringLiteral::new("Hello World"))
        );
    }

    #[test]
    fn keyword() {
        let mut tokenizer = Tokenizer::new("let");
        let mut tokens = Vec::new();

        while let Some(token) = tokenizer.next() {
            tokens.push(token);
        }

        assert_eq!(tokens[0].kind, TokenKind::Keyword(Keyword::new("let")));
    }

    #[test]
    fn identifier() {
        let mut tokenizer = Tokenizer::new("x: int");
        let mut tokens = Vec::new();

        while let Some(token) = tokenizer.next() {
            tokens.push(token);
        }

        assert_eq!(
            tokens[0].kind,
            TokenKind::Identifier(Identifier::new("x", Type::new("int"))),
        );
    }

    #[test]
    fn operator() {
        let mut tokenizer = Tokenizer::new("+");
        let mut tokens = Vec::new();

        while let Some(token) = tokenizer.next() {
            tokens.push(token);
        }

        assert_eq!(tokens[0].kind, TokenKind::Operator(Operator::new("+")));
    }

    #[test]
    fn punctuator() {
        let mut tokenizer = Tokenizer::new(";");
        let mut tokens = Vec::new();

        while let Some(token) = tokenizer.next() {
            tokens.push(token);
        }

        assert_eq!(tokens[0].kind, TokenKind::Punctuator(Punctuator::new(";")));
    }

    #[test]
    fn next_line() {
        let mut tokenizer = Tokenizer::new("\n \"hello world\"");
        let mut tokens = Vec::new();

        while let Some(token) = tokenizer.next() {
            tokens.push(token);
        }

        //position should be line 2
        assert_eq!(tokens[0].position.line, 2);
    }
}

//create a list of all operators
const OPERATORS: [&str; 16] = [
    "+", "-", "*", "/", "%", "++", "--", "==", "!=", "<", ">", "<=", ">=", "&&", "||", "!",
];

//create a list of all punctuators
const PUNCTUATORS: [&str; 10] = ["(", ")", "{", "}", "[", "]", ",", ";", ".", "="];

//create list of all keywords
const KEYWORDS: [&str; 3] = ["let", "const", "func"];

const TYPES: [&str; 4] = ["float", "int", "string", "bool"];

#[derive(Debug, Clone, PartialEq)]
pub struct Operator(pub String);

impl Operator {
    pub fn new(value: &str) -> Self {
        Operator(value.to_string())
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct BoolLiteral(pub bool);

impl BoolLiteral {
    pub fn new(value: &str) -> Self {
        BoolLiteral(value.parse::<bool>().unwrap())
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Punctuator(pub String);

impl Punctuator {
    pub fn new(value: &str) -> Self {
        Punctuator(value.to_string())
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct FloatLitteral(pub f64);

impl FloatLitteral {
    pub fn new(value: &str) -> Self {
        FloatLitteral(value.parse::<f64>().unwrap())
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct IntLiteral(pub i64);

impl IntLiteral {
    pub fn new(value: &str) -> Self {
        IntLiteral(value.parse::<i64>().unwrap())
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct StringLiteral(pub String);

impl StringLiteral {
    pub fn new(value: &str) -> Self {
        StringLiteral(value.to_string())
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Type(pub String);

impl Type {
    pub fn new(value: &str) -> Self {
        Type(value.to_string())
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Identifier(pub String, pub Type);

impl Identifier {
    pub fn new(value: &str, type_: Type) -> Self {
        Identifier(value.to_string(), type_)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Keyword(pub String);

impl Keyword {
    pub fn new(value: &str) -> Self {
        Keyword(value.to_string())
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Token {
    pub kind: TokenKind,
    pub position: Position,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Position {
    pub line: usize,
    pub start_column: usize,
    pub end_column: usize,
}

impl Position {
    pub fn new(line: usize, start_column: usize, end_column: usize) -> Self {
        Position {
            line,
            start_column,
            end_column,
        }
    }
}

impl Token {
    pub fn new(kind: TokenKind, line: usize, start_column: usize, end_column: usize) -> Self {
        Token {
            kind,
            position: Position::new(line, start_column, end_column),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum TokenKind {
    Operator(Operator),
    Punctuator(Punctuator),
    BoolLiteral(BoolLiteral),
    FloatLitteral(FloatLitteral),
    IntLiteral(IntLiteral),
    StringLiteral(StringLiteral),
    Identifier(Identifier),
    Type(Type),
    Keyword(Keyword),
    Error(Error),
}

#[derive(Debug, Clone, PartialEq)]
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
        //start column is the current column
        let start_column = self.column;

        if self.position >= self.input.len() {
            return None;
        }

        let mut token = None;

        while token.is_none() {
            let c = self.input.chars().nth(self.position).unwrap();

            if c == '\n' {
                self.line += 1;
                self.column = 1;
                self.position += 1;
            } else if c.is_whitespace() {
                self.skip_whitespace();
            } else if c.is_numeric() {
                token = Some(self.read_number());
            } else if c == '"' {
                token = Some(self.read_string());
            } else if c.is_alphabetic() {
                token = Some(self.read_identifier());
            } else if c == '/' && self.peek() == Some('/') {
                self.skip_comment();
            } else if OPERATORS.contains(&c.to_string().as_str()) {
                token = Some(self.read_operator());
            } else if PUNCTUATORS.contains(&c.to_string().as_str()) {
                token = Some(self.read_punctuator());
            }
            //check for a new
            else {
                //create an error with an error type
                token = Some(Token::new(
                    TokenKind::Error(Error::new(
                        ErrorType::InvalidToken,
                        "Invalid token",
                        &c.to_string().as_str(),
                    )),
                    self.line,
                    start_column,
                    self.column,
                ));
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
        //read number and check if it is a float or int
        let mut value = String::new();

        while self.position < self.input.len() {
            let c = self.input.chars().nth(self.position).unwrap();

            if c.is_numeric() {
                value.push(c);
                self.position += 1;
                self.column += 1;
            } else if c == '.' {
                value.push(c);
                self.position += 1;
                self.column += 1;
                break;
            } else {
                break;
            }
        }

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

        if value.contains('.') {
            Token::new(
                TokenKind::FloatLitteral(FloatLitteral::new(value.as_str())),
                self.line,
                self.column - value.len(),
                self.column,
            )
        } else {
            Token::new(
                TokenKind::IntLiteral(IntLiteral::new(value.as_str())),
                self.line,
                self.column - value.len(),
                self.column,
            )
        }
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
            //create an error with an error type
            return Token::new(
                TokenKind::Error(Error::new(ErrorType::InvalidToken, "Invalid token", &value)),
                self.line,
                self.column,
                self.column,
            );
        }

        Token::new(
            TokenKind::StringLiteral(StringLiteral::new(value.as_str())),
            self.line,
            self.column - value.len(),
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

        // Check if the next character is a colon, indicating a type is specified
        if let Some(':') = self.input.chars().nth(self.position) {
            self.position += 1;
            self.column += 1;

            // Skip over any whitespace characters before the type name
            while self.position < self.input.len() {
                let c = self.input.chars().nth(self.position).unwrap();

                if c.is_whitespace() {
                    self.position += 1;
                    self.column += 1;
                } else {
                    break;
                }
            }

            let mut type_name = String::new();

            while self.position < self.input.len() {
                let c = self.input.chars().nth(self.position).unwrap();

                if c.is_alphabetic() {
                    type_name.push(c);
                    self.position += 1;
                    self.column += 1;
                } else {
                    break;
                }
            }

            if TYPES.contains(&type_name.as_str()) {
                Token::new(
                    TokenKind::Identifier(Identifier::new(&value, Type::new(&type_name))),
                    self.line,
                    self.column - value.len(),
                    self.column,
                )
            } else {
                //make a &str from value and type_name
                let error = value.to_string() + ":" + &type_name;
                Token::new(
                    TokenKind::Error(Error::new(
                        ErrorType::MissingType,
                        "Missing type",
                        error.as_str(),
                    )),
                    self.line,
                    self.column - type_name.len(),
                    self.column,
                )
            }
        } else if KEYWORDS.contains(&value.as_str()) {
            Token::new(
                TokenKind::Keyword(Keyword::new(&value)),
                self.line,
                self.column - value.len(),
                self.column,
            )
        }

        else if &value == "true" || &value == "false" {
            Token::new(
                TokenKind::BoolLiteral(BoolLiteral::new(&value)),
                self.line,
                self.column - value.len(),
                self.column,
            )
        }
        
        else {
            Token::new(
                TokenKind::Identifier(Identifier::new(&value, Type::new(""))),
                self.line,
                self.column - value.len(),
                self.column,
            )
        }
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
            self.column - value.len(),
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
            self.column - value.len(),
            self.column,
        )
    }
}

//create error struct with a type and a message
#[derive(Debug, Clone, PartialEq)]
pub enum ErrorType {
    UnexpectedToken,
    InvalidToken,
    UnexpectedEndOfInput,
    MissingType,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Error {
    error_type: ErrorType,
    message: String,
    errorstring: String,
}

impl Error {
    pub fn new(error_type: ErrorType, message: &str, error: &str) -> Error {
        Error {
            error_type,
            message: message.to_string(),
            errorstring: error.to_string(),
        }
    }
}
