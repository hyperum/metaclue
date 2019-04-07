#![feature(pattern)]
#![feature(bind_by_move_pattern_guards)]

mod data;
mod parser;

use data::Value;
use parser::{Parser};

fn main()
{
	println!("{:?}", Value::parse("(() -one obj-two verb)").unwrap());
}