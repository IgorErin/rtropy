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
        let mut counts: HashMap<u8, Int> = HashMap::new();
        let mut nall = 0;
        let mut readed = 0;

        loop {
            f.consume(readed);
            let bytes = f.fill_buf().expect("read failed");
            readed = bytes.len();

            if readed == 0 {
                break;
            }

            for ch in bytes {
                *(counts.entry(*ch).or_default()) += 1;
                nall += 1
            }
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
