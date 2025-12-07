use std::io;

fn main() {
	loop {
		let mut input = String::new();
		io::stdin().read_line(&mut input).unwrap();

		let parts: Vec<&str> = input.trim().split_whitespace().collect();

		if parts.len() != 3 {
			panic!("Format: a op b");
		}

		let a: f64 = parts[0].parse().unwrap();
		let op = parts[1].chars().next().unwrap();
		let b: f64 = parts[2].parse().unwrap();
		let result = match op {
			'+' => a + b,
			'-' => a - b,
			'*' => a * b,
			'/' => a / b,
			_ => panic!("NO!"),
		};

		println!("= {}", result);
	}
}
