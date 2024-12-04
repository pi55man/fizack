#![allow(dead_code, unused_imports, unused_variables)]
use std::env;
use std::error::Error;
use std::io::{self, stdout, Write};
use std::task::Wake;

mod token;
use token::TokenType;

struct Fiza{
    args: Vec<String>,
}

#[derive(Debug)] struct Token{
    ttype: TokenType,
    lexeme: String,
    literal: String,
    line: usize,
}

struct Scanner{
    source: String,
    tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: usize,
}

//TODO: implement tostring
impl Token{
    fn new(ttype: TokenType, lexeme: String,  literal:String,line:usize,) -> Self{
        Token{
            ttype,
            lexeme,
            literal,
            line,
        }
    }

}
//TODO: implement scanner
impl Scanner{
    fn new(source: String) -> Self{
        Scanner { 
            source,
            tokens: Vec::new(),
            start: 0,
            current: 0,
            line: 1,
        }
    }

    fn scan_tokens(&mut self) {
        while !self.is_at_end() {
            self.start = self.current;   
            self.scan_token().unwrap();
        }
        self.tokens.push(Token::new(TokenType::EOF, "".to_string(), "".to_string(), self.line));

    }

    fn is_at_end(&self) -> bool{
       self.current >= self.source.len()
    }

    fn scan_token(&mut self)-> Result<(), Box<dyn Error>>{
        let c: char = self.advance();
        match c{
            '(' => self.add_token(TokenType::LEFT_PARAN), 
            ')' => self.add_token(TokenType::RIGHT_PARAN), 
            '{' => self.add_token(TokenType::LEFT_BRACE), 
            '}' => self.add_token(TokenType::RIGHT_BRACE), 
            ',' => self.add_token(TokenType::COMMA), 
            '.' => self.add_token(TokenType::DOT), 
            '-' => self.add_token(TokenType::MINUS), 
            '+' => self.add_token(TokenType::PLUS), 
            ';' => self.add_token(TokenType::SEMICOLON), 
            '*' => self.add_token(TokenType::STAR), 
            '!' => {
                    if self.match_token('=') {
                        self.add_token(TokenType::BANG_EQUAL)
                    } else {
                        self.add_token(TokenType::BANG)
                    }
            },
            _ => Err("invalid token.".into()),
        }
    }

    fn advance(&mut self) -> char{
         let c = self.source.chars().nth(self.current).unwrap();
         self.current+=1;
         c
    }
    
    fn add_token(&mut self, ttype: TokenType) -> Result<(), Box<dyn Error>>{
        let text: String = self.source[self.start..self.current].to_string();
        self.tokens.push(Token::new(ttype, text, "".to_string(),self.line));
        Ok(())
    }

    fn match_token(&mut self, expected: char) -> bool{
        if self.is_at_end() {
            return false;
        }
        if self.source.chars().nth(self.current).unwrap() != expected {
            return false;
        }
        self.current+=1;
        true
    }
}

impl Fiza{
    fn new() -> Self{
        Fiza{
            args: env::args().collect(),
        }
    }

    fn start(&self) -> Result<(), Box<dyn Error>>{
        match self.args.len(){
            1 => {
                self.run_prompt()?;
                Ok(())
            }
            2 => {
                self.run_file(&self.args[1]);
                Ok(())
            },
            _ => Err("usage: fizack [file]".into()),
        }
    }

    fn run_file(&self, file_path: &String){
        ()
    }
    
    fn run_prompt(&self) -> io::Result<()>{
        loop{
            print!(">> ");
            stdout().flush()?;
            let mut line: String = Default::default(); 
            io::stdin().read_line(&mut line)?;
            self.run(&line);
        }
    }
    
    fn run(&self, line: &String){
        print!("{}",line);
    }
}


fn main() {
        let source: String = String::from("(+{}),");
        let mut scan = Scanner::new(source);
        scan.scan_tokens();

        for t in scan.tokens{
            println!("token: {:?}",t);
        }
}

#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn check_left_paran() {
        let source: String = String::from("(");
        let mut scan = Scanner::new(source);
        scan.scan_tokens();
        let a = format!("{:?}",scan.tokens[0]);
        assert_eq!("Token { ttype: LEFT_PARAN, lexeme: \"(\", literal: \"\", line: 1 }",a);
    }
}
