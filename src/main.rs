mod ctx;
mod error;
mod exprs;
mod lex;
mod parse;

use colored::Colorize;
use ctx::*;
use lex::*;
use logos::Logos;
use parse::*;

fn main() {
    let filename = "main.phpxx";
    let contents = match std::fs::read_to_string(&filename) {
        Ok(result) => result,
        Err(_) => {
            println!(
                "{}{}{}",
                "Failed to read file '".red(),
                filename.red(),
                "'".red()
            );
            return;
        }
    };

    let mut ctx = Ctx::new(&contents);
    let mut lexer = Token::lexer(&contents);
    let mut tokens: Vec<Token> = vec![];
    let mut spans: Vec<std::ops::Range<usize>> = vec![];

    loop {
        let value = match lexer.next() {
            Some(value) => value,
            None => break
        };

        tokens.push(value);
        spans.push(lexer.span());
    }

    let statements = parse(&mut ctx, &mut Tokens::new(&tokens[..], &spans[..]));

    let statements = match statements {
        Err(error) => {
            match error.location {
                Some(location) => println!(
                    "{}{}{}{}",
                    error.message.bold().red(),
                    " - '".bold().red(),
                    &ctx.contents[location].bold().red(),
                    "'".bold().red()
                ),
                None => println!("{}", error.message.bold().red()),
            }
            return;
        }
        Ok(statements) => statements,
    };

    for s in statements {
        s.evaluate();
    }
}
