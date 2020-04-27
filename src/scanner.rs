use crate::token::*;
use crate::dictionary::*;
use std::io::prelude::*;
use std::fs::File;
use std::io::SeekFrom;

#[allow(non_camel_case_types)]
/// The state of lexical analysis.
enum State {
    /// original state
    NONE,
    /// end of file
    END_OF_FILE,
    /// parse `TokenType::IDENTIFIER`
    IDENTIFIER,
    /// parse `TokenLocation::INTERGER_LITERAL` or `TokenType::REAL_LITERAL`
    NUMBER,
    /// parse `TokenType::CHAR_LITERAL`
    CHAR_LITERAL,
    /// parse `TokenType::STRING_LITERAL`
    STRING_LITERAL,
    /// parse `TokenType::OPERATION` or `TokenType::DELIMITER`
    OPERATION,
}


/// Lexical scanner
pub struct Scanner {
    file_name_: String,
    file_: File,
    line_: i32,
    column_: i32,
    loc_: TokenLocation,
    current_char_: char,
    state_: State,
    token_: Token,
    dictionary_: Dictionary,
    buffer_: String,
    eof_flag_: bool,
    error_flag_: bool,
}

impl Scanner {
    /// New scanner by the name of source file.
    ///
    /// # Examples
    /// ```
    /// let source_file_name = "./test.mjava";
    /// let mut scanner = Scanner::new(source_file_name);
    /// ```
    pub fn new(file_name: String) -> Self {
        let file = match File::open(file_name.to_owned()) {
            Err(err) => panic!("When trying to open file {}, because {}, an error occurred.", err.to_string(), &file_name),
            Ok(file) => file,
        };

        Scanner {
            file_name_: file_name.to_owned(),
            file_: file,
            line_: 1,
            column_: 0,
            loc_: TokenLocation::new(file_name, 1, 0),
            current_char_: Default::default(),
            state_: State::NONE,
            token_: Default::default(),
            dictionary_: Dictionary::get_dictionary(),
            buffer_: Default::default(),
            eof_flag_: false,
            error_flag_: false,
        }
    }

    fn get_token_location(&self) -> TokenLocation {
        TokenLocation::new(self.file_name_.to_owned(), self.line_, self.column_)
    }

    fn make_token(&mut self, token_type: TokenType, token_value: TokenValue, loc: TokenLocation, name: String, symbol_precedence: i32) {
        self.token_ = Token::new_token(token_type, token_value, loc, name, symbol_precedence);
        self.buffer_.clear();
        self.state_ = State::NONE;
    }

    fn make_int_token(&mut self, loc: TokenLocation, name: String, int_value: i32) {
        self.token_ = Token::new_int_token(loc, name, int_value);
        self.buffer_.clear();
        self.state_ = State::NONE;
    }

    fn make_real_token(&mut self, loc: TokenLocation, name: String, real_value: f64) {
        self.token_ = Token::new_real_token(loc, name, real_value);
        self.buffer_.clear();
        self.state_ = State::NONE;
    }

    fn make_char_token(&mut self, loc: TokenLocation, name: String, char_value: char) {
        self.token_ = Token::new_char_token(loc, name, char_value);
        self.buffer_.clear();
        self.state_ = State::NONE;
    }

    fn make_str_token(&mut self, loc: TokenLocation, name: String, str_value: String) {
        self.token_ = Token::new_str_token(loc, name, str_value);
        self.buffer_.clear();
        self.state_ = State::NONE;
    }

    fn get_next_char(&mut self) {
        let mut buffer = [0; 1];
        match self.file_.read_exact(&mut buffer) {
            Err(_e) => {
                self.eof_flag_ = true;
                self.current_char_ = std::char::MAX;
            },
            Ok(()) => self.current_char_ = buffer[0].into(),
        }

        if self.current_char_ == '\n' {
            self.line_ = self.line_ + 1;
            self.column_ = 0;
        } else {
            self.column_ = self.column_ + 1;
        }
    }

    fn get_peek_char(&mut self) -> char {
        let mut buffer = [0; 1];
        match self.file_.read_exact(&mut buffer) {
            Err(_e) => self.eof_flag_ = true,
            Ok(()) => buffer[0] = std::u8::MAX,
        };
        self.file_.seek(SeekFrom::Current(-1)).unwrap();
        buffer[0].into()
    }

