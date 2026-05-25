# Third-Party Licenses

This project depends on third-party open-source packages from the JavaScript and Rust ecosystems.

- JavaScript dependency licenses are defined in each package under `node_modules` after install.
- Rust dependency licenses are defined in `Cargo.lock` and crate metadata.

When creating releases, include generated dependency notices if required by your distribution channel.

Recommended commands for auditing licenses:

```powershell
npm ls --all
cargo tree --locked
```

For strict license reporting, you can add dedicated tools such as:

- `license-checker` (Node.js)
- `cargo-about` or `cargo-license` (Rust)
