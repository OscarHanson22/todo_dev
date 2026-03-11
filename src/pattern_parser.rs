use crate::UsableArgs;

use chrono::{Days, Months, NaiveDate, NaiveDateTime, NaiveTime, TimeDelta};

#[derive(Debug, PartialEq)]
pub enum DateChange {
	Absolute(NaiveDate),
	Relative(
		i32, /* years */
		i32, /* months */
		i64, /* days */
	),
}

impl DateChange {
	pub fn change(&self, context: NaiveDateTime) -> NaiveDateTime {
		match &self {
			Self::Absolute(date) => NaiveDateTime::new(*date, context.time()),
			Self::Relative(years, months, days) => {
				let years_duration = Months::new(12 * years.unsigned_abs());
				let months_duration = Months::new(months.unsigned_abs());
				let days_duration = Days::new(days.unsigned_abs());

				let context = if *years < 0 {
					context.checked_sub_months(years_duration)
				} else {
					context.checked_add_months(years_duration)
				};
				let context = context.expect("Amount of years specified is not valid or out of range.");

				let context = if *months < 0 {
					context.checked_sub_months(months_duration)
				} else {
					context.checked_add_months(months_duration)
				};
				let context = context.expect("Amount of months specified is not valid or out of range.");

				let context = if *days < 0 {
					context.checked_sub_days(days_duration)
				} else {
					context.checked_add_days(days_duration)
				};
				let context = context.expect("Amount of days specified is not valid or out of range.");

				context
			}
		}
	}
}

#[derive(Debug, PartialEq)]
pub enum TimeChange {
	Absolute(NaiveTime),
	Relative(i64 /* hours */, i64 /* minutes */),
}

impl TimeChange {
	pub fn change(&self, context: NaiveDateTime) -> NaiveDateTime {
		match &self {
			Self::Absolute(time) => NaiveDateTime::new(context.date(), *time),
			Self::Relative(hours, minutes) => context
				.checked_add_signed(TimeDelta::hours(*hours))
				.expect("Amount of hours specified is out of range.")
				.checked_add_signed(TimeDelta::minutes(*minutes))
				.expect("Amount of minutes specified is out of range."),
		}
	}
}

pub struct ParserDescription(String);

impl ParserDescription {
	pub fn new(description: &str) -> Self {
		Self(description.to_string())
	}