    fn add_to_buffer(&mut self, ch: char) {
        self.buffer_.push(ch);
    }

    fn reduce_buffer(&mut self) {
        self.buffer_.pop();
    }

    fn error_token(&mut self, msg: &String) {
        eprintln!("Token Error:{}", msg);
        self.error_flag_ = true;
    }

    fn error_report(&mut self, msg: &String) {
        self.error_token(&format!("Token Error: {}{}", self.get_token_location().to_string(), msg));
    }

    fn handle_line_comment(&mut self) {
        self.loc_ = self.get_token_location();

        if self.current_char_ == '/' && self.get_peek_char() == '/' {
            self.get_next_char();
            self.get_next_char();

            while self.current_char_ != '\n' && !self.eof_flag_ {
                self.get_next_char();
            }

            if !self.eof_flag_ {
                self.get_next_char();
            }
        }
    }

    fn handle_block_comment(&mut self) {
        self.loc_ = self.get_token_location();

        if self.current_char_ == '/' && self.get_peek_char() == '*' {
            self.get_next_char();
            self.get_next_char();

            while !(self.current_char_ == '*' && self.get_peek_char() == '/') {
                if self.eof_flag_ {
                    self.error_report(&format!("end of file happended in comment, */ is expected!, but find {}", self.current_char_));
                }

                self.get_next_char();
            }

            if !self.eof_flag_ {
                self.get_next_char();
                self.get_next_char();
            }
        }
    }

    fn preprocess(&mut self) {
        loop {
            while self.current_char_.is_ascii_whitespace() && !self.eof_flag_ {
                self.get_next_char();
            }

            self.handle_line_comment();
            self.handle_block_comment();

            if !(self.current_char_.is_ascii_whitespace() || self.current_char_ == '/') || self.eof_flag_ {
                break;
            }
        }
    }

    /// Get the current token.
    ///
    /// # Examples
    ///
    /// ```
    /// let scanner = Scanner::new("/test.mjava");
    /// let token = scanner.get_token();
    /// ```
    pub fn get_token(&self) -> Token {
        self.token_.to_owned()
    }

    /// Get the next token.
    ///
    /// # Examples
    /// ```
    /// let scanner = Scanner::new("./test.mjava");
    /// let token = scanner.get_next_token();
    /// ```
    pub fn get_next_token(&mut self) -> Token {
        let mut matched;

        loop {
            self.error_flag_ = false;

            match self.state_ {
                State::NONE => matched = false,
                _ => matched = true,
            }

            match self.state_ {
                State::NONE => self.get_next_char(),
                State::END_OF_FILE => self.handle_eof_state(),
                State::IDENTIFIER => self.handle_identifier_state(),
                State::NUMBER => self.handle_number_state(),
                State::CHAR_LITERAL => self.handle_char_state(),
                State::STRING_LITERAL => self.handle_string_state(),
                State::OPERATION => self.handle_operation_state(),
            }

            match self.state_ {
                State::NONE => {
                    self.preprocess();

                    if self.eof_flag_ {
                        self.state_ = State::END_OF_FILE;
                    } else {
                        if self.current_char_.is_ascii_alphabetic() {
                            self.state_ = State::IDENTIFIER;
                        } else if self.current_char_.is_ascii_digit() {
                            self.state_ = State::NUMBER;
                        } else if self.current_char_ == '\'' {
                            self.state_ = State::CHAR_LITERAL;
                        } else if self.current_char_ == '\"' {
                            self.state_ = State::STRING_LITERAL;
                        } else {
                            self.state_ = State::OPERATION;
                        }
                    }
                },
                _ => {},
            }

            if matched && !self.error_flag_ {
                break;
            }
        }

        self.token_.to_owned()
    }

    fn handle_eof_state(&mut self) {
        self.loc_ = self.get_token_location();
        self.make_token(TokenType::END_OF_FILE, TokenValue::UNRESERVED, self.loc_.to_owned(), "END_OF_FILE".to_string(), -1);
    }

