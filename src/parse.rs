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

        let statement = parse_statement(ctx, tokens, &token, &mut statements)?;
        statements.push(statement);
    }

    Ok(statements)
}

fn parse_statement(
    ctx: &mut Ctx,
    tokens: &mut Tokens,
    lead_token: &Token,
    statements: &mut Vec<Expression>,
) -> Result<Expression, Error> {
    match lead_token {
        Token::Echo => parse_echo(ctx, tokens),
        Token::Variable => parse_assign(ctx, tokens),
        Token::Identifier => parse_call_expr(ctx, tokens),
        Token::Function => parse_function(ctx, tokens, statements.len()),
        Token::End => parse_end(ctx, tokens, statements),
        Token::If => parse_conditional(ctx, tokens, false),
        Token::While => parse_conditional(ctx, tokens, true),
        _ => Err(Error::new(
            "Unknown Statement".to_string(),
            Some(tokens.span()),
        )),
    }
}

fn parse_echo(ctx: &mut Ctx, tokens: &mut Tokens) -> Result<Expression, Error> {
    let mut newline = true;

    if tokens.has_next() && *tokens.peek().unwrap() == Token::Minus {
        // Skip over '-' that indicates option to echo
        tokens.next();

        if !tokens.has_next() || *tokens.next().unwrap() != Token::Identifier {
            return Err(Error::new(
                "Expected option after '-' after 'echo'".to_string(),
                Some(tokens.span()),
            ));
        }

        match &ctx.contents[tokens.span()] {
            "n" => {
                newline = false;
            }
            _ => {
                return Err(Error::new(
                    format!(
                        "Invalid option -{} given to 'echo'",
                        &ctx.contents[tokens.span()]
                    ),
                    Some(tokens.span()),
                ))
            }
        }
    }

    let expr = parse_expr(ctx, tokens)?;
    Ok(EchoExpr::new(expr, newline))
}

fn parse_assign(ctx: &mut Ctx, tokens: &mut Tokens) -> Result<Expression, Error> {
    let variable = &ctx.contents[tokens.span()];

    if match tokens.next() {
        Some(Token::Assign) => false,
        _ => true,
    } {
        return Err(Error::new(
            "Expected '=' after variable name in statement".to_string(),
            Some(tokens.span()),
        ));
    }

    let value = parse_expr(ctx, tokens)?;
    Ok(AssignExpr::new(variable.to_string(), value))
}

fn parse_function(ctx: &mut Ctx, tokens: &mut Tokens, address: usize) -> Result<Expression, Error> {
    if ctx.parsing_function.is_some() {
        return Err(Error::new(
            "Already in function".to_string(),
            Some(tokens.span()),
        ));
    }

    let name = match tokens.next() {
        Some(Token::Identifier) => ctx.contents[tokens.span()].to_string(),
        _ => {
            return Err(Error::new(
                "Expected name of function".to_string(),
                Some(tokens.span()),
            ));
        }
    };

    let mut args: Vec<String> = Vec::new();

    match tokens.next() {
        Some(Token::Open) => (),
        _ => {
            return Err(Error::new(
                "Expected '(' after function name".to_string(),
                Some(tokens.span()),
            ));
        }
    }

    loop {
        let token = tokens.next();
        match token {
            Some(Token::Close) => break,
            Some(Token::Variable) => (),
            _ => {
                return Err(Error::new(
                    "Expected ')' or ',' in list of function arguments".to_string(),
                    Some(tokens.span()),
                ));
            }
        }

        args.push(String::from(&ctx.contents[tokens.span()]));

        match tokens.next() {
            Some(Token::Close) => break,
            Some(Token::Next) => (),
            _ => {
                tokens.next();
                return Err(Error::new(
                    "Expected ')' or ',' after function argument name".to_string(),
                    Some(tokens.span()),
                ));
            }
        }
    }

    match tokens.next() {
        Some(Token::Begin) => (),
        _ => {
            return Err(Error::new(
                "Expected '{' after function".to_string(),
                Some(tokens.span()),
            ));
        }
    }

    ctx.add_function(name, address + 1, args);
    ctx.parsing_function = Some(address);

    // Will be overwritten later with jump instruction
    Ok(VoidExpr::new())
}

