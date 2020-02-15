use logos::Logos;

#[derive(Debug, Logos, Eq, PartialEq)]
pub enum Token {
    #[end]
    End,
    #[error]
    Error,

    #[token = "("]
    ParenOpen,
    #[token = ")"]
    ParenClose,

    #[token("==")]
    #[token("!=")]
    #[token(">")]
    #[token("<")]
    #[token(">=")]
    #[token("<=")]
    FieldOperator,

    #[token = "&&"]
    #[token = "||"]
    #[token = "-|"]
    BoolOperator,

    #[regex = "\"[^\"]+\""]
    Literal,

    #[regex = "\\d+"]
    Number,

    #[regex = "@[a-zA-Z][a-z-A-Z0-9_]+"]
    Field,
}
