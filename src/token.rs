#[allow(dead_code)]
#[allow(non_camel_case_types)]
#[derive(Copy, Clone)]
/// The type of token
pub enum TokenType {
    /// such as `3`, `4` and so on
    INTEGER_LITERAL,
    /// `true` or `false`
    BOOLEAN_LITERAL,
    /// such as `3.14`
    REAL_LITERAL,
    /// such as 'a','b'
    CHAR_LITERAL,
    /// such as "hello world"
    STRING_LITERAL,
    /// such as `abc`
    IDENTIFIER,
    /// such as `if`, `while`
    KEYWORD,
    /// such as `int`
    TYPE,
    /// such as `+`, `-`, `*`, `<`, `&&`
    OPERATOR,
    /// such as `,`, `;`
    DELIMITER,
    /// end of file
    END_OF_FILE,
    /// other unknown token type
    UNKNOWN,
}

#[allow(dead_code)]
#[derive(Copy, Clone)]
/// The value of token
pub enum TokenValue {
    /// keyword
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

    /// type
    DOUBLE,
    INT,
    CHAR,
    STRING,
    BOOL,

    /// boolean
    TRUE,
    FALSE,

    /// symbols
    /// '('
    LPAREN,
    /// ')'
    RPAREN,
    /// '['
    LBRACK,
    /// '['
    RBRACK,
    /// '{'
    LBRACE,
    /// '}'
    RBRACE,
    /// ','
    COMMA,
    /// ';'
    SEMICOLON,
    /// '='
    ASSIGN,
    /// '&&'
    AND,
    /// '<'
    LT,
    /// '-'
    ADD,
    /// '-'
    SUB,
    /// '*'
    MULTI,
    /// '.'
    DOT,
    /// '!'
    NOT,
    /// other token value
    UNRESERVED,
}

#[derive(Default)]
#[derive(Clone)]
/// The location of token
pub struct TokenLocation {
    file_name_: String,
    line_: i32,
    column_: i32
}

impl TokenLocation {
    pub fn new(file_name: String, line: i32, column: i32) -> Self {
        TokenLocation {
            file_name_: file_name,
            line_: line,
            column_: column,
        }
    }

    pub fn to_string(&self) -> String {
        format!("{}:{}:{}:", self.file_name_, self.line_, self.column_)
    }
}

#[derive(Clone)]
/// Lexical token
pub struct Token {
    type_: TokenType,
    value_: TokenValue,
    location_: TokenLocation,
    name_: String,
    symbol_precedence_: i32,

    int_value_: i32,
    real_value_: f64,
    char_value_: char,
    str_value_: String
}

impl Default for Token {
    fn default() -> Self {
        Token {
            type_: TokenType::IDENTIFIER,
            value_: TokenValue::UNRESERVED,
            location_: Default::default(),
            name_: Default::default(),
            symbol_precedence_: -1,
            int_value_: Default::default(),
            real_value_: Default::default(),
            char_value_: Default::default(),
            str_value_: Default::default(),
        }
    }
}

#[allow(dead_code)]
impl Token {
    /// Default constructor
    pub fn new() -> Self {
        Token {
            ..Default::default()
        }
    }

    /// New one `IDENTIFIER` or `KEYWORD` token.
    pub fn new_token(token_type: TokenType, token_value: TokenValue, loc: TokenLocation, name: String, symbol_precedence: i32) -> Self {
        Token {
            type_: token_type,
            value_: token_value,
            location_: loc,
            name_: name,
            symbol_precedence_: symbol_precedence,
            ..Default::default()
        }
    }

    /// New one `INTEGER_LITERAL` token.
    pub fn new_int_token(loc: TokenLocation, name: String, int_value: i32) -> Self {
        Token {
            type_: TokenType::INTEGER_LITERAL,
            location_: loc,
            name_: name,
            int_value_: int_value,
            ..Default::default()
        }
    }

    /// New one`REAL_LITERAL` token.
    pub fn new_real_token(loc: TokenLocation, name: String, real_value: f64) -> Self {
        Token {
            type_: TokenType::REAL_LITERAL,
            location_: loc,
            name_: name,
            real_value_: real_value,
            ..Default::default()
        }
    }

    /// New one `CHAR_LITERAL` token.
    pub fn new_char_token(loc: TokenLocation, name: String, char_value: char) -> Self {
        Token {
            type_: TokenType::CHAR_LITERAL,
            location_: loc,
            name_: name,
            char_value_: char_value,
            ..Default::default()
        }
    }

    /// New one `STRING_LITERAL` token.
    pub fn new_str_token(loc: TokenLocation, name: String, str_value: String) -> Self {
        Token {
            type_: TokenType::STRING_LITERAL,
            location_: loc,
            name_: name,
            str_value_: str_value,
            ..Default::default()
        }
    }

    pub fn get_token_type(&self) -> TokenType {
        self.type_
    }

    pub fn to_string(&self) -> String {
        format!("{} Token Type: {} Token Name: {}", self.location_.to_string(),
                self.token_type_description(), self.name_)
    }

    fn token_type_description(&self) -> String {
        let buffer = match self.type_ {
            TokenType::INTEGER_LITERAL => "integer",
            TokenType::BOOLEAN_LITERAL => "boolean",
            TokenType::REAL_LITERAL => "real",
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
