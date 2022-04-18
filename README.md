# onitama-rs
Onitama game in Bevy engine for university project in AI subject


# WASM

To build the WASM version run the following commands:
```
cargo build --target wasm32-unknown-unknown --release
wasm-bindgen --out-dir ./web/ --target web .\target\wasm32-unknown-unknown\release\onitama-rs.wasm
```

# References
- Sébastien Belzile. [Making Games in Rust](https://dev.to/sbelzile/rust-platformer-part-1-bevy-and-ecs-2pci)
- Félix de Maneville(Qongzi). [Bevy Minesweeper](https://dev.to/qongzi/bevy-minesweeper-introduction-4l7f)
- [Chessprogramming](https://www.chessprogramming.org/)
- maxbennedich. [onitama](https://github.com/maxbennedich/onitama)

## Fonts

- [Orange Kid](https://www.1001fonts.com/orange-kid-font.html)
- [Pixeled]()
