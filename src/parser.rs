use crate::lexer::Lexer;

#[derive(Debug)]
pub enum ParseError
{
	ExpectedSequence {sequence: &'static str, slice: String},
	ExpectedElement {element: &'static str, slice: String},
}

pub type ParseResult <Type> = Result<Type, ParseError>;

pub trait Parse: Sized
{
	fn parse (lexer: &mut Lexer) -> ParseResult<Self>;
}