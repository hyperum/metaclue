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
	pub map: Tag,
	pub arguments: Vec<Tag>,
}

impl <'a> Parser<'a> for Invocation
{
	fn parse (source: Source<'a>) -> Result<(Source<'a>, Self), ParserError<'a>>
	{
		let mut source = source.consume("(")?;

		let mut arguments: Vec<Tag> = vec![];

		loop
		{
			source = source.skip(' ');

			if source.0.starts_with(')') {break;}

			let source_tag = Tag::parse(source)?;
			source = source_tag.0;
			let tag = source_tag.1;

			arguments.push(tag);
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