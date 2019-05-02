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

#[derive(Clone)]
pub struct Token
{
	pub lexeme: Lexeme,
	pub range: Range<usize>,
}

pub struct Lexer <'a>
{
	source: &'a [u8],
	pub token: Token,
	pub next_token: Token,
}

impl <'a> Lexer<'a>
{
	pub fn new (source: &'a str) -> Self
	{
		let mut lexer = Lexer
		{
			source: source.as_bytes(),
			token: Token{lexeme: Lexeme::None, range: 0..0},
			next_token: Token{lexeme: Lexeme::None, range: 0..0},
		};
		lexer.advance();
		lexer.advance();
		lexer
	}

	pub fn advance (&mut self)
	{
		self.token = self.next_token.clone();

		use Lexeme::*;
		use self::BinaryOperator::*;

		if self.next_token.range.end == self.source.len()
		{
			self.next_token.lexeme = None;
			return;
		}

		if self.source.get(self.next_token.range.end) == Some(&b' ')
		{
			self.next_token.range.end += 1;
		}

		self.next_token.range.start = self.next_token.range.end;

		if let Some(next) = self.source.get(self.next_token.range.end)
		{
			self.next_token.range.end += 1;

			self.next_token.lexeme = match *next
			{
				b'~' =>
				{
					if let Some(peek) = self.source.get(self.next_token.range.end)
					{
						match *peek
						{
							b'&' =>
							{
								self.next_token.range.end += 1;
								BinaryOperator(AlternativeDenial)
							},
							b'/' =>
							{
								self.next_token.range.end += 1;
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
					while let Some(peek) = self.source.get(self.next_token.range.end)
					{
						if peek.is_ascii_alphanumeric() || *peek == '-' as u8
						{
							self.next_token.range.end += 1;
						}
						else {break;}
					}
					Tag
				}
				b'+' =>
				{
					if let Some(peek) = self.source.get(self.next_token.range.end)
					{
						if b'>' == *peek
						{
							self.next_token.range.end += 1;
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
					if let Some(peek) = self.source.get(self.next_token.range.end)
					{
						match *peek
						{
							b'>' =>
							{
								self.next_token.range.end += 1;
								BinaryOperator(Implication)
							},
							b'<' =>
							{
								self.next_token.range.end += 1;
								BinaryOperator(ConverseNonimplication)
							},
							_ =>
							{
								while let Some(peek) = self.source.get(self.next_token.range.end)
								{
									if peek.is_ascii_alphanumeric() || *peek == '-' as u8
									{
										self.next_token.range.end += 1;
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
					if let Some(peek) = self.source.get(self.next_token.range.end)
					{
						match *peek
						{
							b'-' =>
							{
								self.next_token.range.end += 1;
								BinaryOperator(ConverseImplication)
							},
							b'>' =>
							{
								self.next_token.range.end += 1;
								BinaryOperator(Biconditional)
							},
							_ => Error,
						}
					}
					else {Error}
				},
				b'>' =>
				{
					if let Some(peek) = self.source.get(self.next_token.range.end)
					{
						match *peek
						{
							b'-' =>
							{
								self.next_token.range.end += 1;
								BinaryOperator(Nonimplication)
							},
							b'<' =>
							{
								self.next_token.range.end += 1;
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
		unsafe {std::str::from_utf8_unchecked(self.source.get_unchecked(self.token.range.clone()))} //TODO: handle UTF8 in advance() to make this safe.
	}
}