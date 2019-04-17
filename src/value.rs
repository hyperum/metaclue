use crate::parse::{ParseError, ParseResult};
use crate::lexer::{Lexer, Lexeme, LogicalOperator};
use std::convert::TryFrom;

#[derive(Debug)]
pub enum Value
{
	Tag (String),
	Invocation {map: Box<Self>, values: Vec<Self>},
	Operation {operator: LogicalOperator, values: Vec<Self>},
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
				let mut values = Vec::<Value>::new();
				let mut map = Option::<Value>::None;
				
				while lexer.lexeme != CloseInvocation
				{
					values.push(Self::inner_parse(lexer)?);

					if lexer.lexeme == MapSuffix
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
					
					return Ok(Value::Invocation{map: Box::new(map.unwrap_or_else(|| values.pop().unwrap())), values})
				}
			},
			OpenValue =>
			{
				lexer.advance();
				let value = Self::inner_parse(lexer)?;

				if let Ok(operator) = LogicalOperator::try_from(lexer.lexeme)
				{
					let mut values = Vec::new();
					values.push(value);

					lexer.advance();

					values.push(Self::inner_parse(lexer)?);

					if operator.is_associative()
					{
						while lexer.lexeme != Lexeme::CloseValue
						{
							if let Ok(next_operator) = LogicalOperator::try_from(lexer.lexeme)
							{
								if next_operator != operator
								{
									return Err(ParseError::ExpectedElement{element: "found different operator in same operation", slice: lexer.slice().to_string()});
								}
							}
							else
							{
								break;
							}

							lexer.advance();
							
							values.push(Self::inner_parse(lexer)?);
						}
					}
					
					lexer.advance();

					return Ok(Value::Operation{operator, values});
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

		if let Ok(operator) = LogicalOperator::try_from(lexer.lexeme)
		{
			let mut values = Vec::new();
			values.push(value);

			lexer.advance();

			values.push(Self::inner_parse(lexer)?);

			if operator.is_associative()
			{
				loop
				{
					if let Ok(next_operator) = LogicalOperator::try_from(lexer.lexeme)
					{
						if next_operator != operator
						{
							return Err(ParseError::ExpectedElement{element: "found different operator in same operation", slice: lexer.slice().to_string()});
						}

						lexer.advance()
					}
					else
					{
						break;
					}

					values.push(Self::inner_parse(lexer)?);
				}
				
				lexer.advance();
			}
			
			return Ok(Value::Operation{operator, values});
		}

		Ok(value)
	}
}