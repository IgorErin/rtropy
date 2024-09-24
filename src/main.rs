use clap::Parser;
use std::fs;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    spath: String,
}

fn main() {
    let args = Args::parse();
    let sfile = fs::read_to_string(args.spath);

    let sfile = match sfile {
        Err(err) => {
            eprintln!("Cannot open file: {err}");
            return;
        }
        Ok(file) => file,
    };

    let res = rtropy::count(sfile.chars());
    let res = res.iter().map(|x| x.1);
    let res = rtropy::compute(res.collect());
    let res = rtropy::fround(res, 4);

    println!("entropy: {res}")
}
