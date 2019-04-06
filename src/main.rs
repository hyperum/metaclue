#![feature(pattern)]

mod data;
mod parser;

use data::Value;
use parser::{Parser};

fn main()
{
	println!("{:?}", Value::parse("((obj) -one obj-two verb)").unwrap());
}