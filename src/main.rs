use std::collections::HashMap;

fn main() {
    let input = vec![1,1,1,2,2,3,4,5];
    let k = 2 as i32;
    let output = Solution::top_k_frequent(input, k);
    dbg!(output);
}

struct Solution;

impl Solution {
    pub fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let mut x = HashMap::new();
	let mut res: Vec<i32> = Vec::new();

	for num in nums {
	    match x.get(&num) {
		// why do these match arms need to be in curly brackets?
		Some(_) => {x.entry(num).and_modify(|counter| *counter += 1);},
		None => {x.entry(num).or_insert(1);},
	    };
	}
	let mut hash_vec: Vec<(&i32, &i32)> = x.iter().collect();
	hash_vec.sort_by(|a, b| b.1.cmp(a.1));
	for i in 0..k {
	    res.push(*hash_vec[i as usize].0);
	}
	res
    }
}
