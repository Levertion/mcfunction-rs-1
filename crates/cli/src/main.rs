#![deny(unsafe_code)]
use clap::arg_enum;
use structopt::StructOpt;

arg_enum! {
    #[derive(Debug)]
    enum ErrorFormat {
        Json,
        Text,
    }
}

#[derive(StructOpt)]
#[structopt()]
struct Opt {
    #[structopt(possible_values = &ErrorFormat::variants(), case_insensitive = true)]
    format: ErrorFormat,
}

fn main() {
    println!("Hello, world!");
    let mut test = 1;
}
