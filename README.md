# Game of Life

A small project implementing [Conway's Game of Life](https://en.wikipedia.org/wiki/Conway's_Game_of_Life).
Uses WASM, with Rust's [Yew](https://yew.rs) library.

## Running

You need cargo installed on your system. Then, using cargo, you need to install trunk.

```
cargo install trunk
```

Then clone the repository and serve it using trunk.

```shell
git clone https://github.com/pjlapinski/wasm_game_of_life
cd wasm_game_of_life
trunk serve
```

You should be able to see the application in your browser under http://localhost:8080
