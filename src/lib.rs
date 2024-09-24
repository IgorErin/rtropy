use std::collections::HashMap;

pub type Int = i32;
pub type Float = f64;

pub fn count(chars: impl Iterator<Item = char>) -> Vec<(char, Int)> {
    let mut counts = HashMap::new();

    for ch in chars {
        *(counts.entry(ch).or_default()) += 1;
    }

    counts.into_iter().collect()
}

pub fn compute(counts: Vec<Int>) -> Float {
    let mut result: Float = Default::default();
    let nall: Int = counts.iter().sum();

    for count in counts {
        // println!("count: {count}");
        let prob = (count as Float) / (nall as Float);
        // println!("prob: {prob}");

        let log = Float::log2(prob);
        // println!("log: {log}");

        result -= prob * log;

        // println!("result: {result}");
    }

    result
}
