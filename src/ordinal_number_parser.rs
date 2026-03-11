use std::str::FromStr;

pub struct OrdinalNumberParser;

impl OrdinalNumberParser {
	pub fn parse_from_args<T>(args: &[String]) -> Result<T, String>
	where
		T: FromStr,
	{
		Self::parse(&args[0])
	}

	pub fn parse<T>(ordinal_number_string: &str) -> Result<T, String>
	where
		T: FromStr,
	{
		// Remove commas for dates
		let ordinal_number_string = match ordinal_number_string.strip_suffix(",") {
			Some(ord_num_str) => ord_num_str,
			_ => ordinal_number_string,
		};

		let mut number_string = String::new();
		let suffixes = ["th", "st", "nd", "rd"];
		for suffix in suffixes {
			match ordinal_number_string.strip_suffix(suffix) {
				Some(num_string) => {
					number_string = num_string.to_string();
					break;
				}
				_ => (),
			}
		}

		number_string.parse::<T>().or(Err(format!(
			"{ordinal_number_string} is not a valid ordinal number."
		)))
	}
}
