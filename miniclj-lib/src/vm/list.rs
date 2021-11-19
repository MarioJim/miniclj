use std::{
    fmt::{self, Display, Formatter},
    hash::{Hash, Hasher},
};

use crate::vm::Value;

#[derive(Debug, Clone)]
pub enum List {
    Cons(Box<Value>, Box<List>),
    EmptyList,
}

impl List {
    pub fn nth(self, mut index: usize) -> Option<Value> {
        let mut list = self;
        while let List::Cons(first, rest) = list {
            if index == 0 {
                return Some(*first);
            }
            index -= 1;
            list = *rest;
        }
        None
    }

    pub fn len(&self) -> usize {
        let mut len = 0;
        let mut list = self;

        while let List::Cons(_, rest) = list {
            len += 1;
            list = rest;
        }

        len
    }

    fn inner_display(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            List::Cons(first, rest) => {
                write!(f, " {}", first)?;
                rest.inner_display(f)
            }
            List::EmptyList => write!(f, ""),
        }
    }
}

impl Display for List {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            List::Cons(first, rest) => {
                write!(f, "({}", first)?;
                rest.inner_display(f)?;
                write!(f, ")")
            }
            List::EmptyList => write!(f, "()"),
        }
    }
}

impl PartialEq for List {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (List::Cons(first1, rest1), List::Cons(first2, rest2)) => {
                first1 == first2 && rest1 == rest2
            }
            (List::EmptyList, List::EmptyList) => true,
            _ => false,
        }
    }
}
impl Eq for List {}

#[derive(Hash)]
struct EmptyListHash;

impl Hash for List {
    fn hash<H: Hasher>(&self, state: &mut H) {
        match self {
            List::Cons(first, rest) => {
                first.hash(state);
                rest.hash(state);
            }
            List::EmptyList => EmptyListHash.hash(state),
        }
    }
}

impl FromIterator<Value> for List {
    fn from_iter<T: IntoIterator<Item = Value>>(iter: T) -> List {
        let mut list = List::EmptyList;
        for value in iter {
            list = List::Cons(Box::new(value), Box::new(list));
        }
        list
    }
}

impl TryFrom<Value> for List {
    type Error = &'static str;

    fn try_from(value: Value) -> Result<List, Self::Error> {
        match value {
            Value::List(list) => Ok(list),
            Value::Vector(vector) => Ok(vector.into_iter().collect()),
            Value::Set(set) => Ok(set.into_iter().collect()),
            Value::Map(map) => Ok(map
                .into_iter()
                .map(|(key, val)| Value::Vector(vec![key, val]))
                .collect()),
            Value::String(string) => Ok(string
                .chars()
                .map(|char| Value::String(String::from(char)))
                .collect()),
            _ => Err(value.type_str()),
        }
    }
}