	pub fn chain(self, other: Self) -> Self {
		let mut chained_string = self.0;
		chained_st
		Self(

	pub fn or(self, ) -> Self {
		Self(self.0.extend_from_slice
}

pub struct PatternParserError {
	found: String,
	position: usize,
	pattern: Vec<String>,
}

impl PatternParserError {
	pub fn new(found: &str, position: usize) -> Self 

// pub struct PatternParserError {
// 	help_msg: 
// }

type ParserResult<T> = std::result::Result<T, PatternParserError>;

// #[derive(Clone, Debug)]
// pub struct PatternParser<F, T>
// where
// 	F: Fn(&[String]) -> Result<T, String>,
// {
// 	length: usize,
// 	pattern_match: F,
// }

// impl<F, T> PatternParser<F, T>
// where
// 	F: Fn(&[String]) -> Result<T, String>,
// {

pub struct DynamicPatternParser<F, T> 
where
	F: Fn(&[String]) -> ParserResult<T>, 

#[derive(Clone, Debug)]
pub struct PatternParser<F, T>
where
	F: Fn(&[String]) -> ParserResult<T>,
{
	length: usize,
	parser: F,
}

impl<F, T> PatternParser<F, T>
where
	F: Fn(&[String]) -> Result<T, String>,
{
	pub fn new(length: usize, parser: F) -> Self {
		Self { length, parser }
	}

	pub fn length(&self) -> usize {
		self.length
	}

	pub fn parse(&self, args: &[String]) -> Result<T, String> {
		if args.len() != self.length() {
			println!("failing args: {:?}", args);
			panic!(
				"length mismatch args length: {}, parser length: {}",
				args.len(),
				self.length()
			);
		}
		(self.parser)(args)
	}

	pub fn chain<F1, T1>(
		self,
		other: PatternParser<F1, T1>,
	) -> PatternParser<impl Fn(&[String]) -> Result<(T, T1), String>, (T, T1)>
	where
		F1: Fn(&[String]) -> Result<T1, String>,
	{
		let total_parser_length = self.length() + other.length();
		PatternParser::new(
			total_parser_length,
			move |args: &[String]| -> Result<(T, T1), String> {
				Ok((
					self.parse(&args[0..self.length()])?,
					other.parse(&args[self.length()..total_parser_length])?,
				))
			},
		)
	}

	pub fn or<F1>(
		self,
		other: PatternParser<F1, T>,
	) -> PatternParser<impl Fn(&[String]) -> Result<T, String>, T>
	where
		F1: Fn(&[String]) -> Result<T, String>,
	{
		if self.length() != other.length() {
			panic!("Length of parser items must be the same.");
		}

		PatternParser::new(self.length(), move |args: &[String]| -> Result<T, String> {
			if let Ok(result) = self.parse(args) {
				return Ok(result);
			}
			if let Ok(result) = other.parse(args) {
				return Ok(result);
			}
			Err("Arguments could not be parsed.".to_string())
		},
	}

	pub fn parse_from_usable_args(&self, args: &UsableArgs) -> Result<(T, usize), String> {
		for (position, args_window) in args.positions_and_windows(self.length) {
			match self.parse(args_window) {
				Ok(target) => return Ok((target, position)),
				_ => (),
			}
		}

		Err("Could not find a match.".to_string())
	}
}

type BoxedPatternParser<T> = PatternParser<Box<dyn Fn(&[String]) -> Result<T, String>>, T>;

impl<F, T> PatternParser<F, T>
where
	F: Fn(&[String]) -> Result<T, String> + 'static,
{
	pub fn into_boxed(self) -> BoxedPatternParser<T> {
		PatternParser {
			length: self.length(),
			pattern_match: Box::new(self.pattern_match),
		}
	}
}
// 	let item = PatternParserItem::new(
// 		1, 
// 		"a number"
// 		move |args: &[String]| -> PatternParserItemResult<T> { ... }, 
// 	);

// 	let other_item = PatternParserItem::new(
// 		1, 
// 		"an ordinal number"
// 		move |args: &[String]| -> PatternParserItemResult<T> { ... }, 
// 	);

// 	item.parse(args)?; // fail -> Expected a number at position 0, found "two".
// 							   // From pattern [a number].

// 							   // From pattern ["at", a number between 0 and 12 or an ordinal number between 0 and 12, "pm" or "am"]
// 	item.or(other_item)?; // fail -> Expected a number or an ordinal number at position 0, found "two"
// 	item.chain(other_item)?; // fail at position 0 -> Expected a number at position 0, found "two".
// 							 // From pattern [a number, an ordinal number]
// 							 // fail at position 1 -> Expected an ordinal number at position 1, found "2".
// 							 // From pattern [a number, an ordinal number]

// 	full

// 	let parser = PatternParser::new(
// 		3,
// 		"a time",
// 		move |args: &[String]| -> PatternParserItemResult<T> { ... },
// 	);

// 	parser.parser(args)?; // fail -> Expected a colon time from pattern: 
// 								  // [number
}

// pub struct SizedSlice<'a, T, const N: usize>(&'a [T]);

// impl<'a, T, const N: usize> SizedSlice<'a, T, N> {
// 	pub fn new(slice: &'a [T]) -> Self {
// 		if slice.len() != N {
// 			panic!("Length of slice must be {N}, found: {}.", slice.len());
// 		}

// 		Self(slice)
// 	}

// 	pub fn len(&self) -> usize {
// 		N
// 	}

// 	pub fn as_slice(&'a self) -> &'a [T] {
// 		self.0
// 	}

// 	pub fn to_slice(self) -> &'a [T] {
// 		self.0
// 	}
// }

pub fn numerical_parser_item<F, T>(
	length: usize,
	parser: F,
) -> PatternParser<impl Fn(&[String]) -> Result<T, String>, T>
where
	F: Fn(&[String]) -> Result<T, String>,
{
	PatternParser::new(length, parser)
}

pub fn ranged_numerical_parser_item<F, T>(
	length: usize,
	min: T,
	max: T,
	parser: F,
) -> PatternParser<impl Fn(&[String]) -> Result<T, String>, T>
where
	T: PartialOrd,
	F: Fn(&[String]) -> Result<T, String>,
{
	if min > max {
		panic!("Minimum cannot be less than maximum.");
	}

	PatternParser::new(length, move |args: &[String]| -> Result<T, String> {
		let result = parser(args)?;
		if result < min || result > max {
			Err("Result is out of range.".to_string())
		} else {
			Ok(result)
		}
	})
}

use std::str::FromStr;

pub fn parse_number<T>(args: &[String]) -> Result<T, String>
where
	T: FromStr,
{
	args[0]
		.parse::<T>()
		.or(Err("Not a valid number string.".to_string()))
}

/// Returns a PatternParser that parses a single arg into a number of type T.
/// Usage:
/// let number_parser_1 = number_parser::<u32>();
/// let result = number_parser_1.parse(&["1".to_string()]);
/// assert_eq!(result, Ok(1));
///
pub fn number_parser_item<T>() -> PatternParser<impl Fn(&[String]) -> Result<T, String>, T>
where
	T: FromStr,
{
	numerical_parser_item(1, parse_number)
}

/// Returns a PatternParser that parses a single arg into a number of type T within the bounds of min..=max (both inclusive).
/// Usage:
/// let ranged_number_parser_1 = ranged_number_parser::<u32>(1, 30);
/// let result_in_range = ranged_number_parser_item_1.parse(&["1".to_string()]);
/// assert_eq!(result_in_range, Ok(1));
/// let result_out_of_range = ranged_number_parser_item_1.parse(&["31".to_string()]);
/// assert!(result_out_of_range.is_err())
///
pub fn ranged_number_parser_item<T>(
	min: T,
	max: T,
) -> PatternParser<impl Fn(&[String]) -> Result<T, String>, T>
where
	T: FromStr + PartialOrd,
{
	ranged_numerical_parser_item(1, min, max, parse_number)
}

use crate::number_word_parser::NumberWordParser;

pub fn number_word_parser_item(
	length: usize,
) -> PatternParser<impl Fn(&[String]) -> Result<u64, String>, u64> {
	numerical_parser_item(length, NumberWordParser::parse_from_args)
}

pub fn ranged_number_word_parser_item(
	length: usize,
	min: u64,
	max: u64,
) -> PatternParser<impl Fn(&[String]) -> Result<u64, String>, u64> {
	ranged_numerical_parser_item(length, min, max, NumberWordParser::parse_from_args)
}

use crate::ordinal_number_parser::OrdinalNumberParser;

pub fn ordinal_number_parser_item<T>(
) -> PatternParser<impl Fn(&[String]) -> Result<T, String>, T>
where
	T: FromStr,
{
	numerical_parser_item(1, OrdinalNumberParser::parse_from_args)
}

pub fn ranged_ordinal_number_parser_item<T>(
	min: T,
	max: T,
) -> PatternParser<impl Fn(&[String]) -> Result<T, String>, T>
where
	T: FromStr + PartialOrd,
{
	ranged_numerical_parser_item(1, min, max, OrdinalNumberParser::parse_from_args)
}

use crate::ordinal_number_word_parser::OrdinalNumberWordParser;

pub fn ordinal_number_word_parser_item(
	length: usize,
) -> PatternParser<impl Fn(&[String]) -> Result<u64, String>, u64> {
	numerical_parser_item(length, OrdinalNumberWordParser::parse_from_args)
}

pub fn ranged_ordinal_number_word_parser_item(
	length: usize,
	min: u64,
	max: u64,
) -> PatternParser<impl Fn(&[String]) -> Result<u64, String>, u64> {
	ranged_numerical_parser_item(length, min, max, OrdinalNumberWordParser::parse_from_args)
}

pub fn split_args(
	expected_length: usize,
	args_and_splitters: &[(&str, Option<&str>)],
) -> Result<Vec<String>, String> {
	let mut split_args = Vec::new();
	for (arg, splitter) in args_and_splitters {
		if splitter.is_none() {
			split_args.push(arg.to_string());
			continue;
		}
		let splitter = splitter.unwrap();
		for split_arg in arg.split(splitter) {
			split_args.push(split_arg.to_string());
		}
	}
	if split_args.len() != expected_length {
		return Err("Arguments were not split as expected.".to_string());
	}
	Ok(split_args)
}

// pub fn am_pm_offset_parser_item(
// ) -> PatternParser<impl Fn(&[String]) -> Result<u32, String>, u32> {

/// [n in (1..=12):n in (0..=59), "am" | "pm"]
/// ["5:15", "pm"] only
pub fn colon_time_parser_item(
) -> PatternParser<impl Fn(&[String]) -> Result<NaiveTime, String>, NaiveTime> {
	let hour_parser = ranged_number_parser_item(1, 12);
	let minute_parser = ranged_number_parser_item(0, 59);
	let am_pm_offset_parser = single_arg_map_parser_item(vec![("am", 0), ("pm", 12)]);
	let time_parser = hour_parser.chain(minute_parser).chain(am_pm_offset_parser);

	PatternParser::new(2, move |args: &[String]| -> Result<NaiveTime, String> {
		let args = split_args(time_parser.length(), &[(&args[0], Some(":")), (&args[1], None)])?;
		let ((hour, minute), am_pm_offset) = time_parser.parse(&args)?;
		let time = NaiveTime::from_hms_opt(hour + am_pm_offset, minute, 00)
			.expect("Should be a valid time.");
		Ok(time)
	})
}

/// [n | nw1 in (1..=12), n | nw1 in (0..=59), "am" | "pm"]
/// ["5", "15", "pm"] or
/// ["five", "fifteen", "pm"] or
/// ["5", "fifteen", pm"] or
/// ["five", "15", "pm"]
pub fn split_time_parser_item(
) -> PatternParser<impl Fn(&[String]) -> Result<NaiveTime, String>, NaiveTime> {
	let hour_parser = ranged_number_parser_item(1, 12)
		.or(ranged_number_word_parser_item(1, 1, 12));
	let minute_parser = ranged_number_parser_item(0, 59)
		.or(ranged_number_word_parser_item(1, 0, 59));
	let am_pm_offset_parser = single_arg_map_parser_item(vec![("am", 0), ("pm", 12)]);
	let time_parser = hour_parser.chain(minute_parser).chain(am_pm_offset_parser);

	PatternParser::new(time_parser.length(), move |args: &[String]| -> Result<NaiveTime, String> {
		let ((hour, minute), am_pm_offset) = time_parser.parse(&args)?;
		let hour: u32 = hour.try_into().expect("Should never fail.");
		let minute: u32 = minute.try_into().expect("Should never fail."); 
		let time = NaiveTime::from_hms_opt(hour + am_pm_offset, minute, 00)
			.expect("Should be a valid time.");
		Ok(time)
	})
}

/// [n | nw1 in (1..=12), nw2 in (0..=59), "am" | "pm"]
/// ["five", "fifty", "five", "pm"] or 
/// ["5", "fifty", "five", "pm"]
pub fn split_double_minute_word_time_parser_item(
) -> PatternParser<impl Fn(&[String]) -> Result<NaiveTime, String>, NaiveTime> {
	let hour_parser = ranged_number_parser_item(1, 12)
		.or(ranged_number_word_parser_item(1, 1, 12));
	let minute_parser = ranged_number_word_parser_item(2, 0, 59);
	let am_pm_offset_parser = single_arg_map_parser_item(vec![("am", 0), ("pm", 12)]);
	let time_parser = hour_parser.chain(minute_parser).chain(am_pm_offset_parser);

	PatternParser::new(time_parser.length(), move |args: &[String]| -> Result<NaiveTime, String> {
		let ((hour, minute), am_pm_offset) = time_parser.parse(&args)?;
		let hour: u32 = hour.try_into().expect("Should never fail.");
		let minute: u32 = minute.try_into().expect("Should never fail."); 
		let time = NaiveTime::from_hms_opt(hour + am_pm_offset, minute, 00)
			.expect("Should be a valid time.");
		Ok(time)
	})
}

pub fn custom_at_time_pattern_parser<F>(
	time_parser: PatternParser<F, NaiveTime>,
) -> PatternParser<impl Fn(&[String]) -> Result<TimeChange, String>, TimeChange>
where
	F: Fn(&[String]) -> Result<NaiveTime, String> + 'static,
{
	let at_parser = arg_match_parser_item(&["at"]);
	let at_time_parser = at_parser.chain(time_parser);
	PatternParser::new(
		at_time_parser.length(),
		move |args: &[String]| -> Result<TimeChange, String> {
			let ((), time) = at_time_parser.parse(&args)?;
			Ok(TimeChange::Absolute(time))
		},
	)
}

#[derive(Copy, Clone)]
enum TimeDuration {
	Hours, 
	Minutes, 
}

fn time_duration_parser_item(
) -> PatternParser<impl Fn(&[String]) -> Result<TimeDuration, String>, TimeDuration> {
	PatternParser::new(1, |args: &[String]| -> Result<TimeDuration, String> {
		match &args[0].to_lowercase()[..] {
			"h" | "hr" | "hrs" | "hour" | "hours" => Ok(TimeDuration::Hours),
			"m" | "min" | "mins" | "minute" | "minutes" => Ok(TimeDuration::Minutes),
			_ => Err("Not a valid time duration.".to_string()),
		}
	})
}


/// ["in", n | nw, "minutes" | "hours"]
pub fn in_time_pattern_parser(
) -> PatternParser<impl Fn(&[String]) -> Result<TimeChange, String>, TimeChange> {
	let in_parser = arg_match_parser_item(&["in"]);
	let amount_parser = number_parser_item()
		.or(number_word_parser_item(1));
	let duration_parser = time_duration_parser_item();
	let in_time_parser = in_parser.chain(amount_parser).chain(duration_parser);
	PatternParser::new(
		in_time_parser.length(),
		move |args: &[String]| -> Result<TimeChange, String> {
			let (((), amount), duration) = in_time_parser.parse(&args)?;
			let amount = amount.try_into().expect("Amount is out of range.");
			let (hours, minutes) = match duration {
				TimeDuration::Hours => (amount, 0),
				TimeDuration::Minutes => (0, amount),
			};
			Ok(TimeChange::Relative(hours, minutes))
		},
	)
}

pub fn at_split_time_pattern_parser(
) -> PatternParser<impl Fn(&[String]) -> Result<TimeChange, String>, TimeChange> {
	custom_at_time_pattern_parser(split_time_parser_item())
}

pub fn at_colon_time_pattern_parser(
) -> PatternParser<impl Fn(&[String]) -> Result<TimeChange, String>, TimeChange> {
	custom_at_time_pattern_parser(colon_time_parser_item())
}

pub fn at_split_double_minute_word_time_pattern_parser(
) -> PatternParser<impl Fn(&[String]) -> Result<TimeChange, String>, TimeChange> {
	custom_at_time_pattern_parser(split_double_minute_word_time_parser_item())
}
// parser_list = (F1, F2, F3, F4, F5, F6, F7, F8, F9, ...)

// pub struct PatternParserList<T>(Vec<BoxedPatternParser<T>>);

// impl<T> PatternParserList<T> {
// 	pub fn new() -> Self {
// 		Self(Vec::new())
// 	}

// 	pub fn add<F>(&mut self, pattern_parser: PatternParser<F, T>)
// 	where
// 		F: Fn(&[String]) -> Result<T, String> + 'static,
// 	{
// 		let boxed_pattern_parser: BoxedPatternParser<T> = PatternParser::new(
// 			pattern_parser.length(),
// 			Box::new(pattern_parser.pattern_match),
// 		);
// 		self.0.push(boxed_pattern_parser);
// 	}

// 	pub fn parse(&self, args: &mut UsableArgs) -> Result<T, String> {
// 		for parser in &self.0 {
// 			if let Ok((result, position)) = parser.parse(&args) {
// 				args.use_args(position, parser.length());
// 				return Ok(result);
// 			}
// 		}
// 		Err("No parser could parse the input.".to_string())
// 	}
// }

pub fn argify(args: &[&str]) -> Vec<String> {
	args.into_iter().map(|arg| arg.to_string()).collect()
}

pub fn parse_with_parsers<T>(
	args: &mut UsableArgs,
	parsers: Vec<BoxedPatternParser<T>>,
) -> Result<T, String> {
	for parser in &parsers {
		if let Ok((result, position)) = parser.parse(&args) {
			args.use_args(position, parser.length());
			return Ok(result);
		}
	}
	Err("No parser could parse the input.".to_string())
}

pub fn parse_time(
	args: &mut UsableArgs,
	parsers: Vec<BoxedPatternParser<TimeChange>>,
) -> Result<TimeChange, String> {
	parse_with_parsers(args, parsers)
}

pub fn parse_date(
	args: &mut UsableArgs,
	parsers: Vec<BoxedPatternParser<DateChange>>,
) -> Result<DateChange, String> {
	parse_with_parsers(args, parsers)
}

pub fn arg_match_parser_item(
	match_on_args: &[&str],
) -> PatternParser<impl Fn(&[String]) -> Result<(), String>, ()> {
	let match_on_args: Vec<String> = match_on_args
		.to_vec()
		.into_iter()
		.map(|arg| arg.to_string())
		.collect();

	PatternParser::new(
		match_on_args.len(),
		move |args: &[String]| -> Result<(), String> {
			if args == &match_on_args[..] {
				Ok(())
			} else {
				Err("Args did not match.".to_string())
			}
		},
	)
}

pub fn single_arg_map_parser_item<T>(
	arg_map: Vec<(&str, T)>,
) -> PatternParser<impl Fn(&[String]) -> Result<T, String>, T>
where
	T: Copy,
{
	let arg_map: Vec<(String, T)> = arg_map
		.into_iter()
		.map(|(arg, map_to_value)| (arg.to_string(), map_to_value))
		.collect();

	PatternParser::new(1, move |args: &[String]| -> Result<T, String> {
		for (arg, map_to_value) in &arg_map {
			if args[0].to_lowercase() == arg.to_lowercase() {
				return Ok(*map_to_value);
			}
		}
		Err("No value found in map.".to_string())
	})
}


#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_1() {
		let number_parser_1 = number_parser_item::<u32>();
		let number_parser_2 = number_parser_item::<u64>();
		let number_parser_3 = number_parser_item::<i32>();

		let auto_parser = number_parser_1.chain(number_parser_2).chain(number_parser_3);

		let args = ["1".to_string(), "2".to_string(), "3".to_string()];
		let result = auto_parser.parse(&args);
		assert_eq!(result, Ok(((1_u32, 2_u64), 3_i32)),);
	}

	#[test]
	fn test_2() {
		let ranged_number_parser_1 = ranged_number_parser_item::<u32>(10, 20);
		let number_parser_2 = number_parser_item::<u64>();
		let number_parser_3 = number_parser_item::<i32>();

		let auto_parser =
			ranged_number_parser_1.chain(number_parser_2).chain(number_parser_3);

		let args = ["1".to_string(), "2".to_string(), "3".to_string()];
		let result = auto_parser.parse(&args);
		println!("{:?}", result);
		assert!(result.is_err());
	}

	#[test]
	fn test_3() {
		let number_parser_1 = number_parser_item::<u64>();
		let number_parser_2 = number_word_parser_item(1);
		let number_parser_3 = ordinal_number_parser_item::<u64>();
		let combo = number_parser_1
			.or(number_parser_2)
			.or(number_parser_3);

		let args = ["1".to_string()];
		let result = combo.parse(&args);
		assert_eq!(result, Ok(1));

		let args = ["one".to_string()];
		let result = combo.parse(&args);
		assert_eq!(result, Ok(1));

		let args = ["1st".to_string()];
		let result = combo.parse(&args);
		assert_eq!(result, Ok(1));
	}

	#[test]
	fn test_4() {
		let on = arg_match_parser_item(&["on"]);
		let by_the_way = arg_match_parser_item(&["by", "the", "way"]);

		let args = ["on".to_string()];
		let result = on.parse(&args);
		println!("on results: {:?}", result);
		assert_eq!(result, Ok(()));

		// /// SHOULD NEVER OCCUR but still works lol
		// let args = ["on".to_string(), "something".to_string()];
		// let result = on.parse(&args);
		// // println!("on results: {:?}", result);
		// assert!(result.is_err());

		let args = ["one".to_string()];
		let result = on.parse(&args);
		println!("on results: {:?}", result);
		assert!(result.is_err());

		let args = ["by".to_string(), "the".to_string(), "way".to_string()];
		let result = by_the_way.parse(&args);
		println!("By the way results: {:?}", result);
		assert_eq!(result, Ok(()));

		let args = ["by".to_string(), "the".to_string(), "wayth".to_string()];
		let result = by_the_way.parse(&args);
		println!("By the way results: {:?}", result);
		assert!(result.is_err());
	}

	#[test]
	fn test_5() {
		let map_parser_item =
			single_arg_map_parser_item(vec![("on", 12), ("some", 18), ("thing", 24)]);

		let args = ["on".to_string()];
		let result = map_parser_item.parse(&args);
		assert_eq!(result, Ok(12));
		let args = ["some".to_string()];
		let result = map_parser_item.parse(&args);
		assert_eq!(result, Ok(18));
		let args = ["thing".to_string()];
		let result = map_parser_item.parse(&args);
		assert_eq!(result, Ok(24));
		let args = ["nope".to_string()];
		let result = map_parser_item.parse(&args);
		assert!(result.is_err());
	}

	#[test]
	fn test_6() {
		let time_parser_item = colon_time_parser_item();

		let args = ["6:15".to_string(), "pm".to_string()];
		let result = time_parser_item.parse(&args);
		let expected = NaiveTime::from_hms_opt(18, 15, 00).unwrap();
		assert_eq!(result, Ok(expected));
		let args = ["6:15".to_string(), "am".to_string()];
		let result = time_parser_item.parse(&args);
		let expected = NaiveTime::from_hms_opt(6, 15, 00).unwrap();
		assert_eq!(result, Ok(expected));
		let args = ["6:60".to_string(), "am".to_string()];
		let result = time_parser_item.parse(&args);
		assert!(result.is_err());
	}

	#[test]
	fn test_7() {
		let args = ["6:15".to_string(), "pm".to_string()];
		let args = split_args(3, &[(&args[0], Some(":")), (&args[1], None)]).unwrap();
		let expected = ["6".to_string(), "15".to_string(), "pm".to_string()];
		println!("a: {:?}, e: {:?}", args, expected);
		assert_eq!(&args[..], expected);

		let args = ["10/26/2025".to_string()];
		let args = split_args(3, &[(&args[0], Some("/"))]).unwrap();
		let expected = ["10".to_string(), "26".to_string(), "2025".to_string()];
		println!("a: {:?}, e: {:?}", args, expected);
		assert_eq!(&args[..], expected);
	}

	#[test]
	fn test_9() {
		let parser_1 = number_parser_item::<u32>();
		let parser_2 = number_word_parser_item(1);
		let auto_parser_item_2 = parser_1.chain(parser_2);

		let args = ["1".to_string(), "two".to_string()];
		let result = auto_parser_item_2.parse(&args);
		assert_eq!(result, Ok((1, 2)));

		let parser_1 = number_parser_item::<u32>();
		let parser_2 = number_word_parser_item(1);
		let parser_3 = number_parser_item::<i32>();
		let auto_parser_item_3 = parser_1.chain(parser_2).chain(parser_3);

		let args = ["1".to_string(), "two".to_string(), "-3".to_string()];
		let result = auto_parser_item_3.parse(&args);
		assert_eq!(result, Ok(((1, 2), -3)));
	}

	#[test]
	fn test_10() {
		let parser_1 = number_parser_item::<u64>();
		let parser_2 = number_word_parser_item(1);
		let parser_3 = number_parser_item::<i32>();
		let auto_parser_item_2 = parser_1.or(parser_2).chain(parser_3);

		let args = ["1".to_string(), "-3".to_string()];
		let result = auto_parser_item_2.parse(&args);
		assert_eq!(result, Ok((1, -3)));

		let args = ["one".to_string(), "-3".to_string()];
		let result = auto_parser_item_2.parse(&args);
		assert_eq!(result, Ok((1, -3)));
	}

	#[test]
	fn test_11() {
		// let p1 = at_split_time_pattern_parser();
		// let p2 = at_colon_time_pattern_parser();
		// let parsers = vec![p1, p2];
		// let args = UsableArgs::from(&["at".to_string(), "6:15".to_string(), "pm".to_string()]);
		// parse_with_parsers(
	}

	#[test]
	fn test_12() {
		// let mut pattern_parser_list = PatternParserList::new();
		// pattern_parser_list.add(at_colon_time_pattern_parser()); // issue with the colon parser when it is first and the input is a SPLIT time
		// pattern_parser_list.add(at_split_time_pattern_parser());

		let mut args = UsableArgs::from(&argify(&["something", "at", "at", "six", "fifty", "five", "pm", "end"]));
		let parsers = vec![
			at_colon_time_pattern_parser().into_boxed(), 
			at_split_time_pattern_parser().into_boxed(),
			at_split_double_minute_word_time_pattern_parser().into_boxed()
		];
		let result = parse_with_parsers(&mut args, parsers);
		println!("12 result: {:?}", result);
		println!("args: {:?}", args);
		assert_eq!(
			result, 
			Ok(TimeChange::Absolute(NaiveTime::from_hms_opt(6 + 12, 55, 00).unwrap()))
		)
	}

	#[test]
	fn test_13() {
		let parser = at_colon_time_pattern_parser();
		let mut args = UsableArgs::from(&argify(&["at", "6", "15", "pm", "end"]));
		let result = parser.parse(&mut args);
		println!("13 result: {:?}", result);
	}

	// #[test]
	// fn test_8() {
	// 	// let slice = ["something", "lol"];
	// 	let vec = vec![1, 2, 3, 4, 5];
	// 	let sized_slice = SizedSlice(&vec[0..3]);
	// 	println!("length: {}", sized_slice.length());
	// }
}
