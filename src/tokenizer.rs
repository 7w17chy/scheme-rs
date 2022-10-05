#[derive(PartialEq, Debug)]
pub enum Token {
    LParen,  // (
    RParen,  // )
    Quote,   // '
    Unquote, // `
    Colon,   // :
    Integer(i32),
    Float(f32),
    Symbol(String),
    WhiteSpace,
    EOF,
}

pub fn tokenize<'a, T>(input: T) -> Vec<Token>
where
    T: IntoIterator<Item = &'a str>,
{
    let mut result = Vec::with_capacity(100);

    for token in input
        .into_iter()
        .collect::<String>()
        .replace('(', " ( ")
        .replace(')', " ) ")
        .split(' ')
    {
        match token {
            "(" => result.push(Token::LParen),
            ")" => result.push(Token::RParen),
            "'" => result.push(Token::Quote),
            "`" => result.push(Token::Unquote),
            ":" => result.push(Token::Colon),
            "\0" => result.push(Token::EOF),
            other if !other.is_empty() => {
                if other.contains('.') {
                    if let Ok(val) = other.parse::<f32>() {
                        result.push(Token::Float(val));
                        continue;
                    }
                }
                if let Ok(val) = other.parse::<i32>() {
                    result.push(Token::Integer(val));
                    continue;
                }
                result.push(Token::Symbol(other.to_string()));
            }
            _ => continue,
        }
    }

    result
}
