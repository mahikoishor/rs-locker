#![allow(dead_code)]
#![allow(unused)]

pub mod constances;
pub mod entry;
pub mod error;
pub mod locker;
pub mod utils;

use std::env;
use std::process::ExitCode;

use error::Result;
use locker::Locker;

fn main() -> ExitCode {
    let args = env::args().collect::<Vec<String>>();

    if args.len() == 1 {
        eprintln!("Need more args.");
        return ExitCode::FAILURE;
    }

    match &args[1][..] {
        "help" | "-h" => help(),
        "create" => {
            if args.len() < 4 {
                println!("ERROR: `create` needs more args.");
                return ExitCode::FAILURE;
            }

            if let Err(err) = Locker::create_archive(&args[2], &args[3]) {
                eprintln!("{err:?}");

                return ExitCode::FAILURE;
            }
        }
        "read" => {
            if args.len() < 4 {
                println!("ERROR: `read` needs more args.");
                return ExitCode::FAILURE;
            }

            if let Err(err) = Locker::open_archive(&args[2], &args[3]) {
                eprintln!("{err:?}");

                return ExitCode::FAILURE;
            }
        }
        _ => help(),
    }

    ExitCode::SUCCESS
}

pub fn help() {
    println!(
        "
        Usage:
  
        help                                  : List of all commands.
        create <SRC> <PASSWORD>               : Create a lock file.  
        read <SRC> <PASSWORD>                 : Read a lock file.  
    "
    );
}
