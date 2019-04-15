use crate::parser::{Parse, ParseError, ParseResult};
use crate::lexer::{Lexer, Lexeme};

#[derive(Debug)]
pub enum Value
{
	Tag(String),
	Invocation{map: Box<Self>, arguments: Vec<Self>},
}

impl Parse for Value
{
	fn parse (lexer: &mut Lexer) -> ParseResult<Self>
	{
		use Lexeme::*;
		match lexer.lexeme
		{
			Tag =>
			{
				Ok(Self::Tag(String::from(lexer.slice())))
			},
			OpenInvocation =>
			{
				lexer.advance();
				let mut arguments = Vec::<Value>::new();
				let mut map = Option::<Value>::None;
				
				while lexer.lexeme != CloseInvocation && lexer.lexeme != None
				{
					arguments.push(Value::parse(lexer)?); //TODO: handle error "properly" -> bubble up to expression level
					lexer.advance();

					if lexer.lexeme == MapSuffix
					{
						if map.is_some()
						{
							return Err(ParseError::ExpectedElement{element: "two maps designated in invocation", slice: lexer.slice().to_string()});
						}

						map = Some(arguments.pop().unwrap());
						lexer.advance();
					}
				}

				if arguments.len() == 0
				{
					return Err(ParseError::ExpectedElement{element: "nonempty invocation", slice: lexer.slice().to_string()});
				}
				else
				{
					return Ok(Value::Invocation{map: Box::new(map.unwrap_or(arguments.pop().unwrap())), arguments})
				}
			}
			_ =>
			{
				return Err(ParseError::ExpectedElement{element: "invocation or tagged value", slice: lexer.slice().to_string()})
			},
		}
	}
}