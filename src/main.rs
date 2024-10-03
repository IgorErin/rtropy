use clap::Parser;
use std::collections::HashMap;
use std::{
    fs::File,
    io::{BufRead, BufReader},
};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    spath: String,
}

const ROUND: u32 = 4;

fn main() {
    let args = Args::parse();
    let mut f = BufReader::new(File::open(args.spath).expect("open failed"));

    type Int = i32;
    let (counts, nall) = {
        let mut counts: HashMap<char, Int> = HashMap::new();
        let mut nall = 0;

        let mut buf: Vec<u8> = Vec::<u8>::new();
        while f.read_until(b'\n', &mut buf).expect("read_until failed") != 0 {
            let s = String::from_utf8(buf).expect("from_utf8 failed");

            for ch in s.chars() {
                *(counts.entry(ch).or_default()) += 1;
                nall += 1;
            }

            buf = s.into_bytes();
            buf.clear();
        }
        (counts, nall)
    };

    type Float = f32;
    let entropy = {
        let mut result: Float = Default::default();
        for (_, count) in counts.into_iter() {
            let prob = (count as Float) / (nall as Float);
            result -= prob * Float::log2(prob);
        }
        result
    };

    println!("entropy: {}", rtropy::fround(entropy, ROUND));
}
