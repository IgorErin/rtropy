use std::collections::HashMap;
use std::io::{BufRead, BufReader, Read};

pub type Int = i32;
pub type Float = f32;

pub fn fround(f: f32, num: u32) -> f32 {
    let m = 10i32.pow(num) as f32;
    (f * m).round() / m
}

pub fn count(f: impl Read, _: u32) -> std::io::Result<(Vec<Int>, Int)> {
    let mut f = BufReader::new(f);

    let mut counts: HashMap<u8, Int> = HashMap::new();
    let mut nall = 0;
    let mut readed = 0;

    loop {
        f.consume(readed);
        let bytes = f.fill_buf()?;
        readed = bytes.len();

        if readed == 0 {
            break;
        }

        for ch in bytes {
            *(counts.entry(*ch).or_default()) += 1;
            nall += 1
        }
    }

    let counts = counts.values().cloned().collect();
    Ok((counts, nall))
}

pub fn compute(counts: Vec<Int>, nall: Int) -> Float {
    let mut result: Float = 0.0;

    for count in counts.into_iter() {
        let prob = (count as Float) / (nall as Float);
        result -= prob * Float::log2(prob);
    }

    result
}
