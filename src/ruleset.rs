use crate::value::{Value};
use crate::lexer::{Lexer, Lexeme};
use crate::parse::{ParseError, ParseResult};
use crate::statement::{Statement};

#[derive(Debug)]
pub struct Ruleset (Vec<Statement>);

impl Ruleset
{
	pub fn parse (lexer: &mut Lexer) -> ParseResult<Self>
	{
		let mut ruleset = Ruleset(Vec::new());

		while lexer.token.lexeme == Lexeme::Newline
		{
			lexer.advance();
		}

		while Statement::initializes(lexer)
		{
			ruleset.0.push(Statement::parse(lexer)?);

			while lexer.token.lexeme == Lexeme::Newline
			{
				lexer.advance();
			}
		}

		Ok(ruleset)
	}
}