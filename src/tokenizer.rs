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

#[cfg(test)]
mod tests {
    use crate::tokenizer::{self, Token};

    #[test]
    fn tokenization() {
        use tokenizer::Token;

        let input = "(henlo '(x 32 32.32))".to_string();
        let tokens = tokenizer::tokenize(input.lines());
        assert_eq!(
            tokens,
            vec![
                Token::LParen,
                Token::Symbol("henlo".to_string()),
                Token::Quote,
                Token::LParen,
                Token::Symbol("x".to_string()),
                Token::Integer(32i32),
                Token::Float(32.32f32),
                Token::RParen,
                Token::RParen
            ]
        );
    }
}
