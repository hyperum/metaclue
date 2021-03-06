use crate::operation::Operation;
use crate::invocation::Invocation;

use crate::parse::{ParseError, ParseResult};
use crate::lexer::{Lexer, Lexeme, BinaryOperator};

#[derive(Debug)]
pub enum Value
{
	Tag (String),
	Invocation (Invocation),
	Operation (Operation),
	Implicit,
}

impl Value
{
	pub fn initializes (lexer: &Lexer) -> bool
	{
		use Lexeme::*;
		lexer.token.lexeme == Tag || lexer.token.lexeme == OpenInvocation || lexer.token.lexeme == OpenValue
	}

	pub fn inner_parse (lexer: &mut Lexer) -> ParseResult<Self>
	{
		use Lexeme::*;

		match lexer.token.lexeme
		{
			Tag =>
			{
				let tag = Self::Tag(String::from(lexer.slice()));

				lexer.advance();

				return Ok(tag);
			},
			OpenInvocation =>
			{
				lexer.advance();
				
				return Ok(Value::Invocation(Invocation::parse(lexer)?));
				// TODO: we should handle the closeinvocation too, or both this and openvalue should handle it themselves.
			},
			OpenValue =>
			{
				lexer.advance();
				let value = Self::parse(lexer)?;
				lexer.advance(); //FIXME: Actually check that the closevalue is received.
				
				return Ok(value);
			},
			_ =>
			{
				return Err(ParseError::ExpectedElement{element: "invocation or tagged value", slice: lexer.slice().to_string()});
			},
		};
	}
	pub fn parse (lexer: &mut Lexer) -> ParseResult<Self>
	{
		let value = Self::inner_parse(lexer)?;

		if let Lexeme::BinaryOperator(operator) = lexer.token.lexeme
		{
			return Ok(Value::Operation(Operation::parse_after(lexer, value, operator)?));
		}

		Ok(value)
	}
}