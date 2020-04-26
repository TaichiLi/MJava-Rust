#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

#[allow(dead_code)]
#[derive(Copy, Clone)]
pub enum TokenType {
    // in fact, we can put these 5 types to one category
    // named constant. but I want to make it cleaner.
    INTEGER,        // such as 3, 4 and so on
    BOOLEAN,        // true or false.
    REAL,           // such as 3.14
    CHAR_LITERAL,   // such as 'a','b'
    STRING_LITERAL, // such as "hello world"

    IDENTIFIER,     // such as abc
    KEYWORD,       // such as if
    TYPE,           // such as int
    OPERATOR,      // such as  + - * /
    DELIMITER,      // such as ,
    END_OF_FILE,    // end of file
    UNKNOWN,
}

#[allow(dead_code)]
#[derive(Copy, Clone)]
pub enum TokenValue {
    // keyword
    CLASS = 0,
    PUBLIC,
    STATIC,
    VOID,
    MAIN,
    EXTENDS,
    RETURN,
    IF,
    ELSE,
    WHILE,
    FOR,
    PRINT,
    LENGTH,
    THIS,
    NEW,

    // type
    DOUBLE,
    INT,
    CHAR,
    STRING,
    BOOL,

    // boolean
    TRUE,
    FALSE,

    // symbols
    LPAREN,             // (
    RPAREN,             // )
    LBRACK,             // [
    RBRACK,             // ]
    LBRACE,             // {
    RBRACE,             // }
    COMMA,              // ,
    SEMICOLON,          // ;
    ASSIGN,             // =
    AND,                // &&
    LT,                 // <
    ADD,                // +
    SUB,                // -
    MULTI,              // *
    DOT,                // .
    NOT,                // !
    UNRESERVED,
}

#[derive(Default)]
#[derive(Clone)]
pub struct TokenLocation {
    fileName_: String,
    line_: i32,
    column_: i32
}

impl TokenLocation {
    pub fn new(fileName: String, line: i32, column: i32) -> TokenLocation {
        TokenLocation {
            fileName_: fileName,
            line_: line,
            column_: column,
        }
    }

    pub fn toString(&self) -> String {
        format!("{}:{}:{}:", self.fileName_, self.line_, self.column_)
    }
}

#[derive(Clone)]
pub struct Token {
    type_: TokenType,
    value_: TokenValue,
    location_: TokenLocation,
    name_: String,
    symbolPrecedence_: i32,

    intValue_: i32,
    realValue_: f64,
    charValue_: char,
    strValue_: String
}

impl Default for Token {
    fn default() -> Self {
        Token {
            type_: TokenType::IDENTIFIER,
            value_: TokenValue::UNRESERVED,
            location_: Default::default(),
            name_: Default::default(),
            symbolPrecedence_: -1,
            intValue_: Default::default(),
            realValue_: Default::default(),
            charValue_: Default::default(),
            strValue_: Default::default(),
        }
    }
}

#[allow(dead_code)]
impl Token {
    pub fn new() -> Self {
        Token {
            ..Default::default()
        }
    }

    pub fn newToken(tokenType: TokenType, tokenValue: TokenValue, loc: TokenLocation, name: String, symbolPrecedence: i32) -> Self {
        Token {
            type_: tokenType,
            value_: tokenValue,
            location_: loc,
            name_: name,
            symbolPrecedence_: symbolPrecedence,
            ..Default::default()
        }
    }

    pub fn newIntToken(loc: TokenLocation, name: String, intValue: i32) -> Self {
        Token {
            type_: TokenType::INTEGER,
            location_: loc,
            name_: name,
            intValue_: intValue,
            ..Default::default()
        }
    }

    pub fn newRealToken(loc: TokenLocation, name: String, realValue: f64) -> Self {
        Token {
            type_: TokenType::REAL,
            location_: loc,
            name_: name,
            realValue_: realValue,
            ..Default::default()
        }
    }

    pub fn newCharToken(loc: TokenLocation, name: String, charValue: char) -> Self {
        Token {
            type_: TokenType::CHAR_LITERAL,
            location_: loc,
            name_: name,
            charValue_: charValue,
            ..Default::default()
        }
    }

    pub fn newStrToken(loc: TokenLocation, name: String, strValue: String) -> Self {
        Token {
            type_: TokenType::STRING_LITERAL,
            location_: loc,
            name_: name,
            strValue_: strValue,
            ..Default::default()
        }
    }

    pub fn getTokenType(&self) -> TokenType {
        self.type_
    }

    pub fn toString(&self) -> String {
        format!("{} Token Type: {} Token Name: {}", self.location_.toString(),
                self.tokenTypeDescription(), self.name_)
    }

    fn tokenTypeDescription(&self) -> String {
        let buffer = match self.type_ {
            TokenType::INTEGER => "integer",
            TokenType::BOOLEAN => "boolean",
            TokenType::REAL => "real",
            TokenType::CHAR_LITERAL => "char literal",
            TokenType::STRING_LITERAL => "string literal",
            TokenType::IDENTIFIER => "identifier",
            TokenType::KEYWORD => "keyword",
            TokenType::TYPE => "type",
            TokenType::OPERATOR => "operator",
            TokenType::DELIMITER => "delimiter",
            TokenType::END_OF_FILE => "eof",
            TokenType::UNKNOWN => "unknown",
        };
        buffer.to_string()
    }
}
