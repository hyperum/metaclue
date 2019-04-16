#[derive(Debug)]
pub enum ParseError
{
	ExpectedSequence {sequence: &'static str, slice: String},
	ExpectedElement {element: &'static str, slice: String},
}

pub type ParseResult <Type> = Result<Type, ParseError>;