#![feature(pattern)]

mod data;
mod parser;

use data::Value;
use parser::{Parser, Source};

fn main()
{
	println!("{:?}", Value::parse(Source("((obj) -one obj-two verb)")).unwrap());
}