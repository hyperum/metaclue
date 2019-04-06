mod data;
mod parser;

use data::Invocation;
use parser::{Parser, Source};

fn main()
{
	println!("{:?}", Invocation::parse(Source("(obj-one obj-two verb)")).unwrap());
}