use clap::Parser;
use hashexample::write_new_template;

/// Construct hash zk-SNARK example
#[derive(Parser, Debug)]
#[clap(about, version, author)]
struct Args {
    /// Path to witness file
    #[clap(short, default_value = "witness")]
    witness_file_path: String,

    /// Output .zok file path
    #[clap(short, long, default_value = "hashexample.zok")]
    output_file_path: String,
}

fn main() {
    let args = Args::parse();
    write_new_template(&args.witness_file_path, &args.output_file_path);

    println!("Creating new .zok file!");
}
