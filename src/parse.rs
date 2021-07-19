use super::ctx::*;
use super::error::Error;
use super::exprs::*;
use super::lex::*;

pub fn parse(ctx: &mut Ctx, tokens: &mut Tokens) -> Result<Vec<Expression>, Error> {
    let mut statements: Vec<Expression> = Vec::new();

    loop {
        let token = match tokens.next() {
            Some(token) => token,
            None => break,
        };

        statements.push(parse_statement(ctx, tokens, &token)?);
    }

    Ok(statements)
}

fn parse_statement(
    ctx: &mut Ctx,
    tokens: &mut Tokens,
    lead_token: &Token,
) -> Result<Expression, Error> {
    match lead_token {
        Token::Echo => parse_echo(ctx, tokens, true),
        Token::EchoNoNewline => parse_echo(ctx, tokens, false),
        _ => Err(Error::new(
            "Unknown Statement".to_string(),
            Some(tokens.span()),
        )),
    }
}

fn parse_echo(ctx: &mut Ctx, tokens: &mut Tokens, newline: bool) -> Result<Expression, Error> {
    let expr = parse_expr(ctx, tokens)?;
    Ok(EchoExpr::new(expr, newline))
}

fn parse_expr(ctx: &mut Ctx, tokens: &mut Tokens) -> Result<Expression, Error> {
    let primary = parse_primary_expr(ctx, tokens)?;
    parse_op_expr(ctx, tokens, 0, primary)
}

fn parse_primary_expr(ctx: &mut Ctx, tokens: &mut Tokens) -> Result<Expression, Error> {
    let token = tokens.next();

    let token = match token {
        Some(value) => value,
        None => return make_simple_parse_error("Failed to parse expression", tokens),
    };

    match token {
        Token::String => {
            let value = snailquote::unescape(&ctx.contents[tokens.span()])
                .or_else(|_err| make_simple_parse_error::<String>("bad string escape", tokens))?;

            return Ok(StringExpr::new(value));
        }
        Token::Number => {
            let value = ctx.contents[tokens.span()]
                .parse::<f64>()
                .or_else(|_err| make_simple_parse_error::<f64>("bad number", tokens))?;

            return Ok(NumberExpr::new(value));
        }
        _ => return make_simple_parse_error("bad expression", tokens),
    }
}

fn parse_op_expr(
    ctx: &mut Ctx,
    tokens: &mut Tokens,
    precedence: usize,
    primary: Expression,
) -> Result<Expression, Error> {
    let mut primary = primary;

    loop {
        // Get next token
        let operator = match tokens.peek() {
            Some(op) => op,
            None => return Ok(primary),
        };

        // Get precedence of potential operator
        let operator_precedence = match get_op_precedence(operator) {
            None => return Ok(primary),
            Some(operator_precedence) => operator_precedence,
        };
        // Don't proceed if over precedence
        if operator_precedence < precedence {
            return Ok(primary);
        }

        let next_primary = match *operator {
            Token::Plus | Token::Minus | Token::Multiply | Token::Divide => {
                parse_math_expr(ctx, tokens, primary, operator_precedence)?
            }
            _ => return Ok(primary),
        };

        primary = next_primary;
    }
}

fn parse_math_expr(
    ctx: &mut Ctx,
    tokens: &mut Tokens,
    left: Expression,
    operator_precedence: usize,
) -> Result<Expression, Error> {
    let operator = tokens.next().unwrap();
    let right = parse_rhs_expr(ctx, tokens, operator_precedence)?;
    Ok(MathExpr::new(left, operator, right))
}

fn parse_rhs_expr(
    ctx: &mut Ctx,
    tokens: &mut Tokens,
    operator_precedence: usize,
) -> Result<Expression, Error> {
    let rhs = parse_primary_expr(ctx, tokens)?;

    let next_token = match tokens.peek() {
        Some(token) => token,
        None => return Ok(rhs),
    };

    let next_operator_precedence = match get_op_precedence(next_token) {
        Some(precedence) => precedence,
        None => return Ok(rhs),
    };

    if operator_precedence < next_operator_precedence {
        let rhs = parse_op_expr(ctx, tokens, operator_precedence + 1, rhs)?;
        return Ok(rhs);
    }

    Ok(rhs)
}

fn get_op_precedence(token: &Token) -> Option<usize> {
    // Higher precedence = Higher Priority

    return match token {
        Token::Plus => Some(6),
        Token::Minus => Some(6),
        Token::Multiply => Some(5),
        Token::Divide => Some(5),
        _ => None,
    };
}

fn make_simple_parse_error<T>(message: &str, tokens: &mut Tokens) -> Result<T, Error> {
    Err(Error::new(message.to_string(), Some(tokens.span())))
}
