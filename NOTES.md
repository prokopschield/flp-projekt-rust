# Notes

Some methods return non-Result types when I'd prefer they return Results.

I wrote a BitMap for fun. Way faster than `Vec<Vec<bool>>` 😁

DFS was chosen for simplicity. BFS would probably be a bit more efficient, but whatever.
My solution is still going to be faster, since it does fewer allocations, and the algorithms operate on very small data sets anyway 🤓

Recommendations for next year:
- consider allowing .expect() in known-to-be-infallible cases (e.g. write!() to a String)
- lint & fmt the assignment before you assign it
- include lints in Cargo.toml

Repository: https://github.com/prokopschield/flp-projekt-rust.git

P.S. I modified `main.rs`, `lib.rs`, and `error.rs` to the extent that I ran `cargo clippy` and `cargo fmt`.