fn parse_end(
    ctx: &mut Ctx,
    tokens: &mut Tokens,
    statements: &mut Vec<Expression>,
) -> Result<Expression, Error> {
    if ctx.parsing_function.is_none() {
        return Err(Error::new(
            "Unexpected '}'".to_string(),
            Some(tokens.span()),
        ));
    }

    statements[ctx.parsing_function.unwrap()] = JumpExpr::new(statements.len() + 1);
    ctx.parsing_function = None;
    Ok(CallExpr::new("throw".to_string(), vec![]))
}

fn parse_conditional(
    ctx: &mut Ctx,
    tokens: &mut Tokens,
    is_while: bool,
) -> Result<Expression, Error> {
    let conditional_kind_name = if is_while { "while" } else { "if" };

    let condition = parse_expr(ctx, tokens)?;

    match tokens.next() {
        Some(Token::Begin) => (),
        _ => {
            return Err(Error::new(
                format!(
                    "Expected '{{' after condition of '{}' statement",
                    conditional_kind_name
                ),
                Some(tokens.span()),
            ));
        }
    }

    let when_true = parse_block(ctx, tokens, conditional_kind_name)?;

    let when_false = match tokens.peek() {
        Some(Token::Else) => {
            tokens.next();
            match tokens.next() {
                Some(Token::Begin) => parse_block(ctx, tokens, conditional_kind_name)?,
                _ => {
                    return Err(Error::new(
                        format!(
                            "Expected '{{' after 'else' keyword of '{}' statement",
                            conditional_kind_name
                        ),
                        Some(tokens.span()),
                    ));
                }
            }
        }
        _ => Vec::new(),
    };

    Ok(ConditionalExpr::new(
        condition, when_true, when_false, is_while,
    ))
}

fn parse_block(
    ctx: &mut Ctx,
    tokens: &mut Tokens,
    statement_kind_name: &str,
) -> Result<Vec<Expression>, Error> {
    let mut statements: Vec<Expression> = Vec::new();

    loop {
        let lead_token = match tokens.next() {
            None => {
                return Err(Error::new(
                    format!(
                        "Expected '}}' to close '{}' statement before end of file",
                        statement_kind_name
                    ),
                    Some(tokens.span()),
                ))
            }
            Some(Token::End) => break,
            token => token.unwrap(),
        };

        let stmt = parse_statement(ctx, tokens, &lead_token, &mut statements)?;
        statements.push(stmt);
    }

    Ok(statements)
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

            Ok(StringExpr::new(value))
        }
        Token::Number => {
            let value = ctx.contents[tokens.span()]
                .parse::<f64>()
                .or_else(|_err| make_simple_parse_error::<f64>("bad number", tokens))?;

            Ok(NumberExpr::new(value))
        }
        Token::Variable => {
            Ok(VariableExpr::new(ctx.contents[tokens.span()].to_string()))
        }
        Token::Identifier => parse_call_expr(ctx, tokens),
        Token::Spread => {
            let inner = parse_primary_expr(ctx, tokens)?;
            Ok(SpreadExpr::new(inner))
        }
        _ => make_simple_parse_error("bad expression", tokens),
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

fn parse_call_expr(ctx: &mut Ctx, tokens: &mut Tokens) -> Result<Expression, Error> {
    let function = ctx.contents[tokens.span()].to_string();
    let mut args = vec![];

    if match tokens.next() {
        Some(token) => *token != Token::Open,
        None => true,
    } {
        return Err(Error::new(
            "Expected '(' after identifer".to_string(),
            Some(tokens.span()),
        ));
    }

    loop {
        if match tokens.peek() {
            Some(Token::Close) => true,
            None => {
                return Err(Error::new(
                    "Expected ')' before end of file".to_string(),
                    Some(tokens.span()),
                ))
            }
            _ => false,
        } {
            tokens.next();
            break;
        }

        args.push(parse_expr(ctx, tokens)?);

        if match tokens.next() {
            Some(Token::Next) => false,
            Some(Token::Close) => true,
            _ => {
                return Err(Error::new(
                    "Expected ',' or ')' after argument to call".to_string(),
                    Some(tokens.span()),
                ))
            }
        } {
            break;
        }
    }

    Ok(CallExpr::new(function, args))
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
