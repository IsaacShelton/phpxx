
use logos::Logos;

#[derive(Logos, Debug, PartialEq)]
pub enum Token {
    #[token("echo")]
    Echo,

    #[token(";")]
    Semicolon,

    #[regex(r#""([^"\\]|\\t|\\u|\\n|\\")*""#)]
    String,

    #[regex(r"[0-9][0-9_]*\.[0-9][0-9_]*")]
    LiteralRealNumberDot,

    // We can also use this variant to define whitespace,
    // or any other matches we wish to skip.
    #[regex(r"[ \t\n\f]+", logos::skip)]
    Whitespace,

    // Logos requires one token variant to handle errors,
    // it can be named anything you wish.
    #[error]
    Error,
}
