use std::fmt;
use std::fmt::{Debug, Formatter};

use crate::Occur;

#[derive(PartialEq)]
pub struct UserInputField {
    pub name: String,
    pub rank: u32,
}

impl Debug for UserInputField {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> Result<(), fmt::Error> {
        write!(formatter, "{}#{}", self.name, self.rank)
    }
}

#[derive(PartialEq)]
pub enum UserInputLeaf {
    Literal(UserInputLiteral),
    All,
    Range {
        field: Option<UserInputField>,
        lower: UserInputBound,
        upper: UserInputBound,
    },
}

impl Debug for UserInputLeaf {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> Result<(), fmt::Error> {
        match self {
            UserInputLeaf::Literal(literal) => literal.fmt(formatter),
            UserInputLeaf::Range {
                ref field,
                ref lower,
                ref upper,
            } => {
                if let Some(ref field) = field {
                    write!(formatter, "{:?}:", field)?;
                }
                lower.display_lower(formatter)?;
                write!(formatter, " TO ")?;
                upper.display_upper(formatter)?;
                Ok(())
            }
            UserInputLeaf::All => write!(formatter, "*"),
        }
    }
}

#[derive(PartialEq)]
pub struct UserInputLiteral {
    pub field: Option<UserInputField>,
    pub phrase: String,
}

impl fmt::Debug for UserInputLiteral {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        match self.field {
            Some(ref field) => write!(formatter, "{}:\"{}\"", field.name, self.phrase),
            None => write!(formatter, "\"{}\"", self.phrase),
        }
    }
}

#[derive(PartialEq)]
pub enum UserInputBound {
    Inclusive(String),
    Exclusive(String),
    Unbounded,
}

impl UserInputBound {
    fn display_lower(&self, formatter: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        match *self {
            UserInputBound::Inclusive(ref word) => write!(formatter, "[\"{}\"", word),
            UserInputBound::Exclusive(ref word) => write!(formatter, "{{\"{}\"", word),
            UserInputBound::Unbounded => write!(formatter, "{{\"*\""),
        }
    }

    fn display_upper(&self, formatter: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        match *self {
            UserInputBound::Inclusive(ref word) => write!(formatter, "\"{}\"]", word),
            UserInputBound::Exclusive(ref word) => write!(formatter, "\"{}\"}}", word),
            UserInputBound::Unbounded => write!(formatter, "\"*\"}}"),
        }
    }

    pub fn term_str(&self) -> &str {
        match *self {
            UserInputBound::Inclusive(ref contents) => contents,
            UserInputBound::Exclusive(ref contents) => contents,
            UserInputBound::Unbounded => &"*",
        }
    }
}

pub enum UserInputAST {
    Clause(Vec<UserInputAST>),
    Unary(Occur, Box<UserInputAST>),
    Leaf(Box<UserInputLeaf>),
}

impl UserInputAST {
    pub fn unary(self, occur: Occur) -> UserInputAST {
        UserInputAST::Unary(occur, Box::new(self))
    }

    fn compose(occur: Occur, asts: Vec<UserInputAST>) -> UserInputAST {
        assert_ne!(occur, Occur::MustNot);
        assert!(!asts.is_empty());
        if asts.len() == 1 {
            asts.into_iter().next().unwrap() //< safe
        } else {
            UserInputAST::Clause(
                asts.into_iter()
                    .map(|ast: UserInputAST| ast.unary(occur))
                    .collect::<Vec<_>>(),
            )
        }
    }

    pub fn empty_query() -> UserInputAST {
        UserInputAST::Clause(Vec::default())
    }

    pub fn and(asts: Vec<UserInputAST>) -> UserInputAST {
        UserInputAST::compose(Occur::Must, asts)
    }

    pub fn or(asts: Vec<UserInputAST>) -> UserInputAST {
        UserInputAST::compose(Occur::Should, asts)
    }
}

impl From<UserInputLiteral> for UserInputLeaf {
    fn from(literal: UserInputLiteral) -> UserInputLeaf {
        UserInputLeaf::Literal(literal)
    }
}

impl From<UserInputLeaf> for UserInputAST {
    fn from(leaf: UserInputLeaf) -> UserInputAST {
        UserInputAST::Leaf(Box::new(leaf))
    }
}

impl fmt::Debug for UserInputAST {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        match *self {
            UserInputAST::Clause(ref subqueries) => {
                if subqueries.is_empty() {
                    write!(formatter, "<emptyclause>")?;
                } else {
                    write!(formatter, "(")?;
                    write!(formatter, "{:?}", &subqueries[0])?;
                    for subquery in &subqueries[1..] {
                        write!(formatter, " {:?}", subquery)?;
                    }
                    write!(formatter, ")")?;
                }
                Ok(())
            }
            UserInputAST::Unary(ref occur, ref subquery) => {
                write!(formatter, "{}({:?})", occur, subquery)
            }
            UserInputAST::Leaf(ref subquery) => write!(formatter, "{:?}", subquery),
        }
    }
}
