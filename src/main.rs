#![feature(try_from)]
#![feature(type_alias_enum_variants)]

mod data;
mod parser;
mod lexer;

use data::Value;
use parser::Parse;
use lexer::Lexer;

fn main()
{
	let mut lexer = Lexer::new("((test1 test2! te3st test4))");

	println!("{:?}", Value::parse(&mut lexer))
}