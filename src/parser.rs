#[derive(Debug)]
pub enum ParserError <'a>
{
	ExpectedSequence {sequence: &'static str, source: &'a str},
	ExpectedElement {element: &'static str, source: &'a str},
}

pub trait Parser <'a>: Sized
{
	fn parse (input: &'a str) -> Result<(&'a str, Self), ParserError<'a>>;
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

pub fn collect <'a, Indicator: Fn(char) -> bool>(source: &'a str, element: &'static str, indicator: Indicator) -> Result<(&'a str, &'a str), ParserError<'a>>
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