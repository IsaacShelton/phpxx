mod ctx;
mod lex;
mod parse;
mod exprs;

use ctx::*;
use lex::*;
use parse::*;
use logos::Logos;
use colored::Colorize;

fn main() {
    let filename = "main.phpxx";
    
    let contents = match std::fs::read_to_string(&filename) {
        Ok(result) => result,
        Err(_) => {
            println!("{}{}{}", "Failed to read file '".red(), filename.red(), "'".red());
            return
        }
    };

    let mut ctx = Ctx::new(&contents);
    let mut lex = Token::lexer(&contents);
    let statements = parse(&mut ctx, &mut lex);

    for s in statements {
        s.evaluate();
    }
}
