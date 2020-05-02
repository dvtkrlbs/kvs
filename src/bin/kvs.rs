use structopt::StructOpt;
use std::process;

#[derive(StructOpt, Debug)]
#[structopt(name = "kvs", about = "Key value store")]
enum Kvs {
    Get {
        key: String,
    },
    Set {
        key: String,
        value: String,
    },
    Rm {
        key: String,
    }
}

fn main() {
    match Kvs::from_args() {
        Kvs::Get { key: _ } => {
            eprintln!("unimplemented");
            process::exit(1);
        },
        Kvs::Set { key: _, value: _ } => {
            eprintln!("unimplemented");
            process::exit(1);
        },
        Kvs::Rm { key: _ } => {
            eprintln!("unimplemented");
            process::exit(1);
        }
    }
}
