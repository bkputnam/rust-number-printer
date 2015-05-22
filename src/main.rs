struct TestExpectation {
	input: i32,
	expectation: &'static str
}

#[test]
fn it_works() {
	let tests = vec![
		TestExpectation { input: 0, expectation: "zero" },
		TestExpectation { input: 1, expectation: "one" },
		TestExpectation { input: 2, expectation: "two" },
		TestExpectation { input: 3, expectation: "three" },
		TestExpectation { input: 4, expectation: "four" },
		TestExpectation { input: 5, expectation: "five" },
		TestExpectation { input: 6, expectation: "six" },
		TestExpectation { input: 7, expectation: "seven" },
		TestExpectation { input: 8, expectation: "eight" },
		TestExpectation { input: 9, expectation: "nine" }
	];

	for test in tests {
		assert_eq!(format(test.input), test.expectation)
	}
}

fn format<'a>(num: i32) -> &'a str {
	match num {
		0 => "zero",
		_ => format_ones(num)
	}
}

fn format_ones(num: i32) -> &'static str {
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
	}
}