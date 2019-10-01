use std::fs::File;
use std::io::{self, BufReader, BufRead, Error};
use std::time::SystemTime;

use distances;

fn test_levenshteins() -> io::Result<()> {
	let x = BufReader::new(File::open("test.txt")?).lines();
	let x: Vec<Vec<char>> = x.map(|x| x.unwrap().chars().collect::<Vec<char>>()).collect();
	println!("{:?}", SystemTime::now());
	let r = distances::levenshteins(&x, &distances::LevenshteinCost::default());
	println!("{:?}", SystemTime::now());
	Ok(())
}

fn test_levenshtein() -> io::Result<()> {
	let x = BufReader::new(File::open("test.txt")?).lines();
	let x: Vec<Vec<char>> = x.map(|x| x.unwrap().chars().collect::<Vec<char>>()).collect();
	println!("{:?}", SystemTime::now());
	let l = x.len();
	let _c = distances::LevenshteinCost::default();
	for i in 0..l {
		for j in i..l {
			// let s: Vec<char> = x[i].chars().collect();
			// let t: Vec<char> = x[j].chars().collect();
			distances::levenshtein(&x[i], &x[j], &_c);
		}
	}
	println!("{:?}", SystemTime::now());
	Ok(())
}
fn main() {
	test_levenshteins();
	// test_levenshtein();
}