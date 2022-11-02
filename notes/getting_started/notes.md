# Notes

<strong>Using https://rust-book.cs.brown.edu/ch01-03-hello-cargo.html</strong>

1. Cargo normally generates a git repository with <code> cargo new "name"</code>
2. Because this cargo project was generated inside of a git repository that already exists, the git files were never generated. This can be overriden with <code> cargo new "name" --vcs=git</code>
3. Projects not started with <code>cargo new</code> can be converted into cargo projects by placing all code into a directory called <code>src</code> and create an appropriate <code>Cargo.toml</code> configuration file in the root directory of the project (level above src).
4. To build a rust/cargo project, use <code>cargo build</code> while in the main dir, ie: hello_cargo. This creates an executable in <i>target/debug/hello_cargo</i>.
5. The default build type for cargo is debug, hence the debug folder path. We can use `cargo build --release ` for building a non-debug version. 
6. The standard way of building then running code is to just use `cargo run` or `cargo run --release`.
7. To check if your code will compile, it's standard to periodically run `cargo check`. If there is an error in your code, it'll print the debug info on it. 
8. Cargo commands are OS-agnostic. 