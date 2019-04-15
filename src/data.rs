use crate::parser::{Parse, ParseError, ParseResult};
use crate::lexer::{Lexer, Lexeme};

#[derive(Debug)]
pub enum Value
{
	None,
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
				let mut arguments: Vec<Value> = Vec::new();
				let mut map: Value = Value::None;
				let mut has_found_map_yet = false;
				while lexer.lexeme != CloseInvocation && lexer.lexeme != None
				{
					arguments.push(Value::parse(lexer)?); //TODO: handle error "properly" -> bubble up to expression level
					lexer.advance();

					if lexer.lexeme == MapSuffix
					{
						if has_found_map_yet
						{
							return Err(ParseError::ExpectedElement{element: "two maps designated in invocation", slice: lexer.slice().to_string()});
						}

						map = arguments.pop().unwrap();
						has_found_map_yet = true;
						lexer.advance();
					}
				}

				if arguments.len() == 0
				{
					return Err(ParseError::ExpectedElement{element: "nonempty invocation", slice: lexer.slice().to_string()});
				}
				if !has_found_map_yet
				{
					map = arguments.pop().unwrap(); //TODO: handle empty invocation
				}

				Ok(Value::Invocation{map: Box::new(map), arguments})
			}
			_ =>
			{
				Err(ParseError::ExpectedElement{element: "invocation or tagged value", slice: lexer.slice().to_string()})
			},
		}
	}
}