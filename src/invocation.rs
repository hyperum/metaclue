use crate::parse::{ParseError, ParseResult};
use crate::lexer::{Lexer, Lexeme};
use crate::value::Value;

#[derive(Debug)]
pub struct Invocation {map: Box<Value>, values: Vec<Value>}

impl Invocation
{
	pub fn parse (lexer: &mut Lexer) -> ParseResult<Self>
	{
		let mut values = Vec::<Value>::new();
		let mut map = Option::<Value>::None;
		
		while lexer.token.lexeme != Lexeme::CloseInvocation
		{
			values.push(Value::inner_parse(lexer)?);

			if lexer.token.lexeme == Lexeme::MapSuffix
			{
				if map.is_some()
				{
					return Err(ParseError::ExpectedElement{element: "two maps designated in invocation", slice: lexer.slice().to_string()});
				}

				lexer.advance();
				map = Some(values.pop().unwrap());
			}
		}

		if values.len() == 0
		{
			return Err(ParseError::ExpectedElement{element: "nonempty invocation", slice: lexer.slice().to_string()});
		}
		else
		{
			lexer.advance();
			
			return Ok(Self{map: Box::new(map.unwrap_or_else(|| values.pop().unwrap())), values})
		}
	}
}