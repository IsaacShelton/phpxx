
use super::lex;
use super::ctx::*;
use super::exprs::*;

pub fn parse(ctx: &mut Ctx, tokens: &mut logos::Lexer<lex::Token>) -> Vec<Box<dyn Expr>> {
    let mut statements: Vec<Box<dyn Expr>> = Vec::new();

    loop {
        let token = tokens.next();

        if token.is_none() {
            break;
        }


        let statement = match token.unwrap() {
        lex::Token::Echo => parse_echo(ctx, tokens),
        _ => Err("Unknown statement".to_string())
        };

        match statement {
        Err(e) => panic!("{}", e),
        Ok(value) => statements.push(value),
        }
    }

    statements
}

fn parse_echo(ctx: &mut Ctx, tokens: &mut logos::Lexer<lex::Token>) -> Result<Box<dyn Expr>, String> {
    let expr = parse_expr(ctx, tokens);

    if expr.is_err() {
        return expr;
    }

    Ok(EchoExpr::new(expr.unwrap()))
}

fn parse_expr(ctx: &mut Ctx, tokens: &mut logos::Lexer<lex::Token>) -> Result<Box<dyn Expr>, String> {
    let token = tokens.next();

    if token.is_none() {
        return Err(String::from("Failed to parse expression"));
    }

    let token = token.unwrap();

    match token {
    lex::Token::String => {
        let value = snailquote::unescape(&ctx.contents[tokens.span()]).or_else(|_err| {
            Err("bad string escape".to_string())
        })?;

        return Ok(StringExpr::new(value))
    },
    lex::Token::LiteralRealNumberDot => {
        let value = ctx.contents[tokens.span()].parse::<f64>().or_else(|_err| {
            Err("bad number".to_string())
        })?;

        return Ok(NumberExpr::new(value))
    },
    _ => return Err("bad expression".to_string())
    }
}
