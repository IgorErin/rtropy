use clap::Parser;
use std::fs;
use std::collections::HashMap;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    spath: String
}

fn main() {
    let args = Args::parse();
    let sfile = fs::read_to_string(args.spath);

    let sfile = match sfile {
        Err(err) => { 
            eprintln!("Cannot open file: {err}");
            return
        },
        Ok(file) => file
    };

    let res = count(sfile);
    let res = compute(res.0, res.1.iter().cloned());

    println!("result: {res}")
}

type Int = i32;
type Float = f64;

fn count(str: String) -> (Int, Vec<Int>) {    
    let mut count: Int = 0;
    let mut counts = HashMap::new();

    for ch in str.chars() {
        *(counts.entry(ch).or_default()) += 1;
        count += 1;
    }

    (count, counts.values().cloned().collect())
}

fn compute<I: Iterator<Item=Int>>(nitems: Int, counts: I) -> Float {
    let mut result = Default::default();

    for c in counts {
        let prob = (c as Float) / (nitems as Float);
        result += prob * Float::log2(prob);
    }

    result
}
