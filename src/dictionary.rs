use crate::token::*;
use std::collections::HashMap;

/// Dictionary of token
pub struct Dictionary {
    dictionary_: HashMap<String, (TokenValue, TokenType, i32)>,
}

impl Dictionary {
    /// Get the default dictionary, which has already inserted reserved word.
    pub fn get_dictionary() -> Dictionary {
        let mut dic = Dictionary {
            dictionary_: HashMap::new(),
        };

        dic.dictionary_.insert("=".to_string(), (TokenValue::ASSIGN, TokenType::OPERATOR, 0));
        dic.dictionary_.insert("<".to_string(), (TokenValue::LT, TokenType::OPERATOR, 2));
        dic.dictionary_.insert("+".to_string(), (TokenValue::ADD, TokenType::OPERATOR, 10));
        dic.dictionary_.insert("-".to_string(), (TokenValue::SUB, TokenType::OPERATOR, 10));
        dic.dictionary_.insert("*".to_string(), (TokenValue::MULTI, TokenType::OPERATOR, 20));
        dic.dictionary_.insert("&&".to_string(), (TokenValue::AND, TokenType::OPERATOR, 20));
        dic.dictionary_.insert("!".to_string(), (TokenValue::NOT, TokenType::OPERATOR, 40));
        dic.dictionary_.insert(".".to_string(), (TokenValue::DOT, TokenType::OPERATOR, 60));
        dic.dictionary_.insert("(".to_string(), (TokenValue::LPAREN, TokenType::DELIMITER, -1));
        dic.dictionary_.insert(")".to_string(), (TokenValue::RPAREN, TokenType::DELIMITER, -1));
        dic.dictionary_.insert("[".to_string(), (TokenValue::LBRACK, TokenType::DELIMITER, -1));
        dic.dictionary_.insert("]".to_string(), (TokenValue::RBRACK, TokenType::DELIMITER, -1));
        dic.dictionary_.insert("{".to_string(), (TokenValue::LBRACE, TokenType::DELIMITER, -1));
        dic.dictionary_.insert("}".to_string(), (TokenValue::RBRACE, TokenType::DELIMITER, -1));
        dic.dictionary_.insert(",".to_string(), (TokenValue::COMMA, TokenType::DELIMITER, -1));
        dic.dictionary_.insert(";".to_string(), (TokenValue::SEMICOLON, TokenType::DELIMITER, -1));
        dic.dictionary_.insert("class".to_string(), (TokenValue::CLASS, TokenType::KEYWORD, -1));
        dic.dictionary_.insert("public".to_string(), (TokenValue::PUBLIC, TokenType::KEYWORD, -1));
        dic.dictionary_.insert("static".to_string(), (TokenValue::STATIC, TokenType::KEYWORD, -1));
        dic.dictionary_.insert("void".to_string(), (TokenValue::VOID, TokenType::KEYWORD, -1));
        dic.dictionary_.insert("main".to_string(), (TokenValue::MAIN, TokenType::KEYWORD, -1));
        dic.dictionary_.insert("extends".to_string(), (TokenValue::EXTENDS, TokenType::KEYWORD, -1));
        dic.dictionary_.insert("return".to_string(), (TokenValue::RETURN, TokenType::KEYWORD, -1));
        dic.dictionary_.insert("if".to_string(), (TokenValue::IF, TokenType::KEYWORD, -1));
        dic.dictionary_.insert("else".to_string(), (TokenValue::ELSE, TokenType::KEYWORD, -1));
        dic.dictionary_.insert("while".to_string(), (TokenValue::WHILE, TokenType::KEYWORD, -1));
        dic.dictionary_.insert("for".to_string(), (TokenValue::FOR, TokenType::KEYWORD, -1));
        dic.dictionary_.insert("System.out.println".to_string(), (TokenValue::PRINT, TokenType::KEYWORD, -1));
        dic.dictionary_.insert("length".to_string(), (TokenValue::LENGTH, TokenType::KEYWORD, -1));
        dic.dictionary_.insert("this".to_string(), (TokenValue::THIS, TokenType::KEYWORD, -1));
        dic.dictionary_.insert("new".to_string(), (TokenValue::NEW, TokenType::KEYWORD, -1));
        dic.dictionary_.insert("true".to_string(), (TokenValue::TRUE, TokenType::BOOLEAN_LITERAL, -1));
        dic.dictionary_.insert("false".to_string(), (TokenValue::FALSE, TokenType::BOOLEAN_LITERAL, -1));
        dic.dictionary_.insert("int".to_string(), (TokenValue::INT, TokenType::TYPE, -1));
        dic.dictionary_.insert("char".to_string(), (TokenValue::CHAR, TokenType::TYPE, -1));
        dic.dictionary_.insert("String".to_string(), (TokenValue::STRING, TokenType::TYPE, -1));
        dic.dictionary_.insert("boolean".to_string(), (TokenValue::BOOL, TokenType::TYPE, -1));
        dic
    }

    #[allow(dead_code)]
    /// Add token to dictionary.
    pub fn add_token(&mut self, name: String, info: (TokenValue, TokenType, i32)) {
        self.dictionary_.insert(name, info);
    }

    /// Find out if name exists,and return `(TokenValue, TokenType, precedence)`.
    ///
    /// if so, return the corresponding tuple,
    ///
    /// if not, return the default tuple, which is `(TokenValue::UNRESERVED, TokenType::IDENTIFIER, -1)`.
    pub fn lookup(&self, name: &String) -> (TokenValue, TokenType, i32) {
        let token_value = TokenValue::UNRESERVED;
        let token_type  = TokenType::IDENTIFIER;
        let precedence = -1;

        if self.dictionary_.contains_key(name) {
            let info = self.dictionary_.get(name).unwrap();
            return *info;
        }

        (token_value, token_type, precedence)
    }

    /// Check if name exists.
    pub fn have_token(&self, name: &String) -> bool {
        self.dictionary_.contains_key(name)
    }
}
