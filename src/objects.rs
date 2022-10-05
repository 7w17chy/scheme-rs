use std::collections::HashMap;
use std::convert::AsRef;

// TODO: add `eval` method
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
