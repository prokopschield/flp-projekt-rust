# Dungeon Map Scribe

A Rust library that parses, validates, and analyses ASCII roguelike dungeon maps.

## Quick start

```bash
# Build the project (all todo!() stubs must at least compile)
cargo build

# Run against a sample map
cargo run -- maps/simple.txt

# Run all tests
cargo test

# Check formatting (must be clean before submission)
cargo fmt --check

# Check for lints (must be clean before submission)
cargo clippy -- -D warnings
```

## Project structure

```
dungeon-scribe/
├── Cargo.toml
├── NOTES.md            ← fill this in before submission
├── maps/
│   ├── simple.txt          valid map — basic happy path
│   ├── complex.txt         valid map — all 8 tile types present
│   ├── disconnected.txt    valid map — two isolated floor regions
│   ├── invalid_no_player.txt   parses OK, fails validation
│   └── invalid_jagged.txt      fails to parse (JaggedMap error)
└── src/
    ├── main.rs         PROVIDED — CLI wrapper, do not modify
    ├── lib.rs          PROVIDED — module re-exports, do not modify
    ├── error.rs        PROVIDED — ParseError / ValidationError, do not modify
    ├── tile.rs         IMPLEMENT — Tile::from_char, Tile::is_passable
    ├── model.rs        IMPLEMENT — DungeonMap::parse and all methods
    ├── analysis.rs     IMPLEMENT — reachable_floor_size (flood fill)
    └── report.rs       IMPLEMENT — generate_report
```

## What you need to implement

| File | Function(s) |
|------|-------------|
| `src/tile.rs` | `Tile::from_char`, `Tile::is_passable` |
| `src/model.rs` | `DungeonMap::parse`, `get`, `rows`, `count_tiles`, `find_all`, `validate` |
| `src/analysis.rs` | `reachable_floor_size` |
| `src/report.rs` | `generate_report` |

Every function you need to fill in has a `todo!()` body and a doc comment
explaining what it must do.

## Rules

- No `unwrap()` or `expect()` in library code (test code is fine).
- No external crates — `std` only.
- No `unsafe` code.
- Do not modify `main.rs`, `lib.rs`, or the error variant declarations in `error.rs`.
- Do not change any public function signature.

## Tile reference

| Char | Tile variant  | Passable? |
|------|---------------|-----------|
| `#`  | `Wall`        | No        |
| `.`  | `Floor`       | Yes       |
| `@`  | `PlayerStart` | Yes       |
| `E`  | `Enemy`       | Yes       |
| `T`  | `Treasure`    | Yes       |
| `X`  | `Exit`        | Yes       |
| `+`  | `Door`        | Yes       |
| `^`  | `Trap`        | Yes       |
