use anyhow::Result;

fn main() -> Result<()> {
    println!("Hello, world!");

    Ok(())
}

pub mod objects {
    use std::collections::HashMap;
    use std::convert::AsRef;

    pub trait Object {}

    pub enum Atom {
        Symbol(String),
        Float(f32),
        Integer(i32),
    }

    impl Object for Atom {}

    pub enum Expression {
        Atom(Atom),
        List(Vec<Box<dyn Object>>),
    }

    impl Object for Expression {}

    pub struct Environment<'a> {
        map: HashMap<&'a str, Box<dyn Object>>,
    }

    impl<'a> Environment<'a> {
        pub fn get<T>(&self, key: T) -> Option<&Box<dyn Object>>
        where
            T: AsRef<&'a str>,
        {
            self.map.get(key.as_ref())
        }
    }
}

pub mod tokenizer {
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
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn tokenization() {
        use tokenizer::Token;

        let input = "(henlo (x 32 32.32))".to_string();
        let tokens = tokenizer::tokenize(input.lines());
        assert_eq!(
            tokens,
            vec![
                Token::LParen,
                Token::Symbol("henlo".to_string()),
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
