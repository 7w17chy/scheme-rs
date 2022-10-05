use crate::objects::{Atom, Expression, Object};
use crate::tokenizer::Token;
use anyhow::{ensure, Result};

fn push_if_list(object: &mut Expression, value: Box<dyn Object>) {
    if let Expression::List(ref mut vec) = object {
        vec.push(value);
    }
}

pub fn parse(tokens: Vec<Token>) -> Result<Vec<Expression>> {
    let mut ast: Vec<Expression> = Vec::with_capacity(tokens.len());
    let mut list_index = 0usize;
    ensure!(
        tokens[0] == Token::LParen,
        "Expected '(', found '{}'",
        format!("{:?}", tokens[0])
    );

    for token in tokens {
        match token {
            Token::LParen => ast.push(Expression::List(Vec::new())),
            Token::RParen => {
                ensure!(list_index > 0, "Unexpected ')' during parsing");
                list_index -= 1;
            }
            Token::Symbol(x) => push_if_list(&mut ast[list_index], Box::new(Atom::Symbol(x))),
            Token::Float(x) => push_if_list(&mut ast[list_index], Box::new(Atom::Float(x))),
            Token::Integer(x) => push_if_list(&mut ast[list_index], Box::new(Atom::Integer(x))),
            Token::WhiteSpace => continue,
            Token::Quote => unimplemented!(),
            Token::Unquote => unimplemented!(),
            Token::Colon => unimplemented!(),
            Token::EOF => unimplemented!(),
        }
    }

    Ok(ast)
}
