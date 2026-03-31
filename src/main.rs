// ============================================================
// dungeon-scribe — CLI entry point
//
// PROVIDED — do not modify this file.
//
// Usage:
//   cargo run -- <path-to-map-file>
//
// Examples:
//   cargo run -- maps/simple.txt
//   cargo run -- maps/invalid_jagged.txt
// ============================================================

use std::env;
use std::fs;
use std::process;

use dungeon_scribe::{generate_report, DungeonMap};

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        eprintln!("Usage: {} <map_file>", args[0]);
        eprintln!("Example: {} maps/simple.txt", args[0]);
        process::exit(1);
    }

    let path = &args[1];

    let contents = match fs::read_to_string(path) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("Error: could not read '{}': {}", path, e);
            process::exit(1);
        }
    };

    let map = match DungeonMap::parse(&contents) {
        Ok(m) => m,
        Err(e) => {
            eprintln!("Parse error: {}", e);
            process::exit(1);
        }
    };

    println!("{}", generate_report(&map));
}
