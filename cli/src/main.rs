use clap::{Parser, Subcommand};

mod invoke;
use invoke::Invoke;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Root {
    #[clap(subcommand)]
    cmd: Cmd,
}

#[derive(Subcommand, Debug)]
enum Cmd {
    Invoke(Invoke),
}

fn main() {
    let root = Root::parse();
    let res = match root.cmd {
        Cmd::Invoke(args) => args.run(),
    };
    match res {
        Ok(_) => println!("ok"),
        Err(e) => println!("error: {}", e),
    }
}
