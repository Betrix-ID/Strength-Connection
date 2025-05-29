mod nedt;
mod help;

use nedt::*;
use std::{env, process::exit, thread::sleep, time::Duration};
use help::usage;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        usage();
        exit(1);
    }

    match args[1].as_str() {
        "-S" => {           
            cout_5();
            println!("- Apply use latency 5ns priority");
        }
        "-U" => {            
            cout_30();
            println!("- Apply use latency 30ns priority");
        }
        "-L" => {           
            cout_50();
            println!("- Apply use latency 50ms priority");
        }
         "-d" => {
            deafult();
            println!("- Apply settings latency default kode");
        }
        "-h" | "--help" => usage(),
        other => {
            println!("Unknown option: {}", other);
            exit(1);
        }
    }

    sleep(Duration::from_secs(1));
    println!("\n⚠️ This module is protected by copyright and is\n\
              intended for use by regular users only. Any use of\n\
              this module, including its code, design, or features,\n\
              by other developers without written permission from\n\
              the copyright owner is strictly prohibited.\n\
              ________________________________________________(+)\n");
}