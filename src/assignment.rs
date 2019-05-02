use crate::value::{Value};
use crate::lexer::{Lexer, Lexeme};
use crate::parse::{ParseError, ParseResult};
use crate::r#type::Type;

#[derive(Debug)]
pub struct Assignment
{
	tag: String,
	r#type: Type,
	value: Value,
}

impl Assignment
{
	pub fn initializes (lexer: &Lexer) -> bool
	{
		lexer.token.lexeme == Lexeme::Tag && (lexer.next_token.lexeme == Lexeme::Is || lexer.next_token.lexeme == Lexeme::In)
	}

	pub fn parse (lexer: &mut Lexer) -> ParseResult<Self> //FIXME: enforce that has_initial is called beforehand, or that the initial values truly are that way.
	{
		let tag = String::from(lexer.slice());

		lexer.advance();

		if lexer.token.lexeme == Lexeme::Is
		{
			lexer.advance();

			return Ok(Assignment{tag, r#type: Type::Implicit, value: Value::parse(lexer)?});
		}

		else if lexer.token.lexeme == Lexeme::In
		{
			lexer.advance();

			let r#type = Type::parse(lexer)?;

			if lexer.token.lexeme != Lexeme::Is
			{
				return Ok(Assignment{tag, r#type, value: Value::Implicit})
			}

			lexer.advance();

			return Ok(Assignment{tag, r#type, value: Value::parse(lexer)?})
		}

		Err(ParseError::ExpectedElement{element: "nonempty assignment", slice: String::from(lexer.slice())})
	}
}