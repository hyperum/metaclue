use crate::parser::{Parser, ParserError, Source};

#[derive(Debug)]
pub struct Tag (pub String);

impl <'a> Parser<'a> for Tag
{
	fn parse (source: Source<'a>) -> Result<(Source<'a>, Self), ParserError<'a>>
	{
		let (source, sequence) = source.collect("letter", |c| c.is_alphabetic() || c == '-')?;
		Ok((source, Self(String::from(sequence))))
	}
}

#[derive(Debug)]
pub struct Invocation
{
	pub map: Value,
	pub arguments: Vec<Value>,
}

impl <'a> Parser<'a> for Invocation
{
	fn parse (source: Source<'a>) -> Result<(Source<'a>, Self), ParserError<'a>>
	{
		let mut source = source.consume("(")?;

		let mut arguments: Vec<Value> = vec![];

		loop
		{
			source = source.skip(' ');

			if source.0.starts_with(')') {break;}

			let source_value = Value::parse(source)?;
			source = source_value.0;
			let value = source_value.1;

			arguments.push(value);
		}

		let source = source.skip(' ').consume(")")?;

		if arguments.is_empty()
		{
			Err(ParserError::ExpectedElement{element: "nonempty invocation", source})
		}
		else
		{
			Ok((source, Invocation{map: arguments.pop().unwrap(), arguments}))
		}
	}
}

#[derive(Debug)]
pub enum Value
{
	Tag(Tag),
	Invocation(Box<Invocation>),
}

impl <'a> Parser<'a> for Value
{
	fn parse (source: Source<'a>) -> Result<(Source<'a>, Self), ParserError<'a>>
	{
		if let Ok((source, tag)) = Tag::parse(source)
		{
			Ok((source, Value::Tag(tag)))
		}
		else if let Ok((source, invocation)) = Invocation::parse(source)
		{
			Ok((source, Value::Invocation(Box::new(invocation))))
		}
		else
		{
			Err(ParserError::ExpectedElement{element: "value", source})
		}
	}
}