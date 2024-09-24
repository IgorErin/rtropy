use std::collections::HashMap;

type Int = i32;
type Float = f64;

pub fn count(str: String) -> (Int, Vec<Int>) {    
    let mut count: Int = 0;
    let mut counts = HashMap::new();

    for ch in str.chars() {
        *(counts.entry(ch).or_default()) += 1;
        count += 1;
    }

    (count, counts.values().cloned().collect())
}

pub fn compute<I: Iterator<Item=Int>>(nitems: Int, counts: I) -> Float {
    let mut result = Default::default();

    for c in counts {
        let prob = (c as Float) / (nitems as Float);
        result += prob * Float::log2(prob);
    }

    result
}