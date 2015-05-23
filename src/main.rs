
#[cfg(test)]
struct TestExpectation {
	input: i32,
	expectation: &'static str
}

#[test]
fn it_works() {
	let tests = vec![
		TestExpectation { input: -1, expectation: "negative one" },
		TestExpectation { input: 0, expectation: "zero" },
		TestExpectation { input: 1, expectation: "one" },
		TestExpectation { input: 2, expectation: "two" },
		TestExpectation { input: 3, expectation: "three" },
		TestExpectation { input: 4, expectation: "four" },
		TestExpectation { input: 5, expectation: "five" },
		TestExpectation { input: 6, expectation: "six" },
		TestExpectation { input: 7, expectation: "seven" },
		TestExpectation { input: 8, expectation: "eight" },
		TestExpectation { input: 9, expectation: "nine" },

		TestExpectation { input: 10, expectation: "ten" },
		TestExpectation { input: 11, expectation: "eleven" },
		TestExpectation { input: 12, expectation: "twelve" },
		TestExpectation { input: 13, expectation: "thirteen" },
		TestExpectation { input: 14, expectation: "fourteen" },
		TestExpectation { input: 15, expectation: "fifteen" },
		TestExpectation { input: 16, expectation: "sixteen" },
		TestExpectation { input: 17, expectation: "seventeen" },
		TestExpectation { input: 18, expectation: "eighteen" },
		TestExpectation { input: 19, expectation: "nineteen" },

		TestExpectation { input: 20, expectation: "twenty" },
		TestExpectation { input: 21, expectation: "twenty-one" },
		TestExpectation { input: 22, expectation: "twenty-two" },
		TestExpectation { input: 23, expectation: "twenty-three" },
		TestExpectation { input: 24, expectation: "twenty-four" },
		TestExpectation { input: 25, expectation: "twenty-five" },
		TestExpectation { input: 26, expectation: "twenty-six" },
		TestExpectation { input: 27, expectation: "twenty-seven" },
		TestExpectation { input: 28, expectation: "twenty-eight" },
		TestExpectation { input: 29, expectation: "twenty-nine" },
		TestExpectation { input: 30, expectation: "thirty" },
		TestExpectation { input: 39, expectation: "thirty-nine" },
		TestExpectation { input: 49, expectation: "forty-nine" },
		TestExpectation { input: 59, expectation: "fifty-nine" },
		TestExpectation { input: 69, expectation: "sixty-nine" },
		TestExpectation { input: 79, expectation: "seventy-nine" },
		TestExpectation { input: 89, expectation: "eighty-nine" },
		TestExpectation { input: 99, expectation: "ninety-nine" },

		TestExpectation { input: 100, expectation: "one hundred" },
		TestExpectation { input: 234, expectation: "two hundred thirty-four" },
		TestExpectation { input: 456, expectation: "four hundred fifty-six" },
		TestExpectation { input: 999, expectation: "nine hundred ninety-nine" },

		TestExpectation { input: 1000, expectation: "one thousand" },
		TestExpectation { input: 2345, expectation: "two thousand three hundred forty-five" },
		TestExpectation { input: 34567, expectation: "thirty-four thousand five hundred sixty-seven" },
		TestExpectation { input: 456789, expectation: "four hundred fifty-six thousand seven hundred eighty-nine" }

	];

	for test in tests {
		assert_eq!(format(test.input), test.expectation)
	}
}

fn format(num: i32) -> String {
	if num < 0 {
		"negative ".to_string() + &format(-num)
	}
	else if num >= 0 && num < 1000000 {
		format_lt_million(num)
	}
	else {
		panic!("Unable to format number: {}", num)
	}
}

fn format_lt_ten(num: i32) -> String {
	match num {
		0 => "zero",
		1 => "one",
		2 => "two",
		3 => "three",
		4 => "four",
		5 => "five",
		6 => "six",
		7 => "seven",
		8 => "eight",
		9 => "nine",
		_ => panic!("You shouldn't have passed {} to format_lt_ten", num)
	}.to_string()
}

fn format_lt_twenty(num: i32) -> String {
	match num {
		10 => "ten".to_string(),
		11 => "eleven".to_string(),
		12 => "twelve".to_string(),
		13 => "thirteen".to_string(),
		15 => "fifteen".to_string(),
		18 => "eighteen".to_string(),
		_ => {
			if num < 10 {
				format_lt_ten(num)
			}
			else {
				let ones_place = num % 10;
				format_lt_ten(ones_place).to_string() + "teen"
			}
		}
	}
}

fn format_lt_hundred(num: i32) -> String {
	if num < 20 {
		format_lt_twenty(num)
	}
	else {
		let tens_place = num / 10;
		let ones_place = num % 10;
		let tens_str = match tens_place {
			2 => "twenty",
			3 => "thirty",
			4 => "forty",
			5 => "fifty",
			6 => "sixty",
			7 => "seventy",
			8 => "eighty",
			9 => "ninety",
			_ => panic!("You shouldn't have passed {} to format_tens", num)
		};

		if ones_place == 0 {
			tens_str.to_string()
		}
		else {
			tens_str.to_string() + "-" + &format_lt_ten(ones_place)
		}
	}
}

fn format_lt_thousand(num: i32) -> String {
	if num < 100 {
		format_lt_hundred(num)
	}
	else {
		let hundreds_place = num / 100;
		let tens = num % 100;
		let hundreds_str = format_lt_ten(hundreds_place) + " hundred";

		if tens == 0 {
			hundreds_str
		}
		else {
			hundreds_str + " " + &format_lt_hundred(tens)
		}
	}
}

fn format_lt_million(num: i32) -> String {
	if num < 1000 {
		format_lt_thousand(num)
	}
	else {
		let thousands_place = num / 1000;
		let hundreds = num % 1000;
		let thousands_str = format_lt_thousand(thousands_place) + " thousand";

		if hundreds == 0 {
			thousands_str
		}
		else {
			thousands_str + " " + &format_lt_thousand(hundreds)
		}
	}
}