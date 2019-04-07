#[derive(Debug)]
pub enum ParserError <'a>
{
	ExpectedSequence {sequence: &'static str, source: &'a str},
	ExpectedElement {element: &'static str, source: &'a str},
	None,
}

pub type ParserResult <'a, Type> = Result<(&'a str, Type), ParserError<'a>>;

pub trait OptionalParserResult <'a, Type>
{
	fn as_option (self) -> Option<ParserResult<'a, Type>>;
}

impl <'a, Type> OptionalParserResult<'a, Type> for ParserResult<'a, Type>
{
	fn as_option (self) -> Option<ParserResult<'a, Type>>
	{
		match self
		{
			Ok(ok) => Some(Ok(ok)),
			Err(err) if std::mem::discriminant(&err) != std::mem::discriminant(&ParserError::None) => Some(Err(err)),
			_ => None,
		}
	}
}

pub trait Parser <'a>: Sized
{
	fn parse (input: &'a str) -> ParserResult<'a, Self>;
}

pub fn consume <'a> (source: &'a str, sequence: &'static str) -> Result<&'a str, ParserError<'a>>
{
	if source.starts_with(sequence)
	{
		Ok(&source[sequence.len()..])
	}
	else
	{
		Err(ParserError::ExpectedSequence{sequence: sequence, source: source})
	}
}

pub fn skip <'a> (source: &'a str, character: char) -> &'a str
{
	source.trim_start_matches(character)
}

pub fn collect <'a, Indicator: Fn(char) -> bool>(source: &'a str, element: &'static str, indicator: Indicator) -> ParserResult<'a, &'a str>
{
	for (i, token) in source.chars().enumerate()
	{
		if !indicator(token)
		{
			if i != 0
			{
				return Ok((&source[i..], &source[..i]));
			}
			else
			{
				break;
			}
		}
	}

	Err(ParserError::ExpectedElement{element, source})
}