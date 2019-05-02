use crate::parse::{ParseError, ParseResult};
use crate::lexer::{Lexer, Lexeme, BinaryOperator};
use crate::value::Value;

#[derive(Debug)]
pub enum Type
{
	Tag(String),
	Map(Vec<Self>, Box<Self>),
	Implicit,
}

impl Type
{
	pub fn initializes (lexer: &Lexer) -> bool
	{
		use Lexeme::*;
		lexer.token.lexeme == Tag || lexer.token.lexeme == OpenType
	}

	pub fn parse (lexer: &mut Lexer) -> ParseResult<Self>
	{
		if lexer.token.lexeme == Lexeme::Tag
		{
			let tag = Self::Tag(lexer.slice().to_string());
			lexer.advance();
			return Ok(tag);
		}

		else if lexer.token.lexeme != Lexeme::OpenType
		{
			return Err(ParseError::ExpectedElement{element: "[", slice: lexer.slice().to_string()});
		}

		lexer.advance();

		let mut parameters = Vec::new();

		while lexer.token.lexeme != Lexeme::To
		{
			parameters.push(Self::parse(lexer)?);
		}

		lexer.advance();

		if lexer.token.lexeme != Lexeme::Tag
		{
			return Err(ParseError::ExpectedElement{element: "return type of map", slice: lexer.slice().to_string()});
		}

		//lexer.advance(); //FIXME: Don't assume close bracket.

		let r#return = Self::parse(lexer)?;

		lexer.advance();

		Ok(Self::Map(parameters, Box::new(r#return)))
	}
}