    fn handle_digit(&mut self) {
        self.add_to_buffer(self.current_char_);
        self.get_next_char();

        while self.current_char_.is_ascii_digit() {
            self.add_to_buffer(self.current_char_);
            self.get_next_char();
        }
    }

    fn handle_xdigit(&mut self) {
        let mut read_flag = false;

        while self.current_char_.is_ascii_hexdigit() {
            read_flag = true;
            self.add_to_buffer(self.current_char_);
            self.get_next_char();
        }

        if !read_flag {
            self.error_report(&"Hexadecimal number format error.".to_string());
        }
    }

    fn handle_odigit(&mut self) {
        let mut read_flag = false;

        while self.current_char_ >= '0' && self.current_char_ <= '7' {
            read_flag = true;
            self.add_to_buffer(self.current_char_);
            self.get_next_char();
        }

        if !read_flag
        {
            self.error_report(&"Octal number format error.".to_string());
        }
    }

    fn handle_fraction(&mut self) {
        if !self.current_char_.is_ascii_digit() {
            self.error_report(&"Fraction number part should be numbers".to_string());
        }

        self.add_to_buffer(self.current_char_);
        self.get_next_char();

        while self.current_char_.is_ascii_digit() {
            self.add_to_buffer(self.current_char_);
            self.get_next_char();
        }
    }

