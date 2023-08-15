use clap::Parser;

#[derive(Parser, Debug)]
#[command(version)]
struct Cli {
    #[arg(long, short, default_value = "")]
    message: String,
}


fn main() {
    let args = Cli::parse();
    dbg!(&args);
}
