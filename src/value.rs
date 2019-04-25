mod operation;
pub use operation::Operation;
mod invocation;
pub use invocation::Invocation;

use crate::parse::{ParseError, ParseResult};
use crate::lexer::{Lexer, Lexeme, BinaryOperator};

#[derive(Debug)]
pub enum Value
{
	Tag (String),
	Invocation (Invocation),
	Operation (Operation),
}

impl Value
{
	fn inner_parse (lexer: &mut Lexer) -> ParseResult<Self>
	{
		use Lexeme::*;

		match lexer.lexeme
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
			},
			OpenValue =>
			{
				lexer.advance();
				let value = Self::inner_parse(lexer)?;

				if let Lexeme::BinaryOperator(operator) = lexer.lexeme
				{
					return Ok(Value::Operation(Operation::parse_after(lexer, value, operator, true)?));
				}
				
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

		if let Lexeme::BinaryOperator(operator) = lexer.lexeme
		{
			return Ok(Value::Operation(Operation::parse_after(lexer, value, operator, false)?));
		}

		Ok(value)
	}
}