#![feature(type_alias_enum_variants)]

mod operation;
mod invocation;
mod value;
mod parse;
mod lexer;
mod statement;
mod assignment;
mod ruleset;
mod r#type;

use value::Value;
use lexer::Lexer;
use statement::Statement;
use ruleset::Ruleset;

fn main()
{
	let mut lexer = Lexer::new("me & (every! {{roman & soldier}})");

	println!("{:?}, {:?}", Value::parse(&mut lexer), lexer.token.lexeme);

	let mut lexer = Lexer::new("this-shall-be-true = me & (every! {{roman & soldier}})");
	println!("{:?}, {:?}", Statement::parse(&mut lexer), lexer.token.lexeme);

	let mut lexer = Lexer::new("n = a ~/ (b c)\nr >< n\nx : [E E [E +> V] V +> V] = y / z");
	println!("{:?}, {:?}", Ruleset::parse(&mut lexer), lexer.token.lexeme);
}