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
	pub fn parse (lexer: &mut Lexer) -> ParseResult<Self>
	{
		if Assignment::has_initial(lexer)
		{
			return Ok(Self::Assignment(Assignment::parse(lexer)?));
		}

		match lexer.token.lexeme
		{
			lexeme if Value::is_initial(lexeme) =>
			{
				Ok(Self::Value(Value::parse(lexer)?))
			}
			_ =>
			{
				Err(ParseError::ExpectedElement{element: "statement", slice: String::from(lexer.slice())})
			}
		}
	}
}