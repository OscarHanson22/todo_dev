pub struct NumberWordParser;

impl NumberWordParser {
	fn lookup(number_string: &str) -> Result<u64, String> {
		let number_string = &number_string.to_lowercase();

		let number = match &number_string[..] {
			"one" | "a" => 1,
			"two" => 2,
			"three" => 3,
			"four" => 4,
			"five" => 5,
			"six" => 6,
			"seven" => 7,
			"eight" => 8,
			"nine" => 9,
			"ten" => 10,
			"eleven" => 11,
			"twelve" => 12,
			"thirteen" => 13,
			"fourteen" => 14,
			"fifteen" => 15,
			"sixteen" => 16,
			"seventeen" => 17,
			"eighteen" => 18,
			"nineteen" => 19,
			"twenty" => 20,
			"thirty" => 30,
			"fourty" | "forty" => 40,
			"fifty" => 50,
			"sixty" => 60,
			"seventy" => 70,
			"eighty" => 80,
			"ninety" => 90,
			"hundred" => 100,
			"thousand" => 1_000,
			"million" => 1_000_000,
			"billion" => 1_000_000_000,
			e => return Err(format!("\"{e}\" is not a valid numerical word.")),
		};

		return Ok(number);
	}

	/// Cleans the inputted number string by removing punctuation, "and"s, and whitespace.
	fn preprocess(number_string: &str) -> Result<Vec<u64>, String> {
		let mut numbers = Vec::new();
		for num_str in number_string
			.trim()
			.split(|c: char| c.is_ascii_whitespace() || c.is_ascii_punctuation())
		{
			if num_str == "" || num_str == "and" {
				continue;
			}

			let num = Self::lookup(num_str)?;
			numbers.push(num);
		}

		Ok(numbers)
	}

	pub fn parse_from_args(args: &[String]) -> Result<u64, String> {
		let number_string = args.join(" ");
		Self::parse(&number_string)
	}

	pub fn parse(number_string: &str) -> Result<u64, String> {
		let numbers = Self::preprocess(number_string)?;
		let chunks = numbers.split_inclusive(|x| *x >= 1000);
		let mut total = 0;

		for chunk in chunks {
			let mut chunk_value = 0;
			let mut prev: u64 = 1;
			for &value in chunk {
				if value >= 1000 {
					if chunk_value == 0 {
						chunk_value = 1;
					}
					chunk_value *= value;
				} else if value == 100 {
					chunk_value += prev * 100 - prev;
				} else {
					chunk_value += value;
				}
				prev = value;
			}
			total += chunk_value;
		}

		Ok(total)
	}
}
