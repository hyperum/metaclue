use crate::value::{Value};
use crate::lexer::{Lexer, Lexeme};
use crate::parse::{ParseError, ParseResult};
use crate::assignment::{Assignment};

#[derive(Debug)]
pub enum Statement
{
	Value (Value),
	Assignment (Assignment),
}

impl Statement
{
	pub fn initializes (lexer: &mut Lexer) -> bool
	{
		Value::initializes(lexer) | Assignment::initializes(lexer)
	}

	pub fn parse (lexer: &mut Lexer) -> ParseResult<Self>
	{
		if Assignment::initializes(lexer)
		{
			return Ok(Self::Assignment(Assignment::parse(lexer)?));
		}
		else if Value::initializes(lexer)
		{
			return Ok(Self::Value(Value::parse(lexer)?));
		}

		Err(ParseError::ExpectedElement{element: "statement", slice: String::from(lexer.slice())})
	}
}