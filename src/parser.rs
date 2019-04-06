#[derive(Debug)]
pub enum ParserError <'a>
{
	ExpectedSequence {sequence: &'static str, source: Source<'a>},
	ExpectedElement {element: &'static str, source: Source<'a>},
}

pub trait Parser <'a>: Sized
{
	fn parse (input: Source<'a>) -> Result<(Source<'a>, Self), ParserError<'a>>;
}

#[derive(Debug)]
pub struct Source <'a> (pub &'a str);

impl <'a> Source<'a>
{
	pub fn consume (self, sequence: &'static str) -> Result<Self, ParserError<'a>>
	{
		if self.0.starts_with(sequence)
		{
			Ok(Self(&self.0[sequence.len()..]))
		}
		else
		{
			Err(ParserError::ExpectedSequence{sequence: sequence, source: self})
		}
	}

	pub fn skip (self, character: char) -> Self
	{
		Self(self.0.trim_start_matches(character))
	}

	pub fn collect <Indicator: Fn(char) -> bool>(self, element: &'static str, indicator: Indicator) -> Result<(Self, &'a str), ParserError<'a>>
	{
		let mut chars = self.0.chars();

		loop
		{
			let rest = chars.as_str();
			match chars.next()
			{
				Some(c) if indicator(c) => {},
				_ =>
				{
					if rest.len() != self.0.len()
					{
						return Ok((Self(rest), &self.0[..self.0.len() - rest.len()]));
					}
					else
					{
						return Err(ParserError::ExpectedElement{element, source: self});
					}
				}
			}
		}
	}
}