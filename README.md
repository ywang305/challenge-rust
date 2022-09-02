# Challenge-Rust

---

### [hello (a tour)](./hello/src/main.rs)

- install [rustup](https://rustup.rs/)
  - it will bring up cargo, rustc,
  - `rustc --version`, `cargo --version`
  - vscode setting auto format
    ```json
    "[rust]": {
        "editor.defaultFormatter": "rust-lang.rust-analyzer"
    }
    ```
- vscode plugins

  - Rust
  - CodeLLDB ( for debug: [generated launch.json](./hello/.vscode/launch.json) )

- cargo
  - `cargo init`
  - `cargo new hello_project`
  - `cargo run`
    - fetch needed crates, compile, build, link, startup
  - `cargo clean`
  - `cargo test`
    - run all tests
  - `cargo test someTestFn`
    - just run that test
  - `cargo build --release`

### [fundamental_types](./fundamental_types/src/main.rs)

    float, bool, char,
    tuples
    arrays, vectors, slices
    string
    type aliases

---

## Walk The Dog (WASM)

### chapter_1

Initialize the project

- rust-webpack-template
  ```sh
  mkdir walk-the-dog && cd walk-the-dog
  npm init rust-webpack
  npm i
  ```
- dev
  ```sh
  npm start
  ```
- install cargo dependencies \
  add under Cargo.toml -> [dependencies]
