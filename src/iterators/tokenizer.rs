use std::{iter::Peekable, str::Chars};

fn is_blank(ch: &char) -> bool {
    *ch == ' ' || *ch == '\t' || *ch == '\n' || *ch == '\r'
}

fn is_num(ch: &char) -> bool {
    *ch >= '0' && *ch <= '9'
}

fn is_letter(ch: &char) -> bool {
    *ch >= 'A' && *ch <= 'Z' || *ch >= 'a' && *ch <= 'z'
}

#[derive(Debug)]
pub enum Token {
    OBracket, // {
    CBracket, // }
    Word(String),
    Number(String),
    String(String),
}

pub struct Tokenizer<'a> {
    chars: Peekable<Chars<'a>>,
}

impl<'a> Tokenizer<'a> {
    pub fn from(string: &'a String) -> Self {
        Self {
            chars: string.chars().peekable(),
        }
    }

    fn skip_comment(&mut self) {
        // Discard starting |
        self.chars.next();

        loop {
            match self.chars.peek() {
                Some('|') | Some('\n') => break,
                None => panic!("Unclosed comment"),
                _ => (),
            }
            self.chars.next();
        }

        // Discard ending | or \n
        self.chars.next();
    }

    fn skip_blanks(&mut self) {
        loop {
            match self.chars.peek() {
                Some(ch) if is_blank(ch) => {
                    self.chars.next();
                }
                _ => break,
            }
        }
    }

    fn build_word(&mut self) -> String {
        let mut word = String::new();

        loop {
            match self.chars.peek() {
                Some(ch) if is_num(ch) || is_letter(ch) || *ch == '_' => word.push(*ch),
                _ => break,
            }

            self.chars.next();
        }

        word
    }

    fn build_number(&mut self) -> String {
        let mut num = String::new();
        let mut is_fraction = false;
        let mut is_signed = false;

        loop {
            match self.chars.peek() {
                Some(ch) if is_num(ch) => num.push(*ch),
                Some('-') => {
                    if is_signed {
                        break;
                    }

                    num.push('-');
                    is_signed = true;
                }
                Some('.') => {
                    if is_fraction {
                        panic!("Number has already fractional part");
                    }

                    num.push('.');
                    is_fraction = true;
                }
                Some('_') => (),
                _ => break,
            }

            self.chars.next();
        }

        num
    }

    fn build_string(&mut self) -> String {
        let mut string = String::new();

        // Discard starting "
        self.chars.next();

        loop {
            match self.chars.peek() {
                Some(ch) if *ch == '"' => {
                    // Discard ending "
                    self.chars.next();
                    break;
                }
                Some(ch) if *ch == '\\' => {
                    // Handle escape characters
                    self.chars.next();

                    match self.chars.peek() {
                        Some(ch) if *ch == 't' => string.push('\t'),
                        Some(ch) if *ch == 'n' => string.push('\n'),
                        Some(ch) if *ch == '\\' => string.push('\\'),
                        Some(ch) if *ch == '"' => string.push('"'),
                        _ => panic!("No supported escape value given"),
                    }
                }
                Some(ch) => string.push(*ch),
                None => panic!("Unclosed string"),
            }

            self.chars.next();
        }

        string
    }
}

impl<'a> Iterator for Tokenizer<'a> {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        // Skip comments and blanks
        loop {
            match self.chars.peek() {
                Some(ch) if is_blank(ch) => self.skip_blanks(),
                Some('|') => self.skip_comment(),
                _ => break,
            }
        }

        match self.chars.peek() {
            Some('{') => {
                self.chars.next();
                Some(Token::OBracket)
            }
            Some('}') => {
                self.chars.next();
                Some(Token::CBracket)
            }
            Some('"') => Some(Token::String(self.build_string())),
            Some(ch) if is_num(ch) || *ch == '-' => Some(Token::Number(self.build_number())),
            Some(ch) if is_letter(ch) || *ch == '_' => Some(Token::Word(self.build_word())),
            Some(ch) => panic!("Unsupported character \"{}\"", ch),
            None => None,
        }
    }
}
