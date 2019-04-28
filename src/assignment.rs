use crate::value::{Value};
use crate::lexer::{Lexer, Lexeme};
use crate::parse::{ParseError, ParseResult};

#[derive(Debug)]
pub struct Assignment
{
	tag: String,
	value: Value,
}

impl Assignment
{
	pub fn has_initial (lexer: &mut Lexer) -> bool
	{
		lexer.token.lexeme == Lexeme::Tag && lexer.next_token.lexeme == Lexeme::Is
	}

	pub fn parse (lexer: &mut Lexer) -> ParseResult<Self> //FIXME: enforce that has_initial is called beforehand, or that the initial values truly are that way.
	{
		let tag = String::from(lexer.slice());

		lexer.advance();

		lexer.advance();

		match lexer.token.lexeme
		{
			lexeme if Value::is_initial(lexeme) =>
			{
				return Ok(Assignment{tag, value: Value::parse(lexer)?});
			},
			_ =>
			{
				return Err(ParseError::ExpectedElement{element: "nonempty assignment", slice: String::from(lexer.slice())})
			}
		}
	}
}