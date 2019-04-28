#![feature(type_alias_enum_variants)]

mod value;
mod parse;
mod lexer;
mod statement;
mod assignment;

use value::Value;
use lexer::Lexer;
use statement::Statement;

fn main()
{
	let mut lexer = Lexer::new("me & (every! {{roman & soldier}})");

	println!("{:?}, {:?}", Value::parse(&mut lexer), lexer.token.lexeme);

	let mut lexer = Lexer::new("this-shall-be-true = me & (every! {{roman & soldier}})");
	println!("{:?}, {:?}", Statement::parse(&mut lexer), lexer.token.lexeme);
}