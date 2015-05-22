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
		TestExpectation { input: 19, expectation: "nineteen" }
	];

	for test in tests {
		assert_eq!(format(test.input), test.expectation)
	}
}

fn format(num: i32) -> String {
	if num < 0 {
		"negative ".to_string() + &format(-num)
	}
	else if num == 0 {
		"zero".to_string()
	}
	else if num > 0 && num < 10 {
		format_ones(num)
	}
	else if num >= 10 && num < 20 {
		format_tens(num)
	}
	else {
		panic!("Unable to format number: {}", num)
	}
}

fn format_ones(num: i32) -> String {
	match num {
		0 => "",
		1 => "one",
		2 => "two",
		3 => "three",
		4 => "four",
		5 => "five",
		6 => "six",
		7 => "seven",
		8 => "eight",
		9 => "nine",
		_ => panic!("You shouldn't have passed {} to format_ones", num)
	}.to_string()
}

fn format_tens(num: i32) -> String {
	match num {
		10 => "ten".to_string(),
		11 => "eleven".to_string(),
		12 => "twelve".to_string(),
		13 => "thirteen".to_string(),
		15 => "fifteen".to_string(),
		18 => "eighteen".to_string(),
		_ => {
			let ones_place = num % 10;
			format_ones(ones_place).to_string() + "teen"
		}
	}
}

