use crate::number_word_parser::NumberWordParser;

use std::str::FromStr;

pub struct OrdinalNumberWordParser;

impl OrdinalNumberWordParser {
	fn lookup(number_string: &str) -> Result<u64, String> {
		let number_string = &number_string.to_lowercase();

		let number = match &number_string[..] {
			"first" => 1,
			"second" => 2,
			"third" => 3,
			"fourth" => 4,
			"fifth" => 5,
			"sixth" => 6,
			"seventh" => 7,
			"eighth" => 8,
			"ninth" => 9,
			"tenth" => 10,
			"eleventh" => 11,
			"twelfth" => 12,
			"thirteenth" => 13,
			"fourteenth" => 14,
			"fifteenth" => 15,
			"sixteenth" => 16,
			"seventeenth" => 17,
			"eighteenth" => 18,
			"nineteenth" => 19,
			"twentieth" => 20,
			"thirtieth" => 30,
			"fourtieth" | "fortieth" => 40,
			"fiftieth" => 50,
			"sixtieth" => 60,
			"seventieth" => 70,
			"eightieth" => 80,
			"ninetieth" => 90,
			"hundreth" => 100,
			"thousandth" => 1_000,
			"millionth" => 1_000_000,
			"billionth" => 1_000_000_000,
			e => return Err(format!("\"{e}\" is not a valid ordinal numerical word.")),
		};

		return Ok(number);
	}

	fn lookup_number_word(number: u64) -> Result<String, String> {
		let number_string = match number {
			1 => "one",
			2 => "two",
			3 => "three",
			4 => "four",
			5 => "five",
			6 => "six",
			7 => "seven",
			8 => "eight",
			9 => "nine",
			10 => "ten",
			11 => "eleven",
			12 => "twelve",
			13 => "thirteen",
			14 => "fourteen",
			15 => "fifteen",
			16 => "sixteen",
			17 => "seventeen",
			18 => "eighteen",
			19 => "nineteen",
			20 => "twenty",
			30 => "thirty",
			40 => "forty",
			50 => "fifty",
			60 => "sixty",
			70 => "seventy",
			80 => "eighty",
			90 => "ninety",
			100 => "hundred",
			1_000 => "thousand",
			1_000_000 => "million",
			1_000_000_000 => "billion",
			_ => return Err(format!("\"{number}\" is not a valid number.")),
		};

		Ok(number_string.to_string())
	}

	pub fn parse_from_args(args: &[String]) -> Result<u64, String> {
		let last_arg_index = args.len() - 1;
		let last_arg = &args[last_arg_index];
		let number = Self::lookup(last_arg)?;
		let number_word = Self::lookup_number_word(number).expect("Should never fail.");
		let mut new_args = args.to_vec();
		new_args[last_arg_index] = number_word;
		NumberWordParser::parse_from_args(&new_args[..])
	}
}

// #[cfg(test)]
// mod tests {
// 	use super::*;

// 	#[test]
// 	fn test_ordinal_number_parser_normal() {
// 		assert_eq!(OrdinalNumberParser::parse("1st"), Ok(1));
// 		assert_eq!(OrdinalNumberParser::parse("2nd"), Ok(2));
// 		assert_eq!(OrdinalNumberParser::parse("3rd"), Ok(3));
// 		assert_eq!(OrdinalNumberParser::parse("115th"), Ok(115));
// 		assert_eq!(OrdinalNumberParser::parse("4th"), Ok(4));
// 		assert_eq!(OrdinalNumberParser::parse("15th"), Ok(15));
// 	}

// 	/// The ordinal number parser should not care about correctness.
// 	#[test]
// 	fn test_ordinal_number_parser_strange() {
// 		assert_eq!(OrdinalNumberParser::parse("1rd"), Ok(1));
// 		assert_eq!(OrdinalNumberParser::parse("2th"), Ok(2));
// 		assert_eq!(OrdinalNumberParser::parse("3st"), Ok(3));
// 		assert_eq!(OrdinalNumberParser::parse("115st"), Ok(115));
// 		assert_eq!(OrdinalNumberParser::parse("4rd"), Ok(4));
// 		assert_eq!(OrdinalNumberParser::parse("15nd"), Ok(15));
// 	}
// }
