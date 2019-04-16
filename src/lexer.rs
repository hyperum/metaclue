#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Lexeme
{
	None = 0,
	Error,

	Tag,
	
	OpenInvocation,
	OpenType,
	OpenValue,

	CloseInvocation,
	CloseType,
	CloseValue,

	Negation,

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
	
	To,
	In,
	Is,

	MapSuffix,
	Newline,
}

#[repr(u8)]
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum LogicalOperator
{
	Negation = Lexeme::Negation as u8,

	Biconditional = Lexeme::Biconditional as u8,
	ExclusiveDisjunction = Lexeme::ExclusiveDisjunction as u8,
	Conjunction = Lexeme::Conjunction as u8,
	Disjunction = Lexeme::Disjunction as u8,

	AlternativeDenial = Lexeme::AlternativeDenial as u8,
	JointDenial = Lexeme::JointDenial as u8,
	Implication = Lexeme::Implication as u8,
	Nonimplication = Lexeme::Nonimplication as u8,
	ConverseImplication = Lexeme::ConverseImplication as u8,
	ConverseNonimplication = Lexeme::ConverseNonimplication as u8,
}

impl LogicalOperator
{
	pub fn is_associative (&self) -> bool
	{
		self >= &Self::Biconditional && self <= &Self::Disjunction
	}
}

use std::convert::TryFrom;
impl TryFrom<Lexeme> for LogicalOperator
{
	type Error = ();
	fn try_from (lexeme: Lexeme) -> Result<Self, Self::Error>
	{
		if lexeme >= Lexeme::Negation && lexeme <= Lexeme::ConverseNonimplication
		{
			return Ok(unsafe {std::mem::transmute(lexeme)});
		}
		else
		{
			return Err(());
		}
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
								AlternativeDenial
							},
							b'/' =>
							{
								self.range.end += 1;
								JointDenial
							},
							_ => Negation
						}
					}
					else {Negation}
				},
				b'&' => Conjunction,
				b'/' => Disjunction,
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
				b'-' =>
				{
					if let Some(peek) = self.source.get(self.range.end)
					{
						match *peek
						{
							b'>' =>
							{
								self.range.end += 1;
								Implication
							},
							b'<' =>
							{
								self.range.end += 1;
								ConverseNonimplication
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
								ConverseImplication
							},
							b'>' =>
							{
								self.range.end += 1;
								Biconditional
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
								Nonimplication
							},
							b'<' =>
							{
								self.range.end += 1;
								ExclusiveDisjunction
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