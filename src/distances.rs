use ndarray::parallel::prelude::*;
use ndarray::{Array2, Axis};
use std::collections::HashSet;

pub struct LevenshteinCost {
    pub i: usize,
    pub d: usize,
    pub r: usize,
}

impl Default for LevenshteinCost {
    fn default() -> Self {
        LevenshteinCost { i: 1, d: 1, r: 1 }
    }
}

pub fn levenshtein_<T: Eq>(s: &[T], t: &[T], c: &LevenshteinCost, dp: &mut [usize]) -> usize {
    let (ls, lt) = (s.len(), t.len());

    let (mut ps, mut pe) = (0, 0);
    while ps < ls && ps < lt && s[ps] == t[ps] {
        ps += 1;
    }

    while pe < ls && pe < lt && s[ls - pe - 1] == t[lt - pe - 1] {
        pe += 1;
    }

    if ps + pe >= ls {
        return lt - ls;
    }
    if ps + pe >= lt {
        return ls - lt;
    }

    let s = &s[ps..ls - pe];
    let t = &t[ps..lt - pe];

    let ls = s.len();
    let lt = t.len();

    let (s, t, _, lt) = if ls > lt {
        (t, s, lt, ls)
    } else {
        (s, t, ls, lt)
    };

    let LevenshteinCost { i, d, r } = c;

    // let mut dp: Vec<usize> = (0..=lt).collect();
    for i in 0..=lt {
        dp[i] = i;
    }

    for (l, sc) in s.iter().enumerate() {
        // println!("{:?}", dp);
        let mut dpc = dp[0];
        dp[0] = l + 1;
        for (c, tc) in t.iter().enumerate() {
            let (dprp1, dpcp1) = (dp[c], dp[c + 1]);
            // dp[0] = dp[c+1];
            if sc == tc {
                dp[c + 1] = dpc;
            } else {
                // dp[c+1] = *[dpc + r, dpcp1 + d, dp[c] + i].iter().min().unwrap();
                dp[c + 1] = std::cmp::min(std::cmp::min(dpc + r, dpcp1 + d), dprp1 + i);
                // dp[c+1] = std::cmp::min(std::cmp::min(dpc, dpcp1), dp[c]) + 1;
            }
            // dpc = dp[0];
            dpc = dpcp1;
        }
    }
    // println!("{:?}", dp);
    dp[lt]
}

pub fn levenshtein(s: &str, t: &str, c: &LevenshteinCost) -> usize {
    let s: Vec<char> = s.chars().collect();
    let t: Vec<char> = t.chars().collect();
    let mut dp = vec![0usize; std::cmp::max(s.len(), t.len())];
    levenshtein_(&s, &t, c, &mut dp)
}

pub fn levenshteins(inputs: &Vec<Vec<char>>, c: &LevenshteinCost) -> Array2<u8> {
    let len = inputs.len();
    let len_ms = inputs.iter().map(|x| x.len()).max().unwrap() + 1;
    let mut m = Array2::<u8>::zeros((len, len));
    m.axis_iter_mut(Axis(0))
        .into_par_iter()
        .enumerate()
        .for_each(|(i, mut v)| {
            let mut dp = vec![0usize; len_ms];
            (i + 1..len).for_each(|j| {
                // let (mut _p, mut _n) = (0, 0);
                // dp.resize_with(std::cmp::max(inputs[i].len(), inputs[j].len()) + 1, || {_p=_n; _n+=1; _p});
                v[j] = levenshtein_(&inputs[i], &inputs[j], c, &mut dp) as u8;
            });
        });
    m
}

pub fn cchars(s: &str, t: &str) -> usize {
    let ss: HashSet<char> = s.chars().collect();
    let st: HashSet<char> = t.chars().collect();
    ss.intersection(&st).count()
}

#[cfg(test)]
mod test {
    use ndarray::array;

    #[test]
    fn test_levenshtein() {
        let t = "woefjweoifwjeio";
        let s = "woefjweiofajfoewifj";
        let d = super::levenshtein(&s, &t, &super::LevenshteinCost::default());
        assert_eq!(d, 8);
    }

    #[test]
    fn test_levenshteins() {
        let x = vec!["aofioaefj", "oweifjfioej", "ioejwiofjiow"];
        let x: Vec<Vec<char>> = x.iter().map(|x| x.chars().collect::<Vec<char>>()).collect();
        let r = super::levenshteins(&x, &super::LevenshteinCost::default());
        assert_eq!(r, array![[0, 8, 9], [0, 0, 8], [0, 0, 0]]);
    }

    #[test]
    fn test_cchars() {
        let s = "abdet";
        let t = "bectt";
        let c = super::cchars(s, t);
        assert_eq!(c, 3);
    }
}
