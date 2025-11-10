See @README.md for project details

- Always add dependencies with `cargo add` to ensure getting the latest version
- Edition 2024 is in fact correct
- Always use `cargo clippy --all-targets` and `cargo test --all-features` to ensure quality when you finish a task
- Markdown documentation of crate and dependencies available in target/doc-md/, index: @target/doc-md/index.md
- To release: bump patch version in workspace Cargo.toml (keeping 0.1.x while API is unstable), commit, `cargo publish -p structural-typing-macros && cargo publish -p structural-typing`, create and push git tag
