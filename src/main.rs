pub mod objects;
pub mod parser;
pub mod tokenizer;

use anyhow::Result;

fn main() -> Result<()> {
    println!("Hello, world!");

    Ok(())
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
