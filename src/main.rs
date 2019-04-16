#![feature(try_from)]
#![feature(type_alias_enum_variants)]

mod value;
mod parser;
mod lexer;

use value::Value;
use parser::Parse;
use lexer::Lexer;

fn main()
{
	let mut lexer = Lexer::new("(i eat! potatoes) & (you eat! potatoes)");

	println!("{:?}, {:?}", Value::parse(&mut lexer), lexer.lexeme);
}