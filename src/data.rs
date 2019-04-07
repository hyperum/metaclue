use crate::parser::{Parser, ParserError, ParserResult, OptionalParserResult, consume, skip, collect};

#[derive(Debug)]
pub struct Tag (pub String);

impl <'a> Parser<'a> for Tag
{
	fn parse (source: &'a str) -> ParserResult<'a, Self>
	{
		let (source, sequence) = collect(source, "letter or dash", |c| c.is_alphabetic() || c == '-').or(Err(ParserError::None))?;
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
	fn parse (source: &'a str) -> ParserResult<'a, Self>
	{
		let original_source = source;
		let mut source = consume(source, "(").or(Err(ParserError::None))?;

		let mut arguments: Vec<Value> = vec![];

		loop
		{
			source = skip(source, ' ');

			if source.starts_with(')') {break;}

			let source_value = Value::parse(source)?;
			source = source_value.0;
			let value = source_value.1;

			arguments.push(value);
		}

		let source = consume(skip(source, ' '), ")")?;

		if arguments.is_empty()
		{
			Err(ParserError::ExpectedElement{element: "nonempty invocation", source: original_source})
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
	fn parse (source: &'a str) -> ParserResult<'a, Self>
	{
		if let Some(result) = Tag::parse(source).as_option()
		{
			let (source, tag) = result?;
			Ok((source, Value::Tag(tag)))
		}
		else if let Some(result) = Invocation::parse(source).as_option()
		{
			let (source, invocation) = result?;
			Ok((source, Value::Invocation(Box::new(invocation))))
		}
		else
		{
			Err(ParserError::ExpectedElement{element: "value", source})
		}
	}
}