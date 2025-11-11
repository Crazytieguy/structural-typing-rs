See @README.md for project details

- Always add dependencies with `cargo add` to ensure getting the latest version
- Edition 2024 is in fact correct
- Always use `cargo clippy --all-targets` and `cargo test --all-features` to ensure quality when you finish a task
- Markdown documentation of crate and dependencies available in target/doc-md/, index: @target/doc-md/index.md
- To release:
  1. Bump patch version in workspace Cargo.toml (keeping 0.1.x while API is unstable)
  2. Update version in installation sections in README.md and structural-typing/src/lib.rs
  3. Commit
  4. `cargo publish -p structural-typing-macros && cargo publish -p structural-typing`
  5. Create and push git tag
- When adding to or changing the crate API surface, update explanations and examples in both README.md and structural-typing/src/lib.rs to reflect the changes
