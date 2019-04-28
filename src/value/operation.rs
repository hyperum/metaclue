use crate::parse::{ParseError, ParseResult};
use crate::lexer::{Lexer, Lexeme, BinaryOperator};
use crate::value::Value;

#[derive(Debug)]
pub struct Operation {operator: BinaryOperator, values: Vec<Value>}

impl Operation
{
	pub fn parse_after (lexer: &mut Lexer, last: Value, operator: BinaryOperator) -> ParseResult<Self>
	{
		lexer.advance();

		let mut values = Vec::new();
		values.push(last);
		values.push(Value::inner_parse(lexer)?);

		if operator.is_associative()
		{
			while lexer.token.lexeme != Lexeme::None
			{
				if let Lexeme::BinaryOperator(next_operator) = lexer.token.lexeme
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
				
				values.push(Value::inner_parse(lexer)?);
			}
		}

		return Ok(Self{operator, values});
	}
}