use clap::Parser;
use std::fs::File;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    spath: String,
}

const ROUND: u32 = 4;

fn main() -> std::io::Result<()> {
    let args = Args::parse();
    let f = File::open(args.spath)?;

    let (counts, nall) = rtropy::count(f, Default::default())?;
    let entropy = rtropy::compute(counts, nall);

    println!("entropy: {}", rtropy::fround(entropy, ROUND));

    Ok(())
}
