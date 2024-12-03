#![allow(dead_code, unused_imports, unused_variables)]
use std::{env, io};
use std::error::Error;
use std::io::{stdout, Write};

mod token;
use token::TokenType;

struct Fiza{
    args: Vec<String>,
}

struct Token{
    ttype: TokenType,
    lexeme: String,
    line: usize,
    literal: String,
}

struct Scanner{
    source: String,
    tokens: Vec<Token>,
}

//TODO: implement tostring
impl Token{
    fn new(ttype: TokenType, lexeme: String, line:usize, literal:String) -> Self{
        Token{
            ttype,
            lexeme,
            line,
            literal
        }
    }

}

impl Scanner{
    fn new(source: String) -> Self{
        Scanner { source, tokens: Vec::new() }
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
    let fiza = Fiza::new();
    fiza.start().expect("error with starting");
}
