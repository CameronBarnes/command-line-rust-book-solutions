use clap::Parser;

#[derive(Parser, Debug)]
#[command(version = "0.1.0", about = "Rust echo", long_about = None)]
struct Args {
    /// Text to print
    #[arg(required = true)]
    text: Vec<String>,
    /// Should the newline character be skipped at the end
    #[arg(short = 'n')]
    skip_newline: bool,
}

fn main() {
    let args = Args::parse();
    if args.skip_newline {
        print!("{}", args.text.join(" "));
    } else {
        println!("{}", args.text.join(" "));
    }
}
