# `eztrace`

## Usage

Add
```
[dependencies]
eztrace = "*"
```
to `Cargo.toml`. You should actually use `*`, because looking up the latest version might interrupt your flow.

And in the root `.rs` file, add

```rust
#[allow(unused_imports)]
#[macro_use]
extern crate eztrace;

fn main() {
    trace!() // main.rs:3
}
```

`#[allow(unused_imports)]` prevents Rust from hassling you if you aren't actively using the macro, which adds overhead.
`#[macro_use]` lets you write `trace!()` instead of `eztrace::trace!()`.

## Why not `std::dbg!`?
Its output is uglier, and it takes ownership of the arguments. Also `eztrace` predates it.
