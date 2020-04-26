#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(dead_code)]

use crate::token::*;
use crate::dictionary::*;
use std::io::prelude::*;
use std::fs::File;
use std::io::SeekFrom;

enum State {
    NONE = 0,
    END_OF_FILE,
    IDENTIFIER,
    NUMBER,
    CHAR_LITERAL,
    STRING_LITERAL,
    OPERATION,
}

pub struct Scanner {
    fileName_: String,
    file_: File,
    line_: i32,
    column_: i32,
    loc_: TokenLocation,
    currentChar_: char,
    state_: State,
    token_: Token,
    dictionary_: Dictionary,
    buffer_: String,
    eofFlag_: bool,
    errorFlag_: bool,
}

impl Scanner {
    pub fn new(fileName: String) -> Self {
        let file = match File::open(fileName.clone()) {
            Err(err) => panic!("When trying to open file {}, because {}, an occurred error.", err.to_string(), fileName),
            Ok(file) => file,
        };

        Scanner {
            fileName_: fileName.to_owned(),
            file_: file,
            line_: 1,
            column_: 0,
            loc_: TokenLocation::new(fileName, 1, 0),
            currentChar_: Default::default(),
            state_: State::NONE,
            token_: Default::default(),
            dictionary_: Dictionary::getDictionary(),
            buffer_: Default::default(),
            eofFlag_: false,
            errorFlag_: false,
        }
    }

    pub fn getTokenLocation(&self) -> TokenLocation {
        TokenLocation::new(self.fileName_.to_owned(), self.line_, self.column_)
    }

    fn makeToken(&mut self, tokenType: TokenType, tokenValue: TokenValue, loc: TokenLocation, name: String, symbolPrecedence: i32) {
        self.token_ = Token::newToken(tokenType, tokenValue, loc, name, symbolPrecedence);
        self.buffer_.clear();
        self.state_ = State::NONE;
    }

    fn makeIntToken(&mut self, loc: TokenLocation, name: String, intValue: i32) {
        self.token_ = Token::newIntToken(loc, name, intValue);
        self.buffer_.clear();
        self.state_ = State::NONE;
    }

    fn makeRealToken(&mut self, loc: TokenLocation, name: String, realValue: f64) {
        self.token_ = Token::newRealToken(loc, name, realValue);
        self.buffer_.clear();
        self.state_ = State::NONE;
    }

    fn makeCharToken(&mut self, loc: TokenLocation, name: String, charValue: char) {
        self.token_ = Token::newCharToken(loc, name, charValue);
        self.buffer_.clear();
        self.state_ = State::NONE;
    }

    fn makeStrToken(&mut self, loc: TokenLocation, name: String, strValue: String) {
        self.token_ = Token::newStrToken(loc, name, strValue);
        self.buffer_.clear();
        self.state_ = State::NONE;
    }

    fn errorToken(&mut self, msg: &String) {
        eprintln!("Token Error:{}", msg);
        self.errorFlag_ = true;
    }

    fn getNextChar(&mut self) {
        let mut buffer = [0; 1];
        match self.file_.read_exact(&mut buffer) {
            Err(_e) => self.eofFlag_ = true,
            Ok(()) => self.currentChar_ = buffer[0].into(),
        }

        if self.currentChar_ == '\n' {
            self.line_ = self.line_ + 1;
            self.column_ = 0;
        } else {
            self.column_ = self.column_ + 1;
        }
    }

    fn getPeekChar(&mut self) -> char {
        let mut buffer = [0; 1];
        match self.file_.read_exact(&mut buffer) {
            Err(_e) => self.eofFlag_ = true,
            Ok(()) => self.currentChar_ = buffer[0].into(),
        };
        self.file_.seek(SeekFrom::Current(-1)).unwrap();
        self.currentChar_
    }

    fn addToBuffer(&mut self, ch: char) {
        self.buffer_.push(ch);
    }

    fn reduceBuffer(&mut self) {
        self.buffer_.pop();
    }

