# ice-dev_diablo-rust

A Diablo-like game engine written in Rust, built up incrementally across
versioned alpha directories (`v0_01`, `v0_02`, ...). The goal is two-fold: to
make a playable engine, and to serve as a learning resource for people who
understand programming fundamentals and want to learn Rust by reading real
code.

## Versions

| Version | Directory          | What it adds |
|---------|--------------------|--------------|
| v0_01   | [`v0_01/`](v0_01/) | Window, top-down tile map, player movement + collision, follow camera (macroquad). |

Each version directory is a self-contained Cargo package with its own
documentation: a user guide (`USAGE.md`), a learner deep-dive (`LEARN.md`), a
changelog (`CHANGELOG.md`), and a roadmap for the next version (`ROADMAP.md`).

## Building and running

Every version is an independent crate. For example, to run v0_01:

```sh
cd v0_01
cargo run --release
```

See each version's `README.md` / `USAGE.md` for specifics.