    fn handle_exponent(&mut self) {
        self.add_to_buffer(self.current_char_);
        self.get_next_char();

        while self.current_char_ != '+' && self.current_char_ != '-' && !self.current_char_.is_ascii_digit() {
            self.error_report(&format!("Scientist presentation number after e / E should be + / - or digits but find
                        \'{}\'", self.current_char_));
        }

        if self.current_char_ == '+' || self.current_char_ == '-' {
            self.add_to_buffer(self.current_char_);
            self.get_next_char();
        }
    }


    fn handle_number_state(&mut self) {
        self.loc_ = self.get_token_location();

        let mut is_float = false;
        let mut is_exponent = false;

        let mut number_base = 10;

        if self.current_char_ == '0' && (self.get_peek_char() == 'x' || self.get_peek_char() == 'X') {
            number_base = 16;

            self.get_next_char();
            self.get_next_char();
        }

        if self.current_char_ == '0' && self.get_peek_char() >= '0' && self.get_peek_char() <= '7' {
            number_base = 8;

            self.get_next_char();
        }

        enum NumberState {
            INTERGER,
            FRACTION,
            EXPONENT,
            DONE,
        }

        let mut number_state = NumberState::INTERGER;

        loop {
            match number_state {
                NumberState::INTERGER => {
                    if number_base == 10 {
                        self.handle_digit();
                    } else if number_base == 16 {
                        self.handle_xdigit();
                    } else if number_base == 8 {
                        self.handle_odigit();
                    }
                },
                NumberState::FRACTION => {
                    self.handle_fraction();
                    is_float = true;
                },
                NumberState::EXPONENT => {
                    self.handle_exponent();
                    is_exponent = true;
                },
                NumberState::DONE => {},
            }

            if self.current_char_ == '.' {
                if is_float {
                    self.error_report(&"Fraction number can not have more than one dot.".to_string());
                }

                if is_exponent {
                    self.error_report(&"Scientist number representation in MJava can not have dot.".to_string());
                }

                if number_base == 16 {
                    self.error_report(&"Hexadecimal number in MJava can only be integer.".to_string());
                }

                if number_base == 8 {
                    self.error_report(&"Octal number in MJava can only be integer.".to_string());
                }

                number_state = NumberState::FRACTION;
            } else if self.current_char_ == 'E' || self.current_char_ == 'e' {
                if is_exponent {
                    self.error_report(&"Scientist presentation can not have more than one e / E".to_string());
                }

                number_state = NumberState::EXPONENT;
            } else {
                number_state = NumberState::DONE;
            }

            match number_state {
                NumberState::DONE => break,
                _ => {},
            }
        }

        if !self.error_flag_ {
            if is_float || is_exponent {
                let real_value: f64 = match self.buffer_.parse::<f64>() {
                    Err(err) => {
                        self.error_report(&format!("When parse floating-point number literal \"{}\", because {}, an error
                                    occurred.", self.buffer_, err.to_string()));
                        self.buffer_.clear();
                        self.state_ = State::NONE;
                        std::f64::MAX
                    },
                    Ok(real_value) => real_value,
                };

                self.make_real_token(self.loc_.to_owned(), self.buffer_.to_owned(), real_value);
            } else {
                let int_value: i32 = match i32::from_str_radix(&self.buffer_.clone(), number_base) {
                    Err(err) => {
                        self.error_report(&format!("When parse integer literal \"{}\", because {}, an error occurred.", self.buffer_,
                                err.to_string()));
                        self.buffer_.clear();
                        self.state_ = State::NONE;
                        std::i32::MAX
                    },
                    Ok(int_value) => int_value,
                };

                self.make_int_token(self.loc_.to_owned(), self.buffer_.to_owned(), int_value);
            }
        } else {
            self.buffer_.clear();
            self.state_ = State::NONE;
        }
    }

    fn handle_char_state(&mut self) {
        self.loc_ = self.get_token_location();

        self.get_next_char();

        loop {
            if self.current_char_ != '\\' && self.get_peek_char() == '\'' {
                self.add_to_buffer(self.current_char_);
                break;
            }

            if self.eof_flag_ {
                self.error_report(&format!("end of file happended in string, \' is expected!, but find {}",
                            self.current_char_));
                break;
            }

            self.add_to_buffer(self.current_char_);
            self.get_next_char();
        }

        if !self.eof_flag_ {
            self.get_next_char();
            self.get_next_char();
        }

        if !self.error_flag_ && self.buffer_.len() == 1 {
            let ch = self.buffer_.chars().next().unwrap();
            self.make_char_token(self.loc_.to_owned(), self.buffer_.clone(), ch);
        } else {
            self.error_report(&"Char can contain only one character!".to_string());
            self.buffer_.clear();
            self.state_ = State::NONE;
        }
    }

    fn handle_string_state(&mut self) {
        self.loc_ = self.get_token_location();

        self.get_next_char();

        loop {
            if self.eof_flag_ {
                self.error_report(&format!("end of file happended in string, \" is expected!, but find {}",
                            self.current_char_));
                break;
            }

            if self.current_char_ != '\\' && self.get_peek_char() == '\"' {
                self.add_to_buffer(self.current_char_);
                break;
            }

            self.add_to_buffer(self.current_char_);
            self.get_next_char();
        }

        if !self.eof_flag_ {
            self.get_next_char();
            self.get_next_char();
        }

        if !self.error_flag_ {
            self.make_str_token(self.loc_.to_owned(), self.buffer_.clone(), self.buffer_.clone());
        } else {
            self.buffer_.clear();
            self.state_ = State::NONE;
        }
    }

    fn handle_identifier_state(&mut self) {
        self.loc_ = self.get_token_location();

        self.add_to_buffer(self.current_char_);
        self.get_next_char();

        while self.current_char_.is_ascii_alphanumeric() || self.current_char_ == '_' {
            self.add_to_buffer(self.current_char_);
            self.get_next_char();
        }

        let copy = self.buffer_.clone();

        if self.buffer_.eq("System") {
            let mut length = 12;

            while length > 0 && !self.eof_flag_ {
                self.add_to_buffer(self.current_char_);
                self.get_next_char();
                length = length - 1;
            }

            if !self.buffer_.eq("System.out.println") {
                self.buffer_ = copy;
                self.file_.seek(SeekFrom::Current(-length)).unwrap();
            }
        }

        let (token_value, token_type, precedence) = self.dictionary_.lookup(&self.buffer_);

        self.make_token(token_type, token_value, self.loc_.to_owned(), self.buffer_.to_owned(), precedence);
    }

    fn handle_operation_state(&mut self) {
        self.loc_ = self.get_token_location();

        self.add_to_buffer(self.current_char_);

        let ch = self.get_peek_char();
        self.add_to_buffer(ch);

        if self.dictionary_.have_token(&self.buffer_) {
            self.get_next_char();
        } else {
            self.reduce_buffer();
        }

        let (token_value, token_type, precedence) = self.dictionary_.lookup(&self.buffer_);
        self.make_token(token_type, token_value, self.loc_.to_owned(), self.buffer_.to_owned(), precedence);
        self.get_next_char();
    }
}
