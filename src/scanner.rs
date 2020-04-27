pub mod scanner {
    use std::vec::Vec;
    use std::collections::HashMap;
    use std::fs::File;
    use std::io::{prelude::*, BufReader};

    lazy_static! {
        static ref TV: HashMap<&'static str, &'static str> = {
            let mut token_value = HashMap::new();
            token_value.insert("class", "CLASS");
            token_value.insert("public", "PUBLIC");
            token_value.insert("static", "STATIC");
            token_value.insert("void", "VOID");
            token_value.insert("main", "MAIN");
            token_value.insert("String", "STRING");
            token_value.insert("extends", "EXTENDS");
            token_value.insert("return", "RETURN");
            token_value.insert("int", "INT");
            token_value.insert("boolean", "BOOLEAN");
            token_value.insert("if", "IF");
            token_value.insert("else", "ELSE");
            token_value.insert("while", "WHILE");
            token_value.insert("System.out.println", "PRINT");
            token_value.insert("length", "LENGTH");
            token_value.insert("true", "TRUE");
            token_value.insert("false", "FALSE");
            token_value.insert("this", "THIS");
            token_value.insert("new", "NEW");
            token_value.insert("[", "LBRACK");
            token_value.insert("]", "RBRACK");
            token_value.insert("(", "LPAREN");
            token_value.insert(")", "RPAREN");
            token_value.insert("{", "LBRACE");
            token_value.insert("}", "RBRACE");
            token_value.insert(",", "COMMA");
            token_value.insert(";", "SEMICOLON");
            token_value.insert("=", "ASSIGN");
            token_value.insert("&&", "AND");
            token_value.insert("<", "LT");
            token_value.insert("+", "ADD");
            token_value.insert("-", "SUB");
            token_value.insert("*", "MULTI");
            token_value.insert("!", "NOT");
            token_value.insert(".", "DOT");
            token_value
        };
    }

    pub fn file_scan(fp: &mut File, of: &mut File) {
        let reader = BufReader::new(fp);
        let mut line_count = 0;
        for chars in reader.lines() {
            let mut str = chars.unwrap();
            str.push(std::char::MAX);
            let line: Vec<char> = str.clone().chars().collect();
            line_count = line_count + 1;
            let length = str.chars().count();
            let mut start_index = 0;
            let mut end_index = start_index;

            while end_index < length - 1 {
                while line[end_index].is_ascii_whitespace() {
                    end_index = end_index + 1;
                }

                start_index = end_index;

                if end_index == length - 1 {
                    break;
                }

                if line[end_index] == '_' {
                    while line[end_index].is_ascii_alphanumeric() || line[end_index] == '_' {
                        end_index = end_index + 1;
                    }

                    of.write_fmt(format_args!("ERROR: Identifiers can not begin with an underscore: {}\n", &str[start_index..end_index])).unwrap();
                    continue;
                }

                if line[end_index].is_ascii_alphabetic() {
                    while line[end_index].is_ascii_alphanumeric() || line[end_index] == '_' {
                        end_index = end_index + 1;
                    }

                    let token = &str[start_index..end_index];

                    if TV.contains_key(&token) {
                        of.write_fmt(format_args!("#{} {} {}\n", line_count, TV.get(&token).unwrap(), token)).unwrap();
                        continue;
                    }

                    if end_index + 12 <= length && str[start_index..end_index + 12].eq("System.out.println") {
                        of.write_fmt(format_args!("#{} {} {}\n", line_count, "PRINT", &str[start_index..end_index+ 12])).unwrap();
                        end_index = end_index + 12;
                        continue;
                    } else {
                        of.write_fmt(format_args!("#{} {} {}\n", line_count, "IDENTIFIER", token)).unwrap();
                        continue;
                    }
                }

                if line[end_index].is_ascii_digit() {
                    while line[end_index].is_ascii_digit() {
                        end_index = end_index + 1;
                    }

                    if !line[end_index].is_ascii_alphabetic() && line[end_index] != '.' && line[end_index] != '_' {
                        of.write_fmt(format_args!("#{} {} {}\n", line_count, "INTEGER", &str[start_index..end_index])).unwrap();
                        continue;
                    } else if line[end_index].is_ascii_alphabetic() || line[end_index] == '_' {
                        while line[end_index].is_ascii_alphanumeric() || line[end_index] == '_' {
                            end_index = end_index + 1;
                        }

                        of.write_fmt(format_args!("#{} ERROR: Identifiers can not begin with a number: {}\n", line_count, &str[start_index..end_index])).unwrap();
                        continue;
                    } else if line[end_index] == '.' {
                        let mut index = end_index + 1;

                        while line[index].is_ascii_digit() {
                            index = index + 1;
                        }

                        if line[index].is_ascii_alphabetic() || line[index] == '_' {
                            of.write_fmt(format_args!("#{} {} {}\n", line_count, "INTEGER", &str[start_index..end_index])).unwrap();
                            of.write_fmt(format_args!("#{} {} {}\n", line_count, "DOT", '.')).unwrap();
                            end_index = end_index + 1;
                            start_index = end_index;

                            while line[index].is_ascii_alphanumeric() || line[index] == '_' {
                                index = index + 1;
                            }

                            end_index = index;

                            if line[end_index].is_ascii_digit() || line[end_index] == '_' {
                                of.write_fmt(format_args!("#{} ERROR: Identifiers can not begin with a number: {}\n", line_count, &str[start_index..end_index])).unwrap();
                                continue;
                            } else {
                                of.write_fmt(format_args!("#{} {} {}\n", line_count, "IDENTIFIER", &str[start_index..end_index])).unwrap();
                                continue;
                            }
                        } else {
                            end_index = index;
                            of.write_fmt(format_args!("#{} ERROR: Floating Numbers are not supported: {}\n", line_count, &str[start_index..end_index])).unwrap();
                            continue;
                        }
                    }
                }

                if line[end_index] == '.' {
                    let index = end_index;
                    end_index = end_index + 1;

                    while line[end_index].is_ascii_digit() {
                        end_index = end_index + 1;
                    }

                    if index == end_index {
                        of.write_fmt(format_args!("#{} DOT .\n", line_count)).unwrap();
                        continue;
                    }

                    if line[end_index].is_ascii_alphabetic() || line[end_index] == '_' {
                        while line[end_index].is_ascii_alphanumeric() || line[end_index] == '_' {
                            end_index = end_index + 1;
                        }

                        of.write_fmt(format_args!("#{} ERROR: Identifiers can not begin with a dot: {}\n", line_count, &str[start_index..end_index])).unwrap();
                        continue;
                    } else {
                        of.write_fmt(format_args!("#{} ERROR: Floating Numbers are not supported: {}\n", line_count, &str[start_index..end_index])).unwrap();
                        continue;
                    }
                }

                if TV.contains_key(&str[end_index..end_index + 1]) {
                    let token = &str[end_index..end_index + 1];
                    of.write_fmt(format_args!("#{} {} {}\n", line_count, TV.get(&token).unwrap(), line[end_index].to_string())).unwrap();
                    end_index = end_index + 1;
                    continue;
                }

                if start_index + 2 < length && str[start_index..start_index + 2].eq("&&") {
                   end_index = end_index + 2;
                   of.write_fmt(format_args!("#{} {} {}\n", line_count, "AND", "&&")).unwrap();
                   continue;
                }

                of.write_fmt(format_args!("#{} ERROR: Unknown character: {}\n", line_count, &str[end_index..end_index + 1])).unwrap();
                end_index = end_index + 1;
            }
        }
    }
}


