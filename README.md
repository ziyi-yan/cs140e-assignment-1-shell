# CS140e assignment 0: blinky

Solution of the assignent 0 of [CS140e](https://cs140e.sergio.bz/syllabus/).

## Environment

Hardware: Raspberry Pi 3B+

Rust toolchain:

```
$ rustc --version
rustc 1.43.0-nightly (96bb8b31c 2020-03-05)
$ xargo --version
xargo 0.3.19
cargo 1.43.0-nightly (bda50510d 2020-03-02)
```

## Problems about breaking changes in Rust

1. `-Z no-trans` is renamed to `-Z no-codegen` (https://github.com/rust-lang/rust/pull/50615)

## Takeways

1. If a struct implements trait `Copy`, it should implement `Clone`.
2. Use `ref` in pattern to take a reference from values.
3. If a struct calls a method which is implemented for a trait, the trait need to be imported in this scope (`use` it).
4. `Into<T>` is a useful trait bound for input parameters.