    fn errorReport(&mut self, msg: String) {
        eprintln!("Token Error: {}{}", self.getTokenLocation().toString(), msg);
        self.errorFlag_ = true;
    }

    fn handleLineComment(&mut self) {
        self.loc_ = self.getTokenLocation();

        if self.currentChar_ == '/' && self.getPeekChar() == '/' {
            self.getNextChar();
            self.getNextChar();

            while self.currentChar_ != '\n' && !self.eofFlag_ {
                self.getNextChar();
            }

            if !self.eofFlag_ {
                self.getNextChar();
            }
        }
    }

    fn handleBlockComment(&mut self) {
        self.loc_ = self.getTokenLocation();

        if self.currentChar_ == '/' && self.getPeekChar() == '*' {
            self.getNextChar();
            self.getNextChar();

            while !(self.currentChar_ == '*' && self.getPeekChar() == '/') {
                if self.eofFlag_ {
                    self.errorReport(format!("end of file happended in comment, */ is expected!, but find {}", self.currentChar_));
                }

                self.getNextChar();
            }

            if !self.eofFlag_ {
                self.getNextChar();
                self.getNextChar();
            }
        }
    }

    fn preprocess(&mut self) {
        loop {
            while self.currentChar_.is_ascii_whitespace() {
                self.getNextChar();
            }

            self.handleLineComment();
            self.handleBlockComment();

            if !(self.currentChar_.is_ascii_whitespace() || self.currentChar_ == '/') {
                break;
            }
        }
    }

    pub fn getToken(&self) -> Token {
        self.token_.to_owned()
    }

    pub fn getNextToken(&mut self) -> Token {
        let mut matched = false;

        loop {
            self.errorFlag_ = false;

            match self.state_ {
                State::NONE => matched = false,
                _ => matched = true,
            }

            match self.state_ {
                State::NONE => self.getNextChar(),
                State::END_OF_FILE => self.handleEOFState(),
                State::IDENTIFIER => self.handleIdentifierState(),
                State::NUMBER => self.handleNumberState(),
                State::CHAR_LITERAL => self.handleCharState(),
                State::STRING_LITERAL => self.handleStringState(),
                State::OPERATION => self.handleOperationState(),
            }

            match self.state_ {
                State::NONE => {
                    self.preprocess();

                    if self.eofFlag_ {
                        self.state_ = State::END_OF_FILE;
                    } else {
                        if self.currentChar_.is_ascii_alphabetic() {
                            self.state_ = State::IDENTIFIER;
                        } else if self.currentChar_.is_ascii_digit() {
                            self.state_ = State::NUMBER;
                        } else if self.currentChar_ == '\'' {
                            self.state_ = State::CHAR_LITERAL;
                        } else if self.currentChar_ == '\"' {
                            self.state_ = State::STRING_LITERAL;
                        } else {
                            self.state_ = State::OPERATION;
                        }
                    }
                },
                _ => {},
            }

            if matched && !self.errorFlag_ {
                break;
            }
        }

        self.token_.to_owned()
    }

    fn handleEOFState(&mut self) {
        self.loc_ = self.getTokenLocation();
        self.makeToken(TokenType::END_OF_FILE, TokenValue::UNRESERVED, self.loc_.to_owned(), "END_OF_FILE".to_string(), -1);
    }

    fn handleDigit(&mut self) {
        println!("current char: {}", self.currentChar_);
        self.addToBuffer(self.currentChar_);
        self.getNextChar();

        while self.currentChar_.is_ascii_digit() {
            self.addToBuffer(self.currentChar_);
            self.getNextChar();
        }

        println!("literal: {}", self.buffer_);
    }

    fn handleXDigit(&mut self) {
        let mut readFlag = false;

        while self.currentChar_.is_ascii_hexdigit() {
            readFlag = true;
            self.addToBuffer(self.currentChar_);
            self.getNextChar();
        }

        if !readFlag {
            self.errorReport("Hexadecimal number format error.".to_string());
        }
    }

