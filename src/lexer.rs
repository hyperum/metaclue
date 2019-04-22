#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Lexeme
{
	None,
	Error,

	Tag,
	
	OpenInvocation,
	OpenType,
	OpenValue,

	CloseInvocation,
	CloseType,
	CloseValue,

	Negation,

	BinaryOperator (BinaryOperator),

	To,
	In,
	Is,

	MapSuffix,
	Newline,
}

#[repr(u8)]
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum BinaryOperator
{
	Biconditional,
	ExclusiveDisjunction,
	Conjunction,
	Disjunction,

	AlternativeDenial,
	JointDenial,
	Implication,
	Nonimplication,
	ConverseImplication,
	ConverseNonimplication,
}

impl BinaryOperator
{
	pub fn is_associative (&self) -> bool
	{
		self >= &Self::Biconditional && self <= &Self::Disjunction
	}
}

use std::ops::Range;

pub struct Lexer <'a>
{
	source: &'a [u8],
	pub lexeme: Lexeme,
	pub range: Range<usize>,
}

impl <'a> Lexer<'a>
{
	pub fn new (source: &'a str) -> Self
	{
		let mut lexer = Lexer
		{
			source: source.as_bytes(),
			lexeme: Lexeme::None,
			range: 0..0,
		};
		lexer.advance();
		lexer
	}

	pub fn advance (&mut self)
	{
		use Lexeme::*;
		use self::BinaryOperator::*;

		if self.range.end == self.source.len()
		{
			self.lexeme = None;
			return;
		}

		if self.source.get(self.range.end) == Some(&b' ')
		{
			self.range.end += 1;
		}

		self.range.start = self.range.end;

		if let Some(next) = self.source.get(self.range.end)
		{
			self.range.end += 1;

			self.lexeme = match *next
			{
				b'~' =>
				{
					if let Some(peek) = self.source.get(self.range.end)
					{
						match *peek
						{
							b'&' =>
							{
								self.range.end += 1;
								BinaryOperator(AlternativeDenial)
							},
							b'/' =>
							{
								self.range.end += 1;
								BinaryOperator(JointDenial)
							},
							_ => Negation
						}
					}
					else {Negation}
				},
				b'&' => BinaryOperator(Conjunction),
				b'/' => BinaryOperator(Disjunction),
				b':' => In,
				b'=' => Is,
				b'(' => OpenInvocation,
				b'[' => OpenType,
				b'{' => OpenValue,
				b')' => CloseInvocation,
				b']' => CloseType,
				b'}' => CloseInvocation,
				next if next.is_ascii_alphanumeric() =>
				{
					while let Some(peek) = self.source.get(self.range.end)
					{
						if peek.is_ascii_alphanumeric() || *peek == '-' as u8
						{
							self.range.end += 1;
						}
						else {break;}
					}
					Tag
				}
				b'+' =>
				{
					if let Some(peek) = self.source.get(self.range.end)
					{
						if b'>' == *peek
						{
							To
						}
						else
						{
							Error
						}
					}
					else
					{
						Error
					}
				}
				b'-' =>
				{
					if let Some(peek) = self.source.get(self.range.end)
					{
						match *peek
						{
							b'>' =>
							{
								self.range.end += 1;
								BinaryOperator(Implication)
							},
							b'<' =>
							{
								self.range.end += 1;
								BinaryOperator(ConverseNonimplication)
							},
							_ =>
							{
								while let Some(peek) = self.source.get(self.range.end)
								{
									if peek.is_ascii_alphanumeric() || *peek == '-' as u8
									{
										self.range.end += 1;
									}
									else {break;}
								}
								Tag
							}
						}
					}
					else {Tag}
				},
				b'<' =>
				{
					if let Some(peek) = self.source.get(self.range.end)
					{
						match *peek
						{
							b'-' =>
							{
								self.range.end += 1;
								BinaryOperator(ConverseImplication)
							},
							b'>' =>
							{
								self.range.end += 1;
								BinaryOperator(Biconditional)
							},
							_ => Error,
						}
					}
					else {Error}
				},
				b'>' =>
				{
					if let Some(peek) = self.source.get(self.range.end)
					{
						match *peek
						{
							b'-' =>
							{
								self.range.end += 1;
								BinaryOperator(Nonimplication)
							},
							b'<' =>
							{
								self.range.end += 1;
								BinaryOperator(ExclusiveDisjunction)
							},
							_ => Error,
						}
					}
					else {Error}
				},
				b'!' => MapSuffix,
				b'\n' => Newline,
				_ =>
				{
					Error
				}
			};
		}
	}

	pub fn slice (&self) -> &'a str
	{
		unsafe {std::str::from_utf8_unchecked(self.source.get_unchecked(self.range.clone()))} //TODO: handle UTF8 in advance() to make this safe.
	}

	pub fn skip_newlines (&mut self) -> usize
	{
		let mut count = 0;

		while Some(&b'\n') == self.source.get(self.range.end)
		{
			self.range.end += 1;
			count += 1;
		}
		
		return count;
	}
}