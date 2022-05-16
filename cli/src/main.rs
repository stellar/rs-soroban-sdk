use clap::{Parser, Subcommand};
use std::fs;
use stellar_contract_env_host::{
    xdr::{ScVal, ScVec},
    Host, VM,
};

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Root {
    #[clap(subcommand)]
    cmd: Cmd,
}

#[derive(Subcommand, Debug)]
enum Cmd {
    Invoke(InvokeArgs),
}

#[derive(Parser, Debug)]
struct InvokeArgs {
    #[clap(long, parse(from_os_str))]
    file: std::path::PathBuf,
    #[clap(long)]
    func: String,
}

impl InvokeArgs {
    pub fn call(&self) {
        let contents = fs::read(&self.file).unwrap();
        let mut h = Host::default();
        let vm = VM::new(&h, &contents).unwrap();
        let params = Vec::<ScVal>::new();
        let res = vm
            .invoke_function(&mut h, &self.func, &ScVec(params.try_into().unwrap()))
            .unwrap();
        println!("{:?}", res);
    }
}

fn main() {
    let root = Root::parse();
    match root.cmd {
        Cmd::Invoke(args) => args.call(),
    }
}
