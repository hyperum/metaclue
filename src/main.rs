#![feature(type_alias_enum_variants)]

mod value;
mod parse;
mod lexer;

use value::Value;
use lexer::Lexer;

fn main()
{
	let mut lexer = Lexer::new("(every! {roman & soldier})");

	println!("{:?}, {:?}", Value::parse(&mut lexer), lexer.lexeme);
}