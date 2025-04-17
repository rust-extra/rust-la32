# Rust LoongArch32 nano kernel demo

## Dependencies

- Rust toolchain with `loongarch32-unknown-none*` targets
- Qemu system emulator with loongarch64(32)

## How to run

### loongarch32-unknown-none

```sh
cargo run -r --target loongarch32-unknown-none
```

### loongarch32-unknown-none-softfloat

```sh
cargo run -r --target loongarch32-unknown-none-softfloat
```

## License

MIT