    fn handleODigit(&mut self) {
        let mut readFlag = false;

        while self.currentChar_ >= '0' && self.currentChar_ <= '7' {
            readFlag = true;
            self.addToBuffer(self.currentChar_);
            self.getNextChar();
        }

        if !readFlag
        {
            self.errorReport("Octal number format error.".to_string());
        }
    }

    fn handleFraction(&mut self) {
        if !self.currentChar_.is_ascii_digit() {
            self.errorReport("Fraction number part should be numbers".to_string());
        }

        self.addToBuffer(self.currentChar_);
        self.getNextChar();

        while self.currentChar_.is_ascii_digit() {
            self.addToBuffer(self.currentChar_);
            self.getNextChar();
        }
    }

    fn handleExponent(&mut self) {
        self.addToBuffer(self.currentChar_);
        self.getNextChar();

        while self.currentChar_ != '+' && self.currentChar_ != '-' && !self.currentChar_.is_ascii_digit() {
            self.errorReport(format!("Scientist presentation number after e / E should be + / - or digits but find
                        \'{}\'", self.currentChar_));
        }

        if self.currentChar_ == '+' || self.currentChar_ == '-' {
            self.addToBuffer(self.currentChar_);
            self.getNextChar();
        }
    }


    fn handleNumberState(&mut self) {
        println!("number: {}", self.currentChar_);
        self.loc_ = self.getTokenLocation();

        let mut isFloat = false;
        let mut isExponent = false;

        let mut numberBase = 10;

        if self.currentChar_ == '0' && (self.getPeekChar() == 'x' || self.getPeekChar() == 'X') {
            numberBase = 16;

            self.getNextChar();
            self.getNextChar();
        }

        if self.currentChar_ == '0' && self.getPeekChar() >= '0' && self.getPeekChar() <= '7' {
            numberBase = 8;

            self.getNextChar();
        }

        enum NumberState {
            INTERGER,
            FRACTION,
            EXPONENT,
            DONE
        }

        let mut numberState = NumberState::INTERGER;

        loop {
            match numberState {
                NumberState::INTERGER => {
                    if numberBase == 10 {
                        self.handleDigit();
                    } else if numberBase == 16 {
                        self.handleXDigit();
                    } else if numberBase == 8 {
                        self.handleODigit();
                    }
                },
                NumberState::FRACTION => {
                    self.handleFraction();
                    isFloat = true;
                },
                NumberState::EXPONENT => {
                    self.handleExponent();
                    isExponent = true;
                },
                NumberState::DONE => {},
            }

            if self.currentChar_ == '.' {
                if isFloat {
                    self.errorReport("Fraction number can not have more than one dot.".to_string());
                }

                if isExponent {
                    self.errorReport("Scientist number representation in MJava can not have dot.".to_string());
                }

                if numberBase == 16 {
                    self.errorReport("Hexadecimal number in MJava can only be integer.".to_string());
                }

                if numberBase == 8 {
                    self.errorReport("Octal number in MJava can only be integer.".to_string());
                }

                numberState = NumberState::FRACTION;
            } else if self.currentChar_ == 'E' || self.currentChar_ == 'e' {
                if isExponent {
                    self.errorReport("Scientist presentation can not have more than one e / E".to_string());
                }

                numberState = NumberState::EXPONENT;
            } else {
                numberState = NumberState::DONE;
            }

            match numberState {
                NumberState::DONE => break,
                _ => {},
            }
        }

        if !self.errorFlag_ {
            if isFloat || isExponent {
                let realValue: f64 = match self.buffer_.parse::<f64>() {
                    Err(err) => {
                        self.errorReport(format!("When parse floating-point number literal \"{}\", because {}, an error
                                    occurred.", self.buffer_, err.to_string()));
                        0.0
                    },
                    Ok(realValue) => realValue,
                };

                self.makeRealToken(self.loc_.to_owned(), self.buffer_.to_owned(), realValue);
            } else {
                let intValue: i32 = match i32::from_str_radix(&self.buffer_.clone(), numberBase) {
                    Err(err) => {
                        self.errorReport(format!("When parse integer literal \"{}\", because {}, an error occurred.", self.buffer_,
                                err.to_string()));
                        0
                    },
                    Ok(intValue) => intValue,
                };

                self.makeIntToken(self.loc_.to_owned(), self.buffer_.to_owned(), intValue);
            }
        } else {
            self.buffer_.clear();
            self.state_ = State::NONE;
        }
    }

    fn handleCharState(&mut self) {
        self.loc_ = self.getTokenLocation();

        self.getNextChar();

        loop {
            if self.currentChar_ != '\\' && self.getPeekChar() == '\'' {
                self.addToBuffer(self.currentChar_);
                break;
            }

            if self.eofFlag_ {
                self.errorReport(format!("end of file happended in string, \' is expected!, but find {}",
                            self.currentChar_));
                break;
            }

            self.addToBuffer(self.currentChar_);
            self.getNextChar();
        }

        if !self.eofFlag_ {
            self.getNextChar();
            self.getNextChar();
        }

        if !self.errorFlag_ && self.buffer_.len() == 1 {
            let ch = self.buffer_.chars().next().unwrap();
            self.makeCharToken(self.loc_.to_owned(), self.buffer_.clone(), ch);
        } else {
            self.errorReport("Char can contain only one character!".to_string());
            self.buffer_.clear();
            self.state_ = State::NONE;
        }
    }

    fn handleStringState(&mut self) {
        self.loc_ = self.getTokenLocation();

        self.getNextChar();

        loop {
            if self.eofFlag_ {
                self.errorReport(format!("end of file happended in string, \" is expected!, but find {}",
                            self.currentChar_));
                break;
            }

            if self.currentChar_ != '\\' && self.getPeekChar() == '\"' {
                self.addToBuffer(self.currentChar_);
                break;
            }

            self.addToBuffer(self.currentChar_);
            self.getNextChar();
        }

        if !self.eofFlag_ {
            self.getNextChar();
            self.getNextChar();
        }

        if !self.errorFlag_ {
            self.makeStrToken(self.loc_.to_owned(), self.buffer_.clone(), self.buffer_.clone());
        } else {
            self.buffer_.clear();
            self.state_ = State::NONE;
        }
    }

    fn handleIdentifierState(&mut self) {
        self.loc_ = self.getTokenLocation();

        self.addToBuffer(self.currentChar_);
        self.getNextChar();

        while self.currentChar_.is_ascii_alphanumeric() || self.currentChar_ == '_' {
            self.addToBuffer(self.currentChar_);
            self.getNextChar();
        }

        let copy = self.buffer_.clone();

        if self.buffer_.eq("System") {
            let mut length = 12;

            while length > 0 && !self.eofFlag_ {
                self.addToBuffer(self.currentChar_);
                self.getNextChar();
                length = length - 1;
            }

            if !self.buffer_.eq("System.out.println") {
                self.buffer_ = copy;
                self.file_.seek(SeekFrom::Current(-length)).unwrap();
            }
        }

        let (tokenValue, tokenType, precedence) = self.dictionary_.lookup(&self.buffer_);

        self.makeToken(tokenType, tokenValue, self.loc_.to_owned(), self.buffer_.to_owned(), precedence);
    }

    fn handleOperationState(&mut self) {
        self.loc_ = self.getTokenLocation();

        self.addToBuffer(self.currentChar_);

        let ch = self.getPeekChar();
        self.addToBuffer(ch);

        if self.dictionary_.haveToken(&self.buffer_) {
            self.getNextChar();
        } else {
            self.reduceBuffer();
        }

        let (tokenValue, tokenType, precedence) = self.dictionary_.lookup(&self.buffer_);
        self.makeToken(tokenType, tokenValue, self.loc_.to_owned(), self.buffer_.to_owned(), precedence);
        self.getNextChar();
    }
}
