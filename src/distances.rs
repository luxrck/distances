use std::collections::HashSet;
use rayon::prelude::*;

pub struct LevenshteinCost {
	pub i: usize, pub d: usize, pub r: usize
}

impl Default for LevenshteinCost {
	fn default() -> Self {
		LevenshteinCost {i:1, d:1, r:1}
	}
}


pub fn levenshtein(s: &str, t: &str, c: &LevenshteinCost) -> usize {
	let (ls, lt) = (s.chars().count(), t.chars().count());
	if ls > lt { return levenshtein(t, s, c); }
	if ls == 0 { return ls; }
	let LevenshteinCost {i, d, r} = c;

	let mut dp: Vec<usize> = (0..lt+1).map(|x| x).collect();

	for (l, sc) in s.chars().enumerate() {
		let mut dpc = dp[0];
		for (c, tc) in t.chars().enumerate() {
			let dpcp1 = dp[c+1];
			if sc == tc {
				dp[c+1] = dpc;
			} else {
				// dp[c+1] = *[dpc + r, dpcp1 + d, dp[c] + i].iter().min().unwrap();
				dp[c+1] = std::cmp::min(std::cmp::min(dpc + r, dpcp1 + d), dp[c] + i);
			}
			dpc = dpcp1;
		}
		dp[0] = l+1;
	}

	*dp.last().unwrap()
}


pub fn levenshteins(inputs: &Vec<&str>, c: &LevenshteinCost) -> Vec<Vec<u8>> {
	let len = inputs.len();
	(0..len).into_par_iter().map(|i| {
		let mut v = vec![0; len];
		(i..len).for_each(|j| {
			v[j] = levenshtein(inputs[i], inputs[j], c) as u8;
		});
		v
	}).collect::<Vec<Vec<u8>>>()
}


pub fn cchars(s: &str, t: &str) -> usize {
	let ss: HashSet<char> = s.chars().collect();
	let st: HashSet<char> = t.chars().collect();
	ss.intersection(&st).count()
}


#[cfg(test)]
mod test {
	#[test]
	fn test_levenshtein() {
		let s = "woefjweoifwjeio";
		let t = "woefjweiofajfoewifj";
		let d = super::levenshtein(s, t, &super::LevenshteinCost::default());
		assert_eq!(d, 8);
	}

	#[test]
	fn test_levenshteins() {
		let x = vec! ["aofioaefj", "oweifjfioej", "ioejwiofjiow"];
		super::levenshteins(&x, &super::LevenshteinCost::default());
		println!("joefie");
	}

	#[test]
	fn test_cchars() {
		let s = "abdet";
		let t = "bectt";
		let c = super::cchars(s, t);
		assert_eq!(c, 3);
	}
}