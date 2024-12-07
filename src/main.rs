use std::{fs, time::Instant};

use itertools::Itertools;
use rayon::prelude::*;

#[derive(Debug, Default, PartialEq, Eq, Clone)]
struct Equation {
	result: u64,
	values: Vec<u64>
}

impl Equation {
	
	pub fn is_valid(&self, opts: Vec<&str>) -> bool {

		self.does_match(&opts)
	}

	fn eval_right_to_left(&self, ops: &Vec<&&str>) -> bool {
		let mut s = self.values[0];
		for i in 0..ops.len() {
			match *ops[i] {
				"+" => s += self.values[i + 1],
				"*" => s *= self.values[i + 1],
				"||" => {
					// let val = format!("{}{}", s, self.values[i + 1]).parse::<u64>().unwrap();
					// s = val;
					let digits = (self.values[i + 1] as f64).log10().floor() as u64 + 1;
					s = (s * 10u64.pow(digits as u32) + self.values[i + 1]) as u64;
				}
				_ => panic!("Invalid operator"),
			}
			if s > self.result {
				return false;
			}
		}
		s == self.result
	}
	
	fn does_match(&self, ops: &Vec<&str>) -> bool {

		for op_vec in std::iter::repeat(ops.iter()).take(self.values.len() - 1).multi_cartesian_product()
		{
			if self.eval_right_to_left(&op_vec) {
				return true;
			}
		}
		false
	}
}

fn main() {

	let input = fs::read_to_string("data.txt").expect("Could not read file");
	let equations: Vec<Equation> = get_equations(&input);

	let now = Instant::now();

	let val: u64 = equations.par_iter().filter(|e| {
		e.is_valid(vec!["+", "*"])
	})
	.map(|e| e.result)
	.sum();

	let elapsed = now.elapsed();


	println!("Part 1: {val}");
	println!("Elapsed Part 1: {:.2?}", elapsed);

	let now = Instant::now();

	let val: u64 = equations.par_iter().filter(|e| {
		e.is_valid(vec!["+", "*", "||"])
	})
	.map(|e| e.result)
	.sum();

	let elapsed = now.elapsed();

	println!("Part 2: {val}");
	println!("Elapsed Part 2: {:.2?}", elapsed);

}

fn get_equations(input: &str) -> Vec<Equation> {

	let list = input
	.lines()
	.map(|l| {
		let ps: Vec<&str> = l.trim().split(':').collect();
		let value = ps[0].parse::<u64>().unwrap();
		let nums: Vec<u64> = ps[1]
			.split_whitespace()
			.map(|s| s.parse::<u64>().unwrap())
			.collect();

		Equation {
			result: value,
			values: nums.clone()
		}
	})
	.collect();

	list
}