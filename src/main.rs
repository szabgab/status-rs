use std::process::exit;
use std::fs::File;
use std::io::Write;

use clap::Parser;
use chrono::{Utc, DateTime};
use chrono::serde::ts_seconds;
use serde::{Serialize};

#[derive(Parser, Debug)]
#[command(version)]
struct Cli {
    #[arg(long, short, default_value = "")]
    message: String,
}

#[derive(Debug, Serialize)]
#[allow(dead_code)]
struct Message {
    text: String,

    #[serde(with = "ts_seconds")]
    now: DateTime<Utc>,
}


fn main() {
    let args = Cli::parse();
    //dbg!(&args);
    if args.message == "" {
        eprintln!("-m message    is required");
        exit(1);
    }

    let msg = Message {
        text: args.message,
        now: Utc::now(),
    };

    let serialized = serde_json::to_string(&msg).unwrap();
    //println!("{}", serialized);

    let filename = "messages.json";
    let mut fh = match File::options().append(true).open(filename) {
        Ok(fh) => fh,
        Err(_) => {
            File::create(filename).unwrap()
        }
    };
    writeln!(&mut fh, "{}", serialized).unwrap();

}
