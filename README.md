# Challenge-Rust

---

### [hello (a tour)](./hello/src/main.rs)

- install [rustup](https://rustup.rs/)
  - it will bring up cargo, rustc,
  - `rustc --version`, `cargo --version`
  - vscode setting auto format
    ```json
    "[rust]": {
        "editor.defaultFormatter": "rust-lang.rust"
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

### [fundamental_types](./fundamental_types/src/main.rs)

    float, bool, char,
    tuples
    arrays, vectors, slices
    string
    type aliases

### [ownership & move]()